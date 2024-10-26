use chrono::{DateTime, Utc};

use crate::{
    error::AppError,
    models::{CopyCategory, CopyItem, CopyItemValue, SearchResult},
};

pub trait Repository {
    fn search(
        &self,
        query: Option<&str>,
        page: i32,
        page_size: i32,
        categories: Vec<CopyCategory>,
    ) -> Result<SearchResult, AppError>;
    fn insert(&self, item_value: &CopyItemValue) -> Result<CopyItem, AppError>;
    fn find_by_value(&self, item_value: &CopyItemValue) -> Result<Option<CopyItem>, AppError>;
    fn update_last_resue(
        &self,
        item_id: &str,
        new_last_reuse: DateTime<Utc>,
    ) -> Result<CopyItem, AppError>;
    fn remove(&self, item_ids: &Vec<&str>) -> Result<usize, AppError>;
    fn remove_older(&self, sinse: DateTime<Utc>) -> Result<usize, AppError>;
}
