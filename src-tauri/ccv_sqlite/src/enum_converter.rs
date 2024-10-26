use ccv_contract::models::CopyCategory::{self, Files, Html, Image, Rtf, Text, Unknown};

pub fn to_copy_category(enum_string: &str) -> CopyCategory {
    match enum_string {
        "Files" => Files,
        "Html" => Html,
        "Image" => Image,
        "Rtf" => Rtf,
        "Text" => Text,
        "Unknown" => Unknown,
        _ => Unknown,
    }
}

pub fn from_copy_category(copy_category: CopyCategory) -> &'static str {
    match copy_category {
        Files => "Files",
        Html => "Html",
        Image => "Image",
        Rtf => "Rtf",
        Text => "Text",
        Unknown => "Unknown",
    }
}
