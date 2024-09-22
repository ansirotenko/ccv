use crate::primary;
use ccv_contract::{error::AppError, models::CopyCategory::Unknown};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tauri_plugin_clipboard::ClipboardManager;

pub fn init_repository(app_handle: AppHandle, app_data_dir: &PathBuf) -> Result<(), AppError> {
    let state_clipboard = app_handle.state::<ClipboardManager>();
    let state = app_handle.state::<primary::state::PrimaryState>();
    let mut repository = state.repository.lock().unwrap();
    #[cfg(feature = "sqlite")]
    {
        let database_path = app_data_dir.join("ccv.db");
        let sqlite_repo =
            ccv_sqlite::repository::SqliteRepository::new(database_path.to_str().unwrap())?;
        *repository = Box::new(sqlite_repo);
    }

    #[cfg(not(feature = "sqlite"))]
    {
        let data: Vec<ccv_contract::models::CopyItem>;
        use ccv_in_memory;
        #[cfg(feature = "in-memory-test-data")]
        {
            data = ccv_in_memory::sample_data::every_type::every_type();
        }
        #[cfg(not(feature = "in-memory-test-data"))]
        {
            data = vec![];
        }
        *repository = Box::new(ccv_in_memory::repository::InMemoryRepository::new(data));
    }

    let category = primary::core::get_clipboard_category(&state_clipboard);
    match category {
        Err(e) => {
            log::warn!("Failed to identify clipboard value: {e}")
        }
        Ok(Unknown) => {}
        _ => {
            primary::core::insert_copy_item_if_not_found(repository.as_ref(), state_clipboard)?;
        }
    };

    Ok(())
}
