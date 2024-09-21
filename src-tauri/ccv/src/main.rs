// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod events;
mod screens;
mod state;

use std::thread;

use ccv::utils::window::{close_window, show_window};
use ccv::splashscreen;
use ccv::tray::{get_tray_menu, tray_event_handler};
use ccv_contract::{error::log_error, models::Settings};
use ccv_contract::models::CopyCategory::Unknown;
use cfg_if::cfg_if;
use commands::{
    main::{
        get_clipboard_category, hide_main_window, insert_copy_item, insert_copy_item_if_not_found,
        reuse_copy_item, search_copy_items, show_main_window,
    },
    settings::{
        get_settings, hide_settings_window, remove_copy_items, remove_copy_items_older,
        set_settings, show_settings_window, register_keybindings, get_hotkey
    },
};
use screens::MAIN;
use state::{CopyItemState, SettingsState};
use tauri::{
    async_runtime, generate_context, generate_handler, Builder, Manager,
    WindowEvent::CloseRequested,
};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_clipboard::ClipboardManager;

fn main() {
    let builder = Builder::default()
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
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
                let state_settings = app.state::<SettingsState>();
                let mut settings = state_settings.settings.lock().unwrap();
                match SettingsState::read_settings(&app_data_dir) {
                    Ok(new_settings) => {
                        *settings = Some(new_settings);
                    },
                    Err(err) => {
                        log::error!("Unable to read settings file. {err}");
                    }
                }

                let state_clipboard = app.state::<ClipboardManager>();
                let state = app.state::<CopyItemState>();
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

                let category = get_clipboard_category(&state_clipboard);
                match category {
                    Err(e) => {
                        log::warn!("Failed to identify clipboard value: {e}")
                    }
                    Ok(Unknown) => {}
                    _ => {
                        if let Err(err) = insert_copy_item_if_not_found(repository.as_ref(), state_clipboard) {
                            log::error!("Failed to insert clipboard state on start. {err}");
                        }
                    }
                };

                // #[cfg(debug_assertions)] // only include this code on debug builds
                // {
                //     main_window.open_devtools();
                // }
                async_runtime::spawn(async move {
                    thread::sleep(std::time::Duration::from_millis(500));
                    if let Err(err) = close_window(&app_handle.get_window(splashscreen::SCREEN)) {
                        log::error!("Unable to hide splashscreen window. {err}") ;
                    }
                    
                    if let Err(err) = show_window(&app_handle.get_window(MAIN)) {
                        log::error!("Unable to show main window. {err}");
                    }
                });

                #[cfg(not(target_os = "linux"))]
                {
                    let settings = settings.clone().unwrap();
                    if let Err(err) = register_keybindings(&app.app_handle(), &settings) {
                        log::error!("Unable to register initial shortcuts. {err}");
                    }
                }
                
                #[cfg(target_os = "linux")]
                {
                    use std::sync::mpsc::channel;
                    use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};

                    let main_window = app.get_window(MAIN).unwrap();
                    let (tx, rx) = channel::<Settings>();
                    let mut hotkey_change = state_settings.hotkey_change.lock().unwrap();
                    *hotkey_change = Some(tx);
                    let settings = settings.clone().unwrap();
                    async_runtime::spawn(async move {
                        let manager = GlobalHotKeyManager::new().unwrap();
                        let mut hotkey = get_hotkey(&settings.keybindings.open_ccv).unwrap();
                        manager.register(hotkey).unwrap();
                        loop {
                            if let Ok(_) = GlobalHotKeyEvent::receiver().try_recv() {
                                if let Err(err) = show_window(&main_window) {
                                    log::error!("Unable to show main window. {err}");
                                }
                            }

                            if let Ok(settings) = rx.try_recv() {
                                manager.unregister(hotkey).unwrap();
                                hotkey = get_hotkey(&settings.keybindings.open_ccv).unwrap();
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
        .manage(CopyItemState::new_uninitialized())
        .manage(SettingsState::new())
        .system_tray(get_tray_menu())
        .on_window_event(|event| match event.event() {
            CloseRequested { api, .. } => {
                log_error(event.window().hide(), "Unable to hide main window").unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .on_system_tray_event(tray_event_handler)
        .invoke_handler(generate_handler![
            search_copy_items,
            reuse_copy_item,
            insert_copy_item,
            remove_copy_items,
            remove_copy_items_older,
            hide_main_window,
            show_main_window,
            ccv::about::commands::get_about_data,
            ccv::about::commands::open,
            ccv::about::commands::hide_about_window,
            ccv::about::commands::show_about_window,
            hide_settings_window,
            show_settings_window,
            get_settings,
            set_settings
        ]);

    log_error(
        builder.run(generate_context!()),
        "Failed to start main application",
    )
    .unwrap();
}
