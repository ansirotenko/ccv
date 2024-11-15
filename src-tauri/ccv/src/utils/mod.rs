use std::path::PathBuf;

use ccv_contract::{app_error, error::AppError};
use tauri::{AppHandle, Manager};

pub mod window;

pub const WINDOW_SHOWN_EVENT: &str = "window-shown";
pub const WINDOW_HIDDEN_EVENT: &str = "window-hidden";

pub fn get_app_data_dir(app_handle: &AppHandle) -> Result<PathBuf, AppError> {
    #[cfg(target_os = "linux")]
    {
        None
    }

    app_handle
        .path()
        .app_data_dir()
        .map_err(|err| app_error!("Unable to get path to application data directory {err}"))
}

pub fn get_app_log_dir(app_handle: &AppHandle) -> Result<PathBuf, AppError> {
    app_handle
        .path()
        .app_log_dir()
        .map_err(|err| app_error!("Unable to get path to application log directory {err}"))
}
