use crate::screens::ABOUT;
use ccv_contract::{
    app_error,
    error::{log_error, AppError},
    models::AboutData,
};
use std::env::current_exe;
use tauri::{command, AppHandle, Manager};

use super::utils::show_window;

#[command]
pub fn get_about_data(app: AppHandle) -> Result<AboutData, AppError> {
    let major_version = env!("CARGO_PKG_VERSION_MAJOR");
    let minor_version = env!("CARGO_PKG_VERSION_MINOR");
    let patch_version = env!("CARGO_PKG_VERSION_PATCH");
    let app_dir = log_error(current_exe(), "Unable to get current app location")?;
    let app_data_dir = app.app_handle().path_resolver().app_data_dir();
    let app_logs_dir = app.app_handle().path_resolver().app_log_dir();

    let about_data = AboutData {
        version: format!("{major_version}.{minor_version}.{patch_version}"),
        authors: env!("CARGO_PKG_AUTHORS").to_string(),
        homepage: env!("CARGO_PKG_HOMEPAGE").to_string(),
        app_dir: app_dir
            .parent()
            .map(|p| p.to_str().unwrap_or("").to_string()),
        app_data_dir: app_data_dir.map(|p| p.to_str().unwrap_or("").to_string()),
        app_logs_dir: app_logs_dir.map(|p| p.to_str().unwrap_or("").to_string()),
        text: env!("CARGO_PKG_DESCRIPTION").to_string(),
    };
    Ok(about_data)
}

#[command]
pub fn open(target: String) -> Result<(), AppError> {
    log_error(open::that(target), "Unable to open")
}

#[command]
pub fn hide_about_window(app: AppHandle) -> Result<(), AppError> {
    let window = app.get_window(ABOUT);
    if let Some(window) = window {
        log_error(window.hide(), "Unable to hide about window")?;
        Ok(())
    } else {
        log_error(
            Err(app_error!("Window option returns None")),
            "Unable to find about window",
        )
    }
}

#[command]
pub fn show_about_window(app: AppHandle) -> Result<(), AppError> {
    let window = app.get_window(ABOUT);
    if let Some(window) = window {
        log_error(show_window(&window), "Unable to show about window")
    } else {
        log_error(
            Err(app_error!("Window option returns None")),
            "Unable to find about window",
        )
    }
}