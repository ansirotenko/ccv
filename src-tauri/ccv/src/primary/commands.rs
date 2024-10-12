use crate::primary;
use crate::utils::window::{hide_window, show_window};
use ccv_contract::{
    error::{log_error, AppError},
    models::{CopyCategory, CopyItem, SearchResult},
};
use chrono::Utc;
use tauri::{command, AppHandle, Manager, State};
use tauri_plugin_clipboard::Clipboard;

#[command]
pub fn search_copy_items(
    query: Option<String>,
    page: i32,
    page_size: i32,
    categories: Vec<CopyCategory>,
    state: State<primary::state::PrimaryState>,
) -> Result<SearchResult, AppError> {
    let repository = state.repository.lock().unwrap();
    log_error(
        repository.search(
            query.as_ref().map(|x| x.as_str()),
            page,
            page_size,
            categories,
        ),
        "Error while searching copy items",
    )
}

#[command]
pub fn reuse_copy_item(
    item_id: String,
    state: State<primary::state::PrimaryState>,
    clipboard_state: State<Clipboard>,
) -> Result<CopyItem, AppError> {
    let repository = state.repository.lock().unwrap();

    let copy_item = log_error(
        repository.update_last_resue(&item_id, Utc::now()),
        "Error on updating last reuse",
    )?;

    log_error(
        primary::core::write_reused_copy_item(&copy_item, &clipboard_state),
        "Error on writing to clipboard",
    )?;

    Ok(copy_item)
}

#[command]
pub fn insert_copy_item(
    state: State<primary::state::PrimaryState>,
    clipboard_state: State<Clipboard>,
) -> Result<CopyItem, AppError> {
    let repository = state.repository.lock().unwrap();
    log_error(
        primary::core::insert_copy_item_if_not_found(repository.as_ref(), &clipboard_state),
        "Error on inserting new copy item",
    )
}

#[command]
pub fn hide_primary_window(app_handle: AppHandle) -> Result<(), AppError> {
    log_error(
        hide_window(&app_handle.get_webview_window(primary::SCREEN)),
        "Unable to hide primary window",
    )
}

#[command]
pub fn show_primary_window(app_handle: AppHandle) -> Result<(), AppError> {
    log_error(
        show_window(&app_handle.get_webview_window(primary::SCREEN)),
        "Unable to show primary window",
    )
}
