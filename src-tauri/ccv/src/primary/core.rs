use ccv_contract::{
    app_error,
    error::AppError,
    models::{
        CopyCategory::{self, Files, Html, Image, Rtf, Text, Unknown},
        CopyItem, CopyItemValue, FileInfo,
    },
    repository::Repository,
};
use chrono::Utc;
use std::path::Path;
use tauri::State;
use tauri_plugin_clipboard::Clipboard;

pub struct InsertIfNotFoundResult {
    pub copy_item: CopyItem,
    pub already_exist: bool,
}

pub fn insert_copy_item_if_not_found(
    repository: &dyn Repository,
    clipboard: &State<Clipboard>,
) -> Result<InsertIfNotFoundResult, AppError> {
    let category = get_clipboard_category(clipboard)?;
    if category == CopyCategory::Unknown {
        log::warn!("Found copy item category Unknown, waiting 50 ms");
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    let new_copy_item_value = get_new_copy_item_value(clipboard)?;
    if new_copy_item_value.category == CopyCategory::Unknown {
        log::warn!("Found copy item category Unknown even after thread sleep");
    }

    let existed_item = repository.find_by_value(&new_copy_item_value)?;
    if let Some(existed_item) = existed_item {
        let existed_item_updated_last_reuse = repository.update_last_resue(&existed_item.id, Utc::now())?;
        Ok(InsertIfNotFoundResult {
            copy_item: existed_item_updated_last_reuse,
            already_exist: true,
        })
    } else {
        let new_copy_item = repository.insert(&new_copy_item_value)?;
        Ok(InsertIfNotFoundResult {
            copy_item: new_copy_item,
            already_exist: false,
        })
    }
}

fn get_new_copy_item_value(clipboard_state: &State<Clipboard>) -> Result<CopyItemValue, AppError> {
    let category = get_clipboard_category(clipboard_state)?;
    let mut image: Option<String> = None;
    let mut files: Option<Vec<FileInfo>> = None;
    let mut rtf: Option<String> = None;
    let mut html: Option<String> = None;
    let mut text: Option<String> = None;
    match category {
        Files => {
            let file_pathes: Vec<String> = clipboard_state
                .read_files()
                .map_err(|e| app_error!("{e}"))?
                .iter()
                .map(|fp| {
                    #[cfg(target_os = "linux")]
                    {
                        match urlencoding::decode(fp) {
                            Err(_) => fp.to_string(),
                            Ok(decoded) => decoded.to_string(),
                        }
                    }
                    #[cfg(not(target_os = "linux"))]
                    {
                        fp.to_string()
                    }
                })
                .collect();

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
                        icon_base64: if path.is_dir() {
                            None
                        } else {
                            get_file_icon(fp)
                        },
                    }
                })
                .collect();
            files = Some(file_infos);
        }
        Image => {
            image = Some(
                clipboard_state
                    .read_image_base64()
                    .map_err(|e| app_error!("{e}"))?,
            );
        }
        Rtf => {
            text = Some(clipboard_state.read_text().map_err(|e| app_error!("{e}"))?);
            rtf = Some(clipboard_state.read_rtf().map_err(|e| app_error!("{e}"))?);
        }
        Html => {
            text = Some(clipboard_state.read_text().map_err(|e| app_error!("{e}"))?);
            html = Some(clipboard_state.read_html().map_err(|e| app_error!("{e}"))?);
        }
        Text => {
            text = Some(clipboard_state.read_text().map_err(|e| app_error!("{e}"))?);
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

pub fn get_file_icon(file_path: &str) -> Option<String> {
    match systemicons::get_icon(file_path, 16) {
        Err(_) => None,
        Ok(icon_bytes) => {
            use base64::{engine::general_purpose::STANDARD, Engine as _};
            Some(STANDARD.encode(icon_bytes))
        }
    }
}

pub fn get_clipboard_category(
    clipboard_state: &State<Clipboard>,
) -> Result<CopyCategory, AppError> {
    if clipboard_state.has_image().map_err(|e| app_error!("{e}"))? {
        return Ok(Image);
    }
    if clipboard_state.has_files().map_err(|e| app_error!("{e}"))? {
        return Ok(Files);
    }
    if clipboard_state.has_rtf().map_err(|e| app_error!("{e}"))? {
        return Ok(Rtf);
    }
    if clipboard_state.has_html().map_err(|e| app_error!("{e}"))? {
        return Ok(Html);
    }
    if clipboard_state.has_text().map_err(|e| app_error!("{e}"))? {
        return Ok(Text);
    }
    return Ok(Unknown);
}

pub fn write_reused_copy_item(
    copy_item: &CopyItem,
    clipboard_state: &State<Clipboard>,
) -> Result<(), AppError> {
    clipboard_state.clear().map_err(|e| app_error!("{e}"))?;

    match copy_item.value.category {
        Files => {
            let files_paths: Vec<String> = copy_item
                .value
                .files
                .as_ref()
                .unwrap()
                .iter()
                .map(|x| x.full_path.clone())
                .collect();

            let mut files_uris: Vec<String> = vec![];
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            {
                for file in &files_paths {
                    files_uris.push(format!("file://{}", file))
                }
            }

            #[cfg(target_os = "windows")]
            {
                for file in &files_paths {
                    files_uris.push(file.clone())
                }
            }

            clipboard_state
                .write_files_uris(files_uris)
                .map_err(|e| app_error!("{e}"))?;
        }
        Image => {
            clipboard_state
                .write_image_base64(copy_item.value.image.as_ref().unwrap().clone())
                .map_err(|e| app_error!("{e}"))?;
        }
        Rtf => {
            use clipboard_rs::{Clipboard, ClipboardContent};
            let ctx = clipboard_state
                .clipboard
                .lock()
                .map_err(|err| app_error!("{err}"))?;

            ctx.set(vec![
                ClipboardContent::Text(copy_item.value.text.as_ref().unwrap().clone()),
                ClipboardContent::Rtf(copy_item.value.rtf.as_ref().unwrap().clone()),
            ])
            .map_err(|e| app_error!("{e}"))?;

            // clipboard_state
            //     .write_text(copy_item.value.text.as_ref().unwrap().clone())
            //     .map_err(|e| app_error!("{e}"))?;
            // clipboard_state
            //     .write_rtf(copy_item.value.rtf.as_ref().unwrap().clone())
            //     .map_err(|e| app_error!("{e}"))?;
        }
        Html => {
            clipboard_state
                .write_html_and_text(
                    copy_item.value.html.as_ref().unwrap().clone(),
                    copy_item.value.text.as_ref().unwrap().clone(),
                )
                .map_err(|e| app_error!("{e}"))?;
        }
        Text => {
            clipboard_state
                .write_text(copy_item.value.text.as_ref().unwrap().clone())
                .map_err(|e| app_error!("{e}"))?;
        }
        Unknown => {}
    };

    Ok(())
}
