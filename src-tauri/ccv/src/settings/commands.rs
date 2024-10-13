use crate::primary;
use crate::settings;
use crate::utils::window::{hide_window, show_window};
use ccv_contract::{
    app_error,
    error::{log_error, AppError},
    models::{EventPayload, Settings},
};
use chrono::{DateTime, Utc};
use tauri::{command, AppHandle, Emitter, Manager, State};
use tauri_plugin_clipboard::Clipboard;

#[command]
pub fn get_settings(
    state: State<settings::state::SettingsState>,
) -> Result<Option<Settings>, AppError> {
    let settings = state.settings.lock().unwrap();
    Ok(settings.clone())
}

#[command]
pub fn set_settings(
    new_settings: Settings,
    app_handle: AppHandle,
    state: State<settings::state::SettingsState>,
) -> Result<(), AppError> {
    let mut settings = state.settings.lock().unwrap();
    let old_settings = settings.clone();
    *settings = Some(new_settings.clone());

    log_error(
        settings::core::store_and_notify_settings(&app_handle, &old_settings, &new_settings),
        "Unable store and notify settings changed",
    )
}

#[command]
pub fn hide_settings_window(app_handle: AppHandle) -> Result<(), AppError> {
    log_error(
        hide_window(&app_handle.get_webview_window(settings::SCREEN)),
        "Unable to hide settings window",
    )
}

#[command]
pub fn show_settings_window(app_handle: AppHandle) -> Result<(), AppError> {
    log_error(
        show_window(&app_handle.get_webview_window(settings::SCREEN)),
        "Unable to show settings window",
    )
}

#[command]
pub fn remove_copy_items(
    item_ids: String,
    app_handle: AppHandle,
    state: State<primary::state::PrimaryState>,
    clipboard_state: State<Clipboard>,
) -> Result<(), AppError> {
    let splitted_item_ids: Vec<&str> = item_ids.split(",").into_iter().map(|x| x.trim()).collect();
    if splitted_item_ids.is_empty() {
        return Err(app_error!("No id specified"));
    }
    let repository = state.repository.lock().unwrap();
    log_error(
        repository.remove(&splitted_item_ids),
        "Error on removing copy items",
    )?;
    log_error(
        primary::core::insert_copy_item_if_not_found(repository.as_ref(), &clipboard_state),
        "Unable to insert item after deletion",
    )?;

    log_error(
        app_handle.emit_to(
            primary::SCREEN,
            primary::ITEMS_CHANGED_EVENT,
            EventPayload {
                data: format!("Delete items with ids '{item_ids}'"),
            },
        ),
        "Unable to send event",
    )?;

    Ok(())
}

#[command]
pub fn remove_copy_items_older(
    sinse: DateTime<Utc>,
    app_handle: AppHandle,
    state: State<primary::state::PrimaryState>,
    clipboard_state: State<Clipboard>,
) -> Result<(), AppError> {
    let repository = state.repository.lock().unwrap();
    log_error(
        repository.remove_older(sinse),
        "Error on removing old copy items",
    )?;
    log_error(
        primary::core::insert_copy_item_if_not_found(repository.as_ref(), &clipboard_state),
        "Unable to insert item after deletion",
    )?;

    log_error(
        app_handle.emit_to(
            primary::SCREEN,
            primary::ITEMS_CHANGED_EVENT,
            EventPayload {
                data: format!("Delete items older {sinse}"),
            },
        ),
        "Unable to send event",
    )?;

    Ok(())
}
