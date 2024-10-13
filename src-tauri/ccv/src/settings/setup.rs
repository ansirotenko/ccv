use crate::primary;
use crate::settings;
use crate::utils::window::show_window;
use ccv_contract::{
    app_error,
    error::AppError,
    models::{EventPayload, MainShortcutPressedPayload},
};
use std::path::PathBuf;
use tauri::Emitter;
use tauri::{AppHandle, Manager, WebviewWindow};
use tauri_plugin_global_shortcut::ShortcutState;

pub fn read_settings_and_register_shortcuts(
    app_handle: &AppHandle,
    app_data_dir: &PathBuf,
) -> Result<(), AppError> {
    let settings_state = app_handle.state::<settings::state::SettingsState>();
    let mut settings = settings_state.settings.lock().unwrap();
    match settings::core::read_settings(app_data_dir) {
        Ok(initial_settings) => {
            *settings = Some(initial_settings.clone());

            app_handle
                .plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |app_handle, shortcut, event| match event.state() {
                            ShortcutState::Pressed => {
                                let settings_state =
                                    app_handle.state::<settings::state::SettingsState>();
                                let settings = settings_state.settings.lock().unwrap();

                                if let Some(settings) = settings.as_ref() {
                                    if let Some(open_ccv_shortcut) =
                                        settings::shortcut::get_shortcut(
                                            &settings.all_shortcuts.open_ccv,
                                        )
                                    {
                                        if shortcut == &open_ccv_shortcut {
                                            activate_primary_window(
                                                &app_handle.get_webview_window(primary::SCREEN),
                                            )
                                        }
                                    }
                                }
                            }
                            ShortcutState::Released => {}
                        })
                        .build(),
                )
                .map_err(|err| app_error!("Unable to initialize global shortcut plugin. {err}"))?;

            settings::shortcut::register_shortcuts(
                app_handle,
                &initial_settings.all_shortcuts,
                false,
            )?;

            settings::autostart::adjust_autostart(app_handle, initial_settings.autostart)?;

            Ok(())
        }
        Err(err) => Err(err),
    }
}

fn activate_primary_window(primary_window_option: &Option<WebviewWindow>) -> () {
    if let Some(primary_window) = primary_window_option {
        let was_visible_before = primary_window.is_visible().unwrap_or(false);
        if let Err(err) = show_window(primary_window_option) {
            log::error!("Unable to show primary window by shortcut. {err}");
        } else {
            if let Err(err) = primary_window.emit_to(
                primary::SCREEN,
                primary::MAIN_SHORTCUT_PRESSED_EVENT,
                EventPayload {
                    data: MainShortcutPressedPayload {
                        changed_from_hidden_to_visile: !was_visible_before,
                    },
                },
            ) {
                log::error!("Unable to send shortcut pressed event. {err}");
            }
        }
    }

    // for some reason on linux app doesnt show form the first attempt
    #[cfg(target_os = "linux")]
    {
        let copy = primary_window_option.clone();

        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(50));
            if let Some(primary_window) = copy {
                if let Err(err) = show_window(&Some(primary_window.clone())) {
                    log::error!("Unable to show primary window by shortcut. {err}");
                }
            }
        });
    }
}
