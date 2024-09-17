use ccv_contract::{app_error, error::AppError};
use tauri::Window;

pub fn show_window(window: &Window) -> Result<(), AppError>{
    window.show().map_err(|_| app_error!("Error on show"))?;
    window.set_always_on_top(true).map_err(|_| app_error!("Error on set always on top"))?;
    window.set_focus().map_err(|_| app_error!("Error on focus"))?;
    window.set_always_on_top(false).map_err(|_| app_error!("Error on unset always on top"))?;
    Ok(())
}