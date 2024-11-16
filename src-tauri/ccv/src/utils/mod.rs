use std::path::PathBuf;

use ccv_contract::{app_error, error::AppError};
use tauri::{AppHandle, Manager};

pub mod window;

pub const WINDOW_SHOWN_EVENT: &str = "window-shown";
pub const WINDOW_HIDDEN_EVENT: &str = "window-hidden";

pub fn get_app_data_dir(app_handle: &AppHandle) -> Result<PathBuf, AppError> {
    //TODO REMOVE THIS LATER
    log::warn!("XDG_DATA_HOME {:?}", std::env::var_os("XDG_DATA_HOME"));
    log::warn!("HOME {:?}", std::env::var_os("HOME"));
    log::warn!("SUC {:?}", std::env::var_os("SUC"));
    log::warn!("SRH {:?}", std::env::var_os("SRH"));

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
