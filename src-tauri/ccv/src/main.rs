// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod events;
mod screens;
mod shortcut;
mod state;
mod tray;

use std::thread;

use ccv_contract::error::log_error;
use ccv_contract::models::CopyCategory::Unknown;
use cfg_if::cfg_if;
use commands::{
    about::{get_about_data, hide_about_window, open, show_about_window},
    main::{
        get_clipboard_category, hide_main_window, insert_copy_item, insert_copy_item_if_not_found,
        reuse_copy_item, search_copy_items, show_main_window,
    },
    settings::{
        get_settings, hide_settings_window, remove_copy_items, remove_copy_items_older,
        set_settings, show_settings_window,
    }, utils::show_window,
};
use screens::{MAIN, SPLASHSCREEN};
use shortcut::listen_shortcut;
use state::{CopyItemState, SettingsState};
use tauri::{
    async_runtime, generate_context, generate_handler, Builder, Manager,
    WindowEvent::CloseRequested,
};
use tauri_plugin_clipboard::ClipboardManager;
use tray::{get_tray_menu, tray_event_handler};

fn main() {
    let builder = Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([tauri_plugin_log::LogTarget::LogDir])
                .level(log::LevelFilter::Warn)
                .level_for("tao::platform_impl::platform::event_loop::runner", log::LevelFilter::Error)
                .build(),
        )
        .plugin(tauri_plugin_clipboard::init())
        .setup(|app| {
            if let Some(app_data_dir) = app.app_handle().path_resolver().app_data_dir() {
                if let Some(splashscreen_window) = app.get_window(SPLASHSCREEN) {
                    if let Some(main_window) = app.get_window(MAIN) {
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
                            thread::sleep(std::time::Duration::from_millis(200));
                            if let Err(err) = splashscreen_window.close() {
                                log::error!("Unable to hide splashscreen window. {err}");
                            }
                            if let Err(err) = show_window(&main_window) {
                                log::error!("Unable to show main window. {err}");
                            }
                            
                            // this will block
                            if let Err(_) = listen_shortcut(main_window) {
                                log::error!("Unable to listen shortcuts");
                            }
                        });
                    } else {
                        log::error!("Unable get main window");
                    }
                } else {
                    log::error!("Unable get splashcreen window");
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
            get_about_data,
            open,
            hide_about_window,
            show_about_window,
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