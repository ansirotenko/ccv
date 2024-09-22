use crate::settings;
use ccv_contract::error::AppError;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub fn read_settings_and_register_keybindings(
    app_handle: &AppHandle,
    app_data_dir: &PathBuf,
) -> Result<(), AppError> {
    let state_settings = app_handle.state::<settings::state::SettingsState>();
    let mut settings = state_settings.settings.lock().unwrap();
    match settings::core::read_settings(app_data_dir) {
        Ok(new_settings) => {
            *settings = Some(new_settings);
        }
        Err(err) => {
            log::error!("Unable to read settings file. {err}");
        }
    }

    let settings = settings.clone().unwrap();
    if let Err(err) = settings::core::register_keybindings(app_handle, &settings) {
        log::error!("Unable to register initial shortcuts. {err}");
    }

    Ok(())
}
