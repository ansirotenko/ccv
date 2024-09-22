use std::sync::Mutex;

use ccv_contract::{app_error, error::AppError, repository::Repository};

// TODO rename to PrimaryState
pub struct CopyItemState {
    pub repository: Mutex<Box<dyn Repository + Send + Sync + 'static>>,
}

impl CopyItemState {
    pub fn new_uninitialized() -> CopyItemState {
        CopyItemState {
            repository: Mutex::new(Box::new(UninitalizedRepository {})),
        }
    }
}

struct UninitalizedRepository;

impl Repository for UninitalizedRepository {
    fn search(
        &self,
        _query: Option<&str>,
        _page: i32,
        _page_size: i32,
        _categories: Vec<ccv_contract::models::CopyCategory>,
    ) -> Result<ccv_contract::models::SearchResult, AppError> {
        Err(app_error!("Repository is unitialzed"))
    }

    fn insert(
        &self,
        _item_value: &ccv_contract::models::CopyItemValue,
    ) -> Result<ccv_contract::models::CopyItem, AppError> {
        Err(app_error!("Repository is unitialzed"))
    }

    fn find_by_value(
        &self,
        _item_value: &ccv_contract::models::CopyItemValue,
    ) -> Result<Option<ccv_contract::models::CopyItem>, AppError> {
        Err(app_error!("Repository is unitialzed"))
    }

    fn update_last_resue(
        &self,
        _item_id: &str,
        _new_last_reuse: chrono::DateTime<chrono::Utc>,
    ) -> Result<ccv_contract::models::CopyItem, AppError> {
        Err(app_error!("Repository is unitialzed"))
    }

    fn remove(&self, _item_ids: &Vec<&str>) -> Result<(), AppError> {
        Err(app_error!("Repository is unitialzed"))
    }

    fn remove_older(&self, _sinse: chrono::DateTime<chrono::Utc>) -> Result<(), AppError> {
        Err(app_error!("Repository is unitialzed"))
    }
}
