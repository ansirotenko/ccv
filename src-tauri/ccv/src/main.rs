// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;

use ccv::about;
use ccv::primary;
use ccv::settings;
use ccv::settings::state::SettingsState;
use ccv::splashscreen;
use ccv::tray;
use ccv::utils::window::{close_window, hide_window, show_window};
use ccv_contract::{
    app_error,
    error::{log_error, AppError},
};
use tauri::{
    async_runtime, generate_context, generate_handler, Builder, Manager,
    WindowEvent::{CloseRequested, Focused},
};

fn main() {
    let builder = Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_clipboard::init())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([tauri_plugin_log::LogTarget::LogDir])
                .level(log::LevelFilter::Warn)
                .level_for(
                    "tao::platform_impl::platform::event_loop::runner",
                    log::LevelFilter::Error,
                )
                .build(),
        )
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .setup(|app| {
            let app_handle = app.app_handle();
            if let Some(app_data_dir) = app_handle.path_resolver().app_data_dir() {
                if !app_data_dir.exists() {
                    if let Err(err) = std::fs::create_dir_all(app_data_dir.clone()) {
                        log::error!("Unable to create application directory. {err}");
                        return Err(Box::new(err));
                    }
                }
                if let Err(err) = primary::setup::init_repository(app.app_handle(), &app_data_dir) {
                    log::error!("Unable to initialize repository. {err}");
                    return Err(Box::new(err));
                }

                if let Err(err) = settings::setup::read_settings_and_register_shortcuts(
                    app.app_handle(),
                    &app_data_dir,
                ) {
                    log::error!("Unable to initialize settings. {err}");
                    return Err(Box::new(err));
                }

                // #[cfg(debug_assertions)] // only include this code on debug builds
                // {
                //     app_handle.get_window(primary::SCREEN).unwrap().open_devtools();
                // }

                async_runtime::spawn(async move {
                    thread::sleep(std::time::Duration::from_millis(500));
                    if let Err(err) = close_window(&app_handle.get_window(splashscreen::SCREEN)) {
                        log::error!("Unable to hide splashscreen window. {err}");
                    }

                    let settings_state = app_handle.state::<SettingsState>();
                    let settings = settings_state.settings.lock().unwrap().clone();
                    if let Some(settings) = settings {
                        if let Some(notifications) = settings.notifications {
                            if !notifications.is_empty() {
                                if let Err(err) =
                                    show_window(&app_handle.get_window(primary::SCREEN))
                                {
                                    log::error!("Unable to show primary window. {err}");
                                }
                            }
                        }
                    }
                });
            } else {
                log::error!("Unable to get path to application data");
            }

            Ok(())
        })
        .manage(primary::state::PrimaryState::new_uninitialized())
        .manage(settings::state::SettingsState::new())
        .system_tray(tray::get_menu())
        .on_window_event(|event| match event.event() {
            CloseRequested { api, .. } => match hide_window(&Some(event.window().clone())) {
                Ok(_) => {
                    api.prevent_close();
                }
                Err(err) => {
                    log::error!("Unable to hide window. {err}");
                }
            },
            Focused(is_focused) => {
                println!("{is_focused}");
                if !is_focused && event.window().label() == primary::SCREEN {
                    match event.window().is_visible() {
                        Ok(is_visible) => {
                            if is_visible {
                                // sometimes window looses focus and then immideately gets it back
                                // e.g. on dragging window or focusing search box (only at linux)
                                let async_perfomer = move || -> Result<(), AppError> {
                                    std::thread::sleep(std::time::Duration::from_millis(50));

                                    let still_visible = event
                                        .window()
                                        .is_visible()
                                        .map_err(|err| app_error!("Cannot get visible {err}"))?;
                                    let still_focsed = event
                                        .window()
                                        .is_focused()
                                        .map_err(|err| app_error!("Cannot get focused {err}"))?;
                                    if still_visible && !still_focsed {
                                        hide_window(&Some(event.window().clone()))?;
                                    }

                                    Ok(())
                                };

                                std::thread::spawn(async_perfomer);
                            }
                        }
                        Err(err) => {
                            log::error!("Unable to get window visibility. {err}")
                        }
                    }
                }
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
            about::commands::open_uri,
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
