use crate::settings;
use ccv_contract::{error::AppError, models::Theme};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub fn read_settings_and_register_shortcuts(
    app_handle: &AppHandle,
    app_data_dir: &PathBuf,
) -> Result<(), AppError> {
    log::warn!("Settings data dir {:?}", app_data_dir);

    let settings_state = app_handle.state::<settings::state::SettingsState>();
    let mut settings = settings_state.settings.lock().unwrap();

    let default_theme = get_default_theme(app_handle);

    match settings::core::read_settings(app_data_dir, default_theme) {
        Ok(initial_settings) => {
            *settings = Some(initial_settings.clone());

            settings::autostart::adjust_autostart(app_handle, initial_settings.autostart)?;
            settings::shortcut::register_handlers(app_handle)?;

            if let Err(_) = settings::shortcut::register_shortcuts(
                app_handle,
                &initial_settings.all_shortcuts,
                false,
            ) {
                let mut initial_settings_with_notification = initial_settings.clone();
                let notification = settings::OCCUPIED_SHORTCUT_NOTIFICATION.to_string();
                if let Some(ref mut notifcations) = initial_settings_with_notification.notifications
                {
                    if !notifcations.contains(&notification) {
                        notifcations.push(notification);
                    }
                } else {
                    initial_settings_with_notification.notifications = Some(vec![notification])
                }

                settings::core::write_settings(app_data_dir, &initial_settings_with_notification)?;

                *settings = Some(initial_settings_with_notification);
            }

            Ok(())
        }
        Err(err) => Err(err),
    }
}

fn get_default_theme(app_handle: &AppHandle) -> Theme {
    match app_handle.get_webview_window(settings::SCREEN) {
        None => Theme::Light,
        Some(settings_window) => match settings_window.theme() {
            Err(_) => Theme::Light,
            Ok(theme) => match theme {
                tauri::Theme::Dark => Theme::Dark,
                _ => Theme::Light,
            },
        },
    }
}
