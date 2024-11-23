use crate::utils;
use ccv_contract::{app_error, error::AppError, models::EventPayload};
use tauri::{Emitter, WebviewWindow};

pub fn show_window(window: &Option<WebviewWindow>) -> Result<(), AppError> {
    if let Some(window) = window {
        window.show().map_err(|_| app_error!("Error on show"))?;
        window
            .set_always_on_top(true)
            .map_err(|_| app_error!("Error on set always on top"))?;
        window
            .set_focus()
            .map_err(|_| app_error!("Error on focus"))?;
        window
            .set_always_on_top(false)
            .map_err(|_| app_error!("Error on unset always on top"))?;
        window
            .emit_to(
                window.label(),
                utils::WINDOW_SHOWN_EVENT,
                EventPayload {
                    data: format!("Window {} was shown", window.label()),
                },
            )
            .map_err(|_| app_error!("Error on send event"))?;
        Ok(())
    } else {
        Err(app_error!("Window was not found"))
    }
}

pub fn hide_window(window: &Option<WebviewWindow>) -> Result<(), AppError> {
    if let Some(window) = window {
        window.hide().map_err(|_| app_error!("Error on hide"))?;
        window
            .emit_to(
                window.label(),
                utils::WINDOW_HIDDEN_EVENT,
                EventPayload {
                    data: format!("Window {} was hidden", window.label()),
                },
            )
            .map_err(|_| app_error!("Error on send event"))?;
        Ok(())
    } else {
        Err(app_error!("Window was not found"))
    }
}

pub fn close_window(window: &Option<WebviewWindow>) -> Result<(), AppError> {
    if let Some(window) = window {
        window.close().map_err(|_| app_error!("Error on close"))?;
        Ok(())
    } else {
        Err(app_error!("Window was not found"))
    }
}
