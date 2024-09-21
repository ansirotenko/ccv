use ccv_contract::{app_error, error::AppError};
use tauri::Window;

pub fn show_window(window: &Option<Window>) -> Result<(), AppError>{
    if let Some(window) = window {
        window.show().map_err(|_| app_error!("Error on show"))?;
        window.set_always_on_top(true).map_err(|_| app_error!("Error on set always on top"))?;
        window.set_focus().map_err(|_| app_error!("Error on focus"))?;
        window.set_always_on_top(false).map_err(|_| app_error!("Error on unset always on top"))?;
        Ok(())
    } else {
        Err(app_error!("Window was not found"))
    }
}

pub fn hide_window(window: &Option<Window>) -> Result<(), AppError> {
    if let Some(window) = window {
        window.hide().map_err(|_| app_error!("Error on hide"))?;
        Ok(())
    } else {
        Err(app_error!("Window was not found"))
    }
}

pub fn close_window(window: &Option<Window>) -> Result<(), AppError> {
    if let Some(window) = window {
        window.close().map_err(|_| app_error!("Error on close"))?;
        Ok(())
    } else {
        Err(app_error!("Window was not found"))
    }
}