use ccv_contract::{app_error, error::AppError};
use tauri::AppHandle;
use tauri_plugin_autostart::{Error, ManagerExt};

pub fn adjust_autostart(app_handle: &AppHandle, autostart: bool) -> Result<(), AppError> {
    adjust_autostart_inner(app_handle, autostart)
        .map_err(|err| app_error!("Unable to change autostart. {err}"))
}

fn adjust_autostart_inner(app_handle: &AppHandle, autostart: bool) -> Result<(), Error> {
    let autostart_manager = app_handle.autolaunch();

    if autostart && !autostart_manager.is_enabled()? {
        #[cfg(not(debug_assertions))] {
            autostart_manager.enable()?;
        }

        return Ok(());
    }
    if !autostart && autostart_manager.is_enabled()? {
        #[cfg(not(debug_assertions))] {
            autostart_manager.disable()?;
        }
        return Ok(());
    }

    Ok(())
}
