use serde::Serialize;

#[derive(Serialize)]
pub struct CopyItem {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "displayName")]
    display_name: String,
}

#[tauri::command]
pub fn search_copy_items(query: String) -> Vec<CopyItem> {
    let mut ret = vec![]; 
    ret.push(CopyItem{id: "it1".to_string(), display_name: format!("{query} 1")});
    ret.push(CopyItem{id: "it2".to_string(), display_name: format!("{query} 2")});
    ret.push(CopyItem{id: "it3".to_string(), display_name: format!("{query} 3")});
    ret.push(CopyItem{id: "it4".to_string(), display_name: format!("{query} 4")});
    ret.push(CopyItem{id: "it5".to_string(), display_name: format!("{query} 5")});
    return ret;
}

#[tauri::command]
pub fn hide_window(window: tauri::Window) {
    window.hide().unwrap();
}

#[tauri::command]
pub fn show_window(window: tauri::Window) {
    window.show().unwrap();
    window.set_focus().unwrap();
}


