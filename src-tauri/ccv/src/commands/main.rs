use crate::{screens::MAIN, state::CopyItemState};
use ccv_contract::{
    app_error,
    error::{log_error, AppError},
    models::{
        CopyCategory::{self, Files, Html, Image, Rtf, Text, Unknown},
        CopyItem, CopyItemValue, FileInfo, SearchResult,
    },
    repository::Repository,
};
use chrono::Utc;
use clipboard_rs::{Clipboard, ClipboardContent, ClipboardContext, ContentFormat};
use std::path::Path;
use tauri::{command, AppHandle, Manager, State};
use tauri_plugin_clipboard::ClipboardManager;

#[command]
pub fn search_copy_items(
    query: Option<String>,
    page: i32,
    page_size: i32,
    categories: Vec<CopyCategory>,
    state: State<CopyItemState>,
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
    state: State<CopyItemState>,
    state_clipboard: State<ClipboardManager>,
) -> Result<CopyItem, AppError> {
    let repository = state.repository.lock().unwrap();

    let copy_item = log_error(
        repository.update_last_resue(&item_id, Utc::now()),
        "Error on updating last reuse",
    )?;

    log_error(
        write_reused_copy_item(&copy_item, state_clipboard),
        "Error on writing to clipboard",
    )?;

    Ok(copy_item)
}

fn write_reused_copy_item(
    copy_item: &CopyItem,
    state_clipboard: State<ClipboardManager>,
) -> Result<(), AppError> {
    state_clipboard.clear().map_err(|e| app_error!("{e}"))?;
    state_clipboard
        .has(ContentFormat::Other("a".to_string()))
        .unwrap();

    match copy_item.value.category {
        Files => {
            let file_uris = copy_item
                .value
                .files
                .as_ref()
                .unwrap()
                .iter()
                .map(|x| format!("file://{}", x.full_path))
                .collect();
            state_clipboard
                .write_files_uris(file_uris)
                .map_err(|e| app_error!("{e}"))?;
        }
        Image => {
            state_clipboard
                .write_image_base64(copy_item.value.image.as_ref().unwrap().clone())
                .map_err(|e| app_error!("{e}"))?;
        }
        Rtf => {
            // this solves problem of reusing rtf format. commented lines writes only rtf, but not text
            let ctx = log_error(ClipboardContext::new(), "Unable to create ClipboardContext")?;
            log_error(
                ctx.set(vec![
                    ClipboardContent::Text(copy_item.value.text.as_ref().unwrap().clone()),
                    ClipboardContent::Rtf(copy_item.value.rtf.as_ref().unwrap().clone()),
                ]),
                "Unable to write rtf and text content",
            )?;

            // state_clipboard
            //     .write_text(copy_item.value.text.as_ref().unwrap().clone())
            //     .map_err(|e| app_error!("{e}"))?;
            // state_clipboard
            //     .write_rtf(copy_item.value.rtf.as_ref().unwrap().clone())
            //     .map_err(|e| app_error!("{e}"))?;
        }
        Html => {
            state_clipboard
                .write_html_and_text(
                    copy_item.value.html.as_ref().unwrap().clone(),
                    copy_item.value.text.as_ref().unwrap().clone(),
                )
                .map_err(|e| app_error!("{e}"))?;
        }
        Text => {
            state_clipboard
                .write_text(copy_item.value.text.as_ref().unwrap().clone())
                .map_err(|e| app_error!("{e}"))?;
        }
        Unknown => {}
    };

    Ok(())
}

#[command]
pub fn insert_copy_item(
    state: State<CopyItemState>,
    state_clipboard: State<ClipboardManager>,
) -> Result<CopyItem, AppError> {
    let repository = state.repository.lock().unwrap();
    log_error(
        insert_copy_item_if_not_found(repository.as_ref(), state_clipboard),
        "Error on inserting new copy item",
    )
}

pub fn insert_copy_item_if_not_found(
    repository: &dyn Repository,
    state_clipboard: State<ClipboardManager>,
) -> Result<CopyItem, AppError> {
    let new_copy_item_value = log_error(
        get_new_copy_item_value(&state_clipboard),
        "Failed to get new copy item",
    )?;
    let existed_item = repository.find_by_value(&new_copy_item_value)?;
    if let Some(existed_item) = existed_item {
        Ok(existed_item)
    } else {
        log_error(
            repository.insert(&new_copy_item_value),
            "Error on inserting new copy item",
        )
    }
}

fn get_new_copy_item_value(
    clipboard_manager: &ClipboardManager,
) -> Result<CopyItemValue, AppError> {
    let category = get_clipboard_category(clipboard_manager)?;
    let mut image: Option<String> = None;
    let mut files: Option<Vec<FileInfo>> = None;
    let mut rtf: Option<String> = None;
    let mut html: Option<String> = None;
    let mut text: Option<String> = None;
    match category {
        Files => {
            let file_pathes = clipboard_manager
                .read_files()
                .map_err(|e| app_error!("{e}"))?;

            let file_infos = file_pathes
                .iter()
                .map(|fp| {
                    let path = Path::new(fp);
                    let directory_path = if let Some(parent) = path.parent() {
                        Some(parent.join("").to_str().unwrap_or("").to_string())
                    } else {
                        None
                    };
                    let file_name = path.file_name();

                    FileInfo {
                        full_path: fp.to_owned(),
                        directory_path: directory_path,
                        file_name: file_name
                            .map(|f| f.to_os_string().into_string().unwrap_or("".to_string())),
                        is_directory: path.is_dir(),
                        icon_base64: None,
                    }
                })
                .collect();
            files = Some(file_infos);
        }
        Image => {
            image = Some(
                clipboard_manager
                    .read_image_base64()
                    .map_err(|e| app_error!("{e}"))?,
            );
        }
        Rtf => {
            text = Some(
                clipboard_manager
                    .read_text()
                    .map_err(|e| app_error!("{e}"))?,
            );
            rtf = Some(
                clipboard_manager
                    .read_rtf()
                    .map_err(|e| app_error!("{e}"))?,
            );
        }
        Html => {
            text = Some(
                clipboard_manager
                    .read_text()
                    .map_err(|e| app_error!("{e}"))?,
            );
            html = Some(
                clipboard_manager
                    .read_html()
                    .map_err(|e| app_error!("{e}"))?,
            );
        }
        Text => {
            text = Some(
                clipboard_manager
                    .read_text()
                    .map_err(|e| app_error!("{e}"))?,
            );
        }
        Unknown => {}
    };

    Ok(CopyItemValue {
        text: text,
        rtf: rtf,
        html: html,
        image: image,
        files: files,
        category: category,
    })
}

#[command]
pub fn hide_main_window(app: AppHandle) -> Result<(), AppError> {
    let window = app.get_window(MAIN);
    if let Some(window) = window {
        log_error(window.hide(), "Unable to hide main window")?;
        Ok(())
    } else {
        log_error(
            Err(app_error!("Window option returns None")),
            "Unable to find main window",
        )
    }
}

#[command]
pub fn show_main_window(app: AppHandle) -> Result<(), AppError> {
    let window = app.get_window(MAIN);
    if let Some(window) = window {
        log_error(window.show(), "Unable to show main window")?;
        log_error(window.set_always_on_top(true), "Unable to focus about window")?;
        log_error(window.set_focus(), "Unable to focus main window")?;
        log_error(window.set_always_on_top(false), "Unable to focus about window")?;
        Ok(())
    } else {
        log_error(
            Err(app_error!("Window option returns None")),
            "Unable to find main window",
        )
    }
}

pub fn get_clipboard_category(
    clipboard_manager: &ClipboardManager,
) -> Result<CopyCategory, AppError> {
    if clipboard_manager
        .has_image()
        .map_err(|e| app_error!("{e}"))?
    {
        return Ok(Image);
    }
    if clipboard_manager
        .has_files()
        .map_err(|e| app_error!("{e}"))?
    {
        return Ok(Files);
    }
    if clipboard_manager.has_rtf().map_err(|e| app_error!("{e}"))? {
        return Ok(Rtf);
    }
    if clipboard_manager
        .has_html()
        .map_err(|e| app_error!("{e}"))?
    {
        return Ok(Html);
    }
    if clipboard_manager
        .has_text()
        .map_err(|e| app_error!("{e}"))?
    {
        return Ok(Text);
    }
    return Ok(Unknown);
}
