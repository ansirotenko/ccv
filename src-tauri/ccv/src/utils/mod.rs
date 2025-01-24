use std::path::PathBuf;

use ccv_contract::{app_error, error::AppError};
use tauri::{AppHandle, Manager};

pub mod window;

pub const WINDOW_SHOWN_EVENT: &str = "window-shown";
pub const WINDOW_HIDDEN_EVENT: &str = "window-hidden";

pub fn get_app_data_dir(app_handle: &AppHandle) -> Result<PathBuf, AppError> {
    #[cfg(target_os = "linux")]
    {
        // by default XDG_DATA_HOME environment variable is used instead of HOME environment variable
        // In case application was started by desktop-launch script, this variable is set to
        // export HOME=$SNAP_USER_DATA
        // export XDG_CONFIG_HOME=$SNAP_USER_DATA
        // This leads to new settings.json and ccv.db file at every new installation with snap
        // https://forum.snapcraft.io/t/is-xdg-data-home-modified-by-snaps-adapters/17826/7
        //
        // In case overriding $HOME or $XDG_CONFIG_HOME with with real $SNAP_REAL_HOME, autostart feature doesnt work properly
        if let Some(ccv_home) = std::env::var_os("SNAP_REAL_HOME") {
            return Ok(PathBuf::from(ccv_home).join(".local/share").join(app_handle.config().identifier.as_str()))
        };
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
