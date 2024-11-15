use std::path::PathBuf;

use ccv_contract::{app_error, error::AppError};
use tauri::{AppHandle, Manager};

pub mod window;

pub const WINDOW_SHOWN_EVENT: &str = "window-shown";
pub const WINDOW_HIDDEN_EVENT: &str = "window-hidden";

pub fn get_app_data_dir(app_handle: &AppHandle) -> Result<PathBuf, AppError> {
    #[cfg(target_os = "linux")]
    {
        return match dirs_sys::home_dir() {
            None => Err(app_error!("Unable to get path to $HOME directory")),
            Some(home_dir) => Ok(home_dir.join(".local/share").join(app_handle.config().identifier.as_str()))
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        app_handle
        .path()
        .app_data_dir()
        .map_err(|err| app_error!("Unable to get path to application data directory {err}"))
    }
}

pub fn get_app_log_dir(app_handle: &AppHandle) -> Result<PathBuf, AppError> {
    app_handle
        .path()
        .app_log_dir()
        .map_err(|err| app_error!("Unable to get path to application log directory {err}"))
}
