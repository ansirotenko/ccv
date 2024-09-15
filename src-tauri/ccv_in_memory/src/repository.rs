use std::sync::Mutex;

use chrono::{DateTime, Utc};
use ccv_contract::{
    app_error,
    error::AppError,
    models::{CopyCategory, CopyItem, CopyItemValue, SearchResult},
    repository::Repository,
};

pub struct InMemoryRepository {
    items: Mutex<Vec<CopyItem>>,
}

impl InMemoryRepository {
    pub fn new(seed: Vec<CopyItem>) -> Self {
        InMemoryRepository {
            items: Mutex::new(seed),
        }
    }
}

impl Repository for InMemoryRepository {
    fn search(
        &self,
        query: Option<&str>,
        page: i32,
        page_size: i32,
        categories: Vec<CopyCategory>
    ) -> Result<SearchResult, AppError> {
        let page = usize::try_from(page).map_err(|_| app_error!("Page cannot be negative"))?;
        let page_size = usize::try_from(page_size).map_err(|_| app_error!("Page size cannot be negative"))?;
        if page_size <= 0 {
            return Err(app_error!("Page size cannot be zero"));
        }

        let mut items = self.items.lock().unwrap();
        items.sort_by(|a, b| b.last_reuse.cmp(&a.last_reuse));

        let get_iterator = || {
            items.iter()
                .filter(|c| {
                    if let Some(query) = query {
                        if let Some(text) = &c.value.text {
                            return text.contains(query);
                        }
                        if let Some(files) = &c.value.files {
                            return files.iter().any(|f|f.full_path.contains(query));
                        }
                        return false;
                    }
                    return true;
                })
                .filter(|c| categories.contains(&c.value.category))
        };

        let total_number = get_iterator().count();

        let result: Vec<CopyItem> = get_iterator()
            .skip(page * page_size)
            .take(page_size)
            .map(|c| c.clone())
            .collect();

        Ok(SearchResult{
            items: result,
            total_number: total_number
        })
    }

    fn insert(&self, item_value: &CopyItemValue) -> Result<CopyItem, AppError> {
        let mut items = self.items.lock().unwrap();
        let date = Utc::now();
        let new_item = CopyItem {
            id: items.len().to_string(),
            created: date,
            last_reuse: date,
            value: item_value.clone()
        };
        items.push(new_item.clone());
        Ok(new_item)
    }

    fn update_last_resue(&self, item_id: &str, new_last_reuse: DateTime<Utc>) -> Result<CopyItem, AppError> {
        let mut items = self.items.lock().unwrap();
        if let Some(index) = items.iter().position(|c| c.id == item_id) {
            let item = items.get_mut(index).unwrap();
            item.last_reuse = new_last_reuse;
            Ok(item.clone())
        } else {
            Err(app_error!("Cannot find item with id {item_id}"))
        }
    }

    fn find_by_value(&self, item_value: &CopyItemValue) -> Result<Option<CopyItem>, AppError> {
        let items = self.items.lock().unwrap();
        if let Some(index) = items.iter().position(|c| &c.value == item_value) {
            Ok(Some(items.get(index).unwrap().clone()))
        } else {
            Ok(None)
        }
    }

    fn remove(&self, item_ids: &Vec<&str>) -> Result<(), AppError> {
        let mut items = self.to_owned().items.lock().unwrap();
        items.retain(|c| !item_ids.contains(&c.id.as_str()));
        Ok(())
    }

    fn remove_older(&self, sinse: DateTime<Utc>) -> Result<(), AppError> {
        let mut items = self.to_owned().items.lock().unwrap();
        items.retain(|c| c.last_reuse >= sinse);
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::vec;

    use chrono::TimeZone;
    use ccv_contract::models::{CopyCategory::{Files, Html, Image, Rtf, Text, Unknown}, FileInfo};

    use super::*;

    fn copy_item_unknown(id: &str, created: DateTime<Utc>, last_reuse: DateTime<Utc>) -> CopyItem {
        CopyItem{
            id: id.to_string(), 
            created: created, 
            last_reuse: last_reuse, 
            value: CopyItemValue { category: Unknown, image: None, rtf: None, files: None, text: None, html: None } 
        }
    }
    fn copy_item_text(id: &str, s: &str, created: DateTime<Utc>, last_reuse: DateTime<Utc>) -> CopyItem {
        CopyItem{
            id: id.to_string(), 
            created: created, 
            last_reuse: last_reuse, 
            value: CopyItemValue { category: Text, image: None, rtf: None, files: None, text: Some(s.to_string()), html: None } 
        }
    }
    fn copy_item_image(id: &str, s: &str, created: DateTime<Utc>, last_reuse: DateTime<Utc>) -> CopyItem {
        CopyItem{
            id: id.to_string(), 
            created: created, 
            last_reuse: last_reuse, 
            value: CopyItemValue { category: Image, image: Some(s.to_string()), rtf: None, files: None, text: None, html: None } 
        }
    }
    fn copy_item_html(id: &str, s: &str, created: DateTime<Utc>, last_reuse: DateTime<Utc>) -> CopyItem {
        CopyItem{
            id: id.to_string(), 
            created: created, 
            last_reuse: last_reuse, 
            value: CopyItemValue { category: Html, image:None, rtf: None, files: None, text: Some(s.to_string()), html: Some(s.to_string()) } 
        }
    }
    fn copy_item_rtf(id: &str, s: &str, created: DateTime<Utc>, last_reuse: DateTime<Utc>) -> CopyItem {
        CopyItem{
            id: id.to_string(), 
            created: created, 
            last_reuse: last_reuse, 
            value: CopyItemValue { category: Rtf, image:None, rtf: Some(s.to_string()), files: None, text: Some(s.to_string()), html: None } 
        }
    }
    fn copy_item_files(id: &str, s: &str, created: DateTime<Utc>, last_reuse: DateTime<Utc>) -> CopyItem {
        let files = vec![
            FileInfo {
                directory_path: Some(s.to_string()),
                full_path: s.to_string(),
                file_name: Some(s.to_string()),
                icon_base64: None,
                is_directory: true
            },
            FileInfo {
                directory_path: Some(s.to_string()),
                full_path: s.to_string(),
                file_name: Some(s.to_string()),
                icon_base64: None,
                is_directory: false
            }
        ];
        CopyItem{
            id: id.to_string(), 
            created: created, 
            last_reuse: last_reuse, 
            value: CopyItemValue { category: Files, image:None, rtf: None, files: Some(files), text: None, html: None } 
        }
    }

    #[test]
    fn search_paging() {
        let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        let t1 = Utc.with_ymd_and_hms(2001, 1, 1, 0, 0, 0).unwrap();
        let t2 = Utc.with_ymd_and_hms(2002, 1, 1, 0, 0, 0).unwrap();
        let t3 = Utc.with_ymd_and_hms(2003, 1, 1, 0, 0, 0).unwrap();
        let t4 = Utc.with_ymd_and_hms(2004, 1, 1, 0, 0, 0).unwrap();
        let t5 = Utc.with_ymd_and_hms(2005, 1, 1, 0, 0, 0).unwrap();
        let t6 = Utc.with_ymd_and_hms(2006, 1, 1, 0, 0, 0).unwrap();

        let rtf = copy_item_rtf("1", "ab", t, t1);
        let html = copy_item_html("2", "ab", t, t2);
        let image = copy_item_image("3", "ab", t, t3);
        let files = copy_item_files("4", "ab", t, t4);
        let text = copy_item_text("5", "ab", t, t5);
        let unknown = copy_item_unknown("6", t, t6);
        
        let repo = InMemoryRepository::new(vec![rtf.clone(), html.clone(), image.clone(), files.clone(), text.clone(), unknown.clone()]);

        let result = repo.search(None, 1, 2, vec![Unknown, Image, Rtf, Files, Text, Html]).unwrap();

        assert_eq!(result.items, vec![files, image]);
        assert_eq!(result.total_number, 6);
    }

    #[test]
    fn search_sorting() {
        let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        let t1 = Utc.with_ymd_and_hms(2001, 1, 1, 0, 0, 0).unwrap();
        let t2 = Utc.with_ymd_and_hms(2002, 1, 1, 0, 0, 0).unwrap();
        let t3 = Utc.with_ymd_and_hms(2003, 1, 1, 0, 0, 0).unwrap();
        let t4 = Utc.with_ymd_and_hms(2004, 1, 1, 0, 0, 0).unwrap();
        let t5 = Utc.with_ymd_and_hms(2005, 1, 1, 0, 0, 0).unwrap();
        let t6 = Utc.with_ymd_and_hms(2006, 1, 1, 0, 0, 0).unwrap();

        let rtf = copy_item_rtf("1", "ab", t, t1);
        let html = copy_item_html("2", "ab", t, t2);
        let image = copy_item_image("3", "ab", t, t3);
        let files = copy_item_files("4", "ab", t, t4);
        let text = copy_item_text("5", "ab", t, t5);
        let unknown = copy_item_unknown("6", t, t6);

        let repo = InMemoryRepository::new(vec![rtf.clone(), html.clone(), image.clone(), files.clone(), text.clone(), unknown.clone()]);

        let result = repo.search(None, 0, 100, vec![Unknown, Image, Rtf, Files, Text, Html]).unwrap();

        assert_eq!(result.items, vec![unknown, text, files, image, html, rtf]);
        assert_eq!(result.total_number, 6);
    }

    #[test]
    fn search_filter_by_query_and_category() {
        let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();

        let rtf = copy_item_rtf("1", "ab", t, t);
        let html = copy_item_html("2", "ab", t, t);
        let image = copy_item_image("3", "ab", t, t);
        let files = copy_item_files("4", "ab", t, t);
        let text = copy_item_text("5", "ab", t, t);
        let unknown = copy_item_unknown("6", t, t);
        
        let repo = InMemoryRepository::new(vec![rtf.clone(), html.clone(), image.clone(), files.clone(), text.clone(), unknown.clone()]);

        let result = repo.search(Some("a"), 0, 100, vec![Unknown, Image, Rtf, Files]).unwrap();

        assert_eq!(result.items, vec![rtf, files]);
        assert_eq!(result.total_number, 2);
    }

    #[test]
    fn search_filter_by_category() {
        let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();

        let rtf = copy_item_rtf("1", "ab", t, t);
        let html = copy_item_html("2", "ab", t, t);
        let image = copy_item_image("3", "ab", t, t);
        let files = copy_item_files("4", "ab", t, t);
        let text = copy_item_text("5", "ab", t, t);
        let unknown = copy_item_unknown("6", t, t);
        
        let repo = InMemoryRepository::new(vec![rtf.clone(), html.clone(), image.clone(), files.clone(), text.clone(), unknown.clone()]);

        let result = repo.search(None, 0, 100, vec![Image]).unwrap();

        assert_eq!(result.items, vec![image]);
        assert_eq!(result.total_number, 1);
    }

    #[test]
    fn search_filter_by_query() {
        let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();

        let rtf = copy_item_rtf("1", "ab", t, t);
        let html = copy_item_html("2", "ab", t, t);
        let image = copy_item_image("3", "ab", t, t);
        let files = copy_item_files("4", "ab", t, t);
        let text = copy_item_text("5", "ab", t, t);
        let unknown = copy_item_unknown("6", t, t);

        let repo = InMemoryRepository::new(vec![rtf.clone(), html.clone(), image.clone(), files.clone(), text.clone(), unknown.clone()]);

        let result = repo.search(Some("b"), 0, 100, vec![Unknown, Html, Rtf, Image, Text, Files]).unwrap();

        assert_eq!(result.items, vec![rtf, html, files, text]);
        assert_eq!(result.total_number, 4);
    }

    #[test]
    fn search_empty_filter() {
        let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();

        let rtf = copy_item_rtf("1", "ab", t, t);
        let html = copy_item_html("2", "ab", t, t);
        let image = copy_item_image("3", "ab", t, t);
        let files = copy_item_files("4", "ab", t, t);
        let text = copy_item_text("5", "ab", t, t);
        let unknown = copy_item_unknown("6", t, t);

        let repo = InMemoryRepository::new(vec![rtf.clone(), html.clone(), image.clone(), files.clone(), text.clone(), unknown.clone()]);

        let result = repo.search(None, 0, 100, vec![Unknown, Html, Rtf, Image, Text, Files]).unwrap();

        assert_eq!(result.items, vec![rtf, html, image, files, text, unknown]);
        assert_eq!(result.total_number, 6);
    }

    #[test]
    fn search_false_filter() {
        let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();

        let rtf = copy_item_rtf("1", "ab", t, t);
        let html = copy_item_html("2", "ab", t, t);
        let image = copy_item_image("3", "ab", t, t);
        let files = copy_item_files("4", "ab", t, t);
        let text = copy_item_text("5", "ab", t, t);
        let unknown = copy_item_unknown("6", t, t);

        let repo = InMemoryRepository::new(vec![rtf.clone(), html.clone(), image.clone(), files.clone(), text.clone(), unknown.clone()]);

        let result = repo.search(Some("c"), 0, 100, vec![Unknown, Html, Rtf, Image, Text, Files]).unwrap();

        assert_eq!(result.items.len(), 0);
        assert_eq!(result.total_number, 0);
    }

    #[test]
    fn insert_happy_path() {
        let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();

        let item1 = copy_item_unknown("0", t, t);
        let item2 = copy_item_unknown("1", t, t);

        let repo = InMemoryRepository::new(vec![item1.clone(), item2.clone()]);

        let new_item = copy_item_files("", "a", t, t);
        let inserted = repo.insert(&new_item.value).unwrap();

        let items = repo.items.lock().unwrap();
        assert_eq!(items.len(), 3);
        assert_eq!(items.get(0).unwrap().id, "0");
        assert_eq!(items.get(1).unwrap().id, "1");
        assert_eq!(items.get(2).unwrap().id, "2");
        assert_eq!(items.get(2).unwrap().created, items.get(2).unwrap().last_reuse);
        assert_eq!(items.get(2).unwrap().value, new_item.value);
        assert_eq!(inserted, items.get(2).unwrap().clone());
    }

    #[test]
    fn update_last_resue_happy_path() {
        let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();

        let item1 = copy_item_unknown("1", t, t);
        let item2 = copy_item_unknown("2", t, t);

        let repo = InMemoryRepository::new(vec![item1.clone(), item2.clone()]);

        let new_last_reuse = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let result = repo.update_last_resue("1", new_last_reuse).unwrap();

        let items = repo.items.lock().unwrap().clone();
        assert_eq!(items, vec![copy_item_unknown("1", t, new_last_reuse), item2]);
        assert_eq!(result, copy_item_unknown("1", t, new_last_reuse));
    }
    
    #[test]
    #[should_panic]
    fn update_last_resue_has_no_effect() {
        let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();

        let item1 = copy_item_unknown("1", t, t);
        let item2 = copy_item_unknown("2", t, t);

        let repo = InMemoryRepository::new(vec![item1.clone(), item2.clone()]);

        let last_reuse = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        repo.update_last_resue("3", last_reuse).unwrap();
    }

    #[test]
    fn find_by_value_happy_path() {
        let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();

        let rtf = copy_item_rtf("1", "aa1", t, t);
        let html = copy_item_html("2", "aa1", t, t);
        let image = copy_item_image("3", "aa1", t, t);
        let files = copy_item_files("4", "aa1", t, t);
        let text = copy_item_text("5", "aa1", t, t);
        let unknown = copy_item_unknown("6", t, t);
        
        let repo = InMemoryRepository::new(vec![rtf.clone(), html.clone(), image.clone(), files.clone(), text.clone(), unknown.clone()]);

        assert_eq!(repo.find_by_value(&rtf.value).unwrap().unwrap(), rtf);
        assert_eq!(repo.find_by_value(&html.value).unwrap().unwrap(), html);
        assert_eq!(repo.find_by_value(&image.value).unwrap().unwrap(), image);
        assert_eq!(repo.find_by_value(&files.value).unwrap().unwrap(), files);
        assert_eq!(repo.find_by_value(&text.value).unwrap().unwrap(), text);
        assert_eq!(repo.find_by_value(&unknown.value).unwrap().unwrap(), unknown);
        assert!(repo.find_by_value(&copy_item_rtf("1", "b", t, t).value).unwrap().is_none());
        assert!(repo.find_by_value(&copy_item_html("2", "b", t, t).value).unwrap().is_none());
        assert!(repo.find_by_value(&copy_item_image("3", "b", t, t).value).unwrap().is_none());
        assert!(repo.find_by_value(&copy_item_files("4", "b", t, t).value).unwrap().is_none());
        assert!(repo.find_by_value(&copy_item_text("5", "b", t, t).value).unwrap().is_none());
    }

    #[test]
    fn remove_happy_path() {
        let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();

        let item1 = copy_item_text("1", "asda", t, t);
        let item2 = copy_item_files("2", "asda", t, t);
        let item3 = copy_item_files("3", "asda", t, t);

        let repo = InMemoryRepository::new(vec![item1.clone(), item2.clone(), item3.clone()]);

        repo.remove(&vec!["1", "3"]).unwrap();

        let items = repo.items.lock().unwrap().clone();
        assert_eq!(items, vec![item2]);
    }

    #[test]
    fn remove_has_no_effect() {
        let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();

        let item1 = copy_item_text("1", "asda", t, t);
        let item2 = copy_item_files("2", "asda", t, t);

        let repo = InMemoryRepository::new(vec![item1.clone(), item2.clone()]);

        repo.remove(&vec!["4", "3"]).unwrap();

        let items = repo.items.lock().unwrap().clone();
        assert_eq!(items, vec![item1, item2]);
    }

    #[test]
    fn remove_older_happy_path() {
        let t1 = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        let t2 = Utc.with_ymd_and_hms(2001, 1, 1, 0, 0, 0).unwrap();
        let t3 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();

        let item1 = copy_item_text("1", "asda", t1, t1);
        let item2 = copy_item_files("2", "asda", t2, t2);
        let item3 = copy_item_files("3", "asda", t3, t3);

        let repo = InMemoryRepository::new(vec![item1.clone(), item2.clone(), item3.clone()]);

        let sinse = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();

        repo.remove_older(sinse).unwrap();

        let items = repo.items.lock().unwrap().clone();
        assert_eq!(items, vec![item3]);
    }

    #[test]
    fn remove_older_has_no_effect() {
        let t1 = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
        let t2 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();

        let item1 = copy_item_text("1", "asda", t1, t1);
        let item2 = copy_item_files("2", "asda", t2, t2);

        let repo = InMemoryRepository::new(vec![item1.clone(), item2.clone()]);

        let sinse = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();

        repo.remove_older(sinse).unwrap();

        let items = repo.items.lock().unwrap().clone();
        assert_eq!(items, vec![item1, item2]);
    }
}