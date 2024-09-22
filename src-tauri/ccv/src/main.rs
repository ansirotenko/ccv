// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;

use ccv::about;
use ccv::settings;
use ccv::primary;
use ccv::utils::window::{close_window, show_window};
use ccv::splashscreen;
use ccv::tray;
use ccv_contract::{error::log_error, models::Settings};
use ccv_contract::models::CopyCategory::Unknown;
use cfg_if::cfg_if;
use tauri::{
    async_runtime, generate_context, generate_handler, Builder, Manager,
    WindowEvent::CloseRequested,
};
use tauri_plugin_clipboard::ClipboardManager;

fn main() {
    let builder = Builder::default()
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, None))
        .plugin(tauri_plugin_clipboard::init())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([tauri_plugin_log::LogTarget::LogDir])
                .level(log::LevelFilter::Warn)
                .level_for("tao::platform_impl::platform::event_loop::runner", log::LevelFilter::Error)
                .build(),
        )
        .plugin(tauri_plugin_single_instance::init(|_,_,_| {}))
        .setup(|app| {
            let app_handle = app.app_handle();
            if let Some(app_data_dir) = app_handle.path_resolver().app_data_dir() {
                if !app_data_dir.exists() {
                    std::fs::create_dir_all(app_data_dir.clone()).unwrap();
                }
                let state_settings = app.state::<settings::state::SettingsState>();
                let mut settings = state_settings.settings.lock().unwrap();
                match settings::state::SettingsState::read_settings(&app_data_dir) {
                    Ok(new_settings) => {
                        *settings = Some(new_settings);
                    },
                    Err(err) => {
                        log::error!("Unable to read settings file. {err}");
                    }
                }

                let state_clipboard = app.state::<ClipboardManager>();
                let state = app.state::<primary::state::CopyItemState>();
                let mut repository = state.repository.lock().unwrap();
                cfg_if! {
                    if #[cfg(feature = "sqlite")] {
                        use ccv_sqlite::repository::SqliteRepository;
                        match SqliteRepository::new(app_data_dir.join("ccv.db").to_str().unwrap()) {
                            Ok(sqlite_repo) => {
                                *repository = Box::new(sqlite_repo);
                            },
                            Err(err) => {
                                log::error!("Failed to connect to sqlite database. {err}");
                            }
                        };
                    } else {
                        use ccv_in_memory::{repository::InMemoryRepository, sample_data::every_type};
                        if #[cfg(feature = "in-memory-test-data")] {
                            let data = every_type();
                        } else {
                            let data = Vec::new();
                        }
                        *repository = Box::new(InMemoryRepository::new(data));
                    }
                }

                let category = primary::commands::get_clipboard_category(&state_clipboard);
                match category {
                    Err(e) => {
                        log::warn!("Failed to identify clipboard value: {e}")
                    }
                    Ok(Unknown) => {}
                    _ => {
                        if let Err(err) = primary::commands::insert_copy_item_if_not_found(repository.as_ref(), state_clipboard) {
                            log::error!("Failed to insert clipboard state on start. {err}");
                        }
                    }
                };

                // #[cfg(debug_assertions)] // only include this code on debug builds
                // {
                //     primary_window.open_devtools();
                // }
                async_runtime::spawn(async move {
                    thread::sleep(std::time::Duration::from_millis(500));
                    if let Err(err) = close_window(&app_handle.get_window(splashscreen::SCREEN)) {
                        log::error!("Unable to hide splashscreen window. {err}") ;
                    }
                    
                    if let Err(err) = show_window(&app_handle.get_window(primary::SCREEN)) {
                        log::error!("Unable to show primary window. {err}");
                    }
                });

                #[cfg(not(target_os = "linux"))]
                {
                    let settings = settings.clone().unwrap();
                    if let Err(err) = settings::commands::register_keybindings(&app.app_handle(), &settings) {
                        log::error!("Unable to register initial shortcuts. {err}");
                    }
                }
                
                #[cfg(target_os = "linux")]
                {
                    use std::sync::mpsc::channel;
                    use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};

                    let primary_window = app.get_window(primary::SCREEN);
                    let (tx, rx) = channel::<Settings>();
                    let mut hotkey_change = state_settings.hotkey_change.lock().unwrap();
                    *hotkey_change = Some(tx);
                    let settings = settings.clone().unwrap();
                    async_runtime::spawn(async move {
                        let manager = GlobalHotKeyManager::new().unwrap();
                        let mut hotkey = settings::commands::get_hotkey(&settings.keybindings.open_ccv).unwrap();
                        manager.register(hotkey).unwrap();
                        loop {
                            if let Ok(_) = GlobalHotKeyEvent::receiver().try_recv() {
                                if let Err(err) = show_window(&primary_window) {
                                    log::error!("Unable to show primary window. {err}");
                                }
                            }

                            if let Ok(settings) = rx.try_recv() {
                                manager.unregister(hotkey).unwrap();
                                hotkey = settings::commands::get_hotkey(&settings.keybindings.open_ccv).unwrap();
                                manager.register(hotkey).unwrap();
                            }
                            
                            // TODO
                            // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                        }
                    });
                }
            } else {
                log::error!("Unable to get path to application data");
            }

            Ok(())
        })
        .manage(primary::state::CopyItemState::new_uninitialized())
        .manage(settings::state::SettingsState::new())
        .system_tray(tray::get_menu())
        .on_window_event(|event| match event.event() {
            CloseRequested { api, .. } => {
                log_error(event.window().hide(), "Unable to hide primary window").unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .on_system_tray_event(tray::event_handler)
        .invoke_handler(generate_handler![
            primary::commands::search_copy_items,
            primary::commands::reuse_copy_item,
            primary::commands::insert_copy_item,
            primary::commands::hide_primary_window,
            primary::commands::show_primary_window,
            about::commands::get_about_data,
            about::commands::open,
            about::commands::hide_about_window,
            about::commands::show_about_window,
            settings::commands::hide_settings_window,
            settings::commands::show_settings_window,
            settings::commands::get_settings,
            settings::commands::set_settings,
            settings::commands::remove_copy_items,
            settings::commands::remove_copy_items_older,
        ]);

    log_error(
        builder.run(generate_context!()),
        "Failed to start application",
    )
    .unwrap();
}
