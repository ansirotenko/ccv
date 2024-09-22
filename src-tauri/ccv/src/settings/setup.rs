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

    #[cfg(not(target_os = "linux"))]
    {
        let settings = settings.clone().unwrap();
        if let Err(err) = settings::core::register_keybindings(app_handle, &settings) {
            log::error!("Unable to register initial shortcuts. {err}");
        }
    }

    // TODO change to linux
    #[cfg(target_os = "linux")]
    {
        use crate::primary;
        use crate::utils::window::show_window;
        use ccv_contract::models::Settings;
        use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
        use std::sync::mpsc::channel;
        use tauri::async_runtime;

        let primary_window = app_handle.get_window(primary::SCREEN);
        let (tx, rx) = channel::<Settings>();
        let mut hotkey_change = state_settings.hotkey_change.lock().unwrap();
        *hotkey_change = Some(tx);
        let settings = settings.clone().unwrap();
        async_runtime::spawn(async move {
            let manager = GlobalHotKeyManager::new().unwrap();
            let mut hotkey = settings::core::get_hotkey(&settings.keybindings.open_ccv).unwrap();
            manager.register(hotkey).unwrap();
            loop {
                if let Ok(_) = GlobalHotKeyEvent::receiver().try_recv() {
                    if let Err(err) = show_window(&primary_window) {
                        log::error!("Unable to show primary window. {err}");
                    }
                }

                if let Ok(settings) = rx.try_recv() {
                    manager.unregister(hotkey).unwrap();
                    hotkey = settings::core::get_hotkey(&settings.keybindings.open_ccv).unwrap();
                    manager.register(hotkey).unwrap();
                }

                // TODO
                // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });
    }

    Ok(())
}
