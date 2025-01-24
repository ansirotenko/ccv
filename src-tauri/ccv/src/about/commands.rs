use crate::about;
use crate::utils::{
    get_app_data_dir, get_app_log_dir,
    window::{hide_window, show_window},
};
use ccv_contract::{
    error::{log_error, AppError},
    models::{AboutData, Os},
};
use tauri_plugin_opener::OpenerExt;
use std::env::current_exe;
use tauri::{command, AppHandle, Manager};

#[command]
pub fn get_about_data(app_handle: AppHandle) -> Result<AboutData, AppError> {
    let major_version = env!("CARGO_PKG_VERSION_MAJOR");
    let minor_version = env!("CARGO_PKG_VERSION_MINOR");
    let patch_version = env!("CARGO_PKG_VERSION_PATCH");
    let exe_path = log_error(current_exe(), "Unable to get current app location")?;
    let app_data_dir = log_error(get_app_data_dir(&app_handle), "Failed to get data dir")?;
    let app_logs_dir = log_error(get_app_log_dir(&app_handle), "Failed to get log dir")?;

    let os = if cfg!(target_os = "macos") {
        Os::MacOs
    } else {
        if cfg!(target_os = "windows") {
            Os::Windows
        } else {
            Os::Linux
        }
    };

    let about_data = AboutData {
        version: format!("{major_version}.{minor_version}.{patch_version}"),
        authors: env!("CARGO_PKG_AUTHORS").to_string(),
        homepage: env!("CARGO_PKG_HOMEPAGE").to_string(),
        app_dir: match exe_path.parent() {
            Some(app_dir) => app_dir.to_str().unwrap_or("").to_string(),
            None => String::new(),
        },
        app_data_dir: app_data_dir.to_str().unwrap_or("").to_string(),
        app_logs_dir: app_logs_dir.to_str().unwrap_or("").to_string(),
        text: env!("CARGO_PKG_DESCRIPTION").to_string(),
        os: os,
    };
    Ok(about_data)
}

#[command]
pub fn open_url(target: String, app_handle: AppHandle) -> Result<(), AppError> {
    log_error(app_handle.opener().open_url(target.as_str(), None::<&str>), "Unable to open url")
}

#[command]
pub fn open_path(target: String, app_handle: AppHandle) -> Result<(), AppError> {
    log_error(app_handle.opener().open_path(target.as_str(),  None::<&str>), "Unable to open path")
}

#[command]
pub fn hide_about_window(app_handle: AppHandle) -> Result<(), AppError> {
    log_error(
        hide_window(&app_handle.get_webview_window(about::SCREEN)),
        "Unable to hide about window",
    )
}

#[command]
pub fn show_about_window(app_handle: AppHandle) -> Result<(), AppError> {
    log_error(
        show_window(&app_handle.get_webview_window(about::SCREEN)),
        "Unable to show about window",
    )
}
