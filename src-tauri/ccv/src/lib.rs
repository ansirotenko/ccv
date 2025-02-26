pub mod about;
pub mod primary;
pub mod settings;
pub mod splashscreen;
pub mod tray;
pub mod utils;

use std::thread;
use tauri::{async_runtime, generate_context, generate_handler, Builder, Manager, WindowEvent};
use utils::{
    get_app_data_dir,
    window::{close_window, hide_window, show_window},
};

pub fn run() -> () {
    let builder = Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init()) // required for restart after updater completes job
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        )) // test autostart
        .plugin(tauri_plugin_clipboard::init())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("ccv".to_string()),
                    },
                )])
                .level(log::LevelFilter::Warn)
                .level_for(
                    "tao::platform_impl::platform::event_loop::runner",
                    log::LevelFilter::Error,
                )
                .build(),
        )
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .setup(|app| {
            let app_handle = app.app_handle().clone();
            if let Err(err) = tray::setup::build(&app_handle) {
                log::error!("Failed to setup tray. {err}");
                return Err(Box::new(err));
            }

            match get_app_data_dir(&app_handle) {
                Ok(app_data_dir) => {
                    if !app_data_dir.exists() {
                        if let Err(err) = std::fs::create_dir_all(app_data_dir.clone()) {
                            log::error!("Unable to create application directory. {err}");
                            return Err(Box::new(err));
                        }
                    }
                    if let Err(err) = primary::setup::init_repository(app.app_handle()) {
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

                    let splash_screen_window = app_handle.get_webview_window(splashscreen::SCREEN);
                    async_runtime::spawn(async move {
                        thread::sleep(std::time::Duration::from_millis(500));
                        if let Err(err) = close_window(&splash_screen_window) {
                            log::error!("Unable to hide splashscreen window. {err}");
                        }

                        let settings_state = app_handle.state::<settings::state::SettingsState>();
                        let settings = settings_state.settings.lock().unwrap().clone();
                        if let Some(settings) = settings {
                            if let Some(notifications) = settings.notifications {
                                if !notifications.is_empty() {
                                    if let Err(err) =
                                        show_window(&app_handle.get_webview_window(primary::SCREEN))
                                    {
                                        log::error!("Unable to show primary window. {err}");
                                    }
                                }
                            }
                        }
                    });

                    Ok(())
                }
                Err(err) => {
                    log::error!("Unable to get path to application data");

                    Err(Box::new(err))
                }
            }
        })
        .manage(primary::state::PrimaryState::new_uninitialized())
        .manage(settings::state::SettingsState::new())
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                match hide_window(&window.get_webview_window(window.label())) {
                    Ok(_) => {
                        api.prevent_close();
                    }
                    Err(err) => {
                        log::error!("Unable to hide window. {err}");
                    }
                }
            }
            #[cfg(not(debug_assertions))]
            WindowEvent::Focused(is_focused) => {
                use ccv_contract::{app_error, error::AppError};

                if !is_focused && window.label() == primary::SCREEN {
                    match window.is_visible() {
                        Ok(is_visible) => {
                            if is_visible {
                                // sometimes window looses focus and then immideately gets it back
                                // e.g. on dragging window or focusing search box (only at linux)

                                if let Some(window) =
                                    window.app_handle().get_webview_window(window.label())
                                {
                                    let async_perfomer = move || -> Result<(), AppError> {
                                        std::thread::sleep(std::time::Duration::from_millis(50));

                                        let still_visible = window.is_visible().map_err(|err| {
                                            app_error!("Cannot get visible {err}")
                                        })?;
                                        let still_focsed = window.is_focused().map_err(|err| {
                                            app_error!("Cannot get focused {err}")
                                        })?;
                                        if still_visible && !still_focsed {
                                            hide_window(&Some(window))?;
                                        }

                                        Ok(())
                                    };

                                    std::thread::spawn(async_perfomer);
                                }
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
        .invoke_handler(generate_handler![
            primary::commands::search_copy_items,
            primary::commands::reuse_copy_item,
            primary::commands::insert_copy_item,
            primary::commands::hide_primary_window,
            primary::commands::show_primary_window,
            about::commands::get_about_data,
            about::commands::open_path,
            about::commands::open_url,
            about::commands::hide_about_window,
            about::commands::show_about_window,
            settings::commands::hide_settings_window,
            settings::commands::show_settings_window,
            settings::commands::get_settings,
            settings::commands::set_settings,
            settings::commands::remove_copy_items,
            settings::commands::remove_copy_items_older,
        ]);

    if let Err(err) = builder.run(generate_context!()) {
        log::error!("Failed to start application. {err}")
    }
}
