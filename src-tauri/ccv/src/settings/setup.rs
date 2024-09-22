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
    if let Err(err) = settings::core::register_keybindings(app_handle, &settings, &None) {
        log::error!("Unable to register initial shortcuts. {err}");
    }

    // TODO change to linux
    #[cfg(target_os = "linux")]
    {
        use global_hotkey::GlobalHotKeyEvent;
        use tauri::async_runtime;

        let primary_window = app_handle.get_window(crate::primary::SCREEN);
        
        async_runtime::spawn(async move {
            loop {
                if let Ok(_) = GlobalHotKeyEvent::receiver().try_recv() {
                    if let Err(err) = crate::utils::window::show_window(&primary_window) {
                        log::error!("Unable to show primary window. {err}");
                    }
                }
            }
        });
    } 

    Ok(())
}
