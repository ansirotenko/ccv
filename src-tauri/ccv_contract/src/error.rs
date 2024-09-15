use serde::Serialize;
use std::fmt::{self, Display};

#[derive(Serialize, Clone, Debug)]
pub struct AppError {
    #[serde(rename = "message")]
    pub message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

#[macro_export]
macro_rules! app_error {
    ($($args: tt)*) => {
        AppError{ message: format!($($args)*)}
    }
}

pub fn log_error<TOk, TErr>(value: Result<TOk, TErr>, error_message: &str) -> Result<TOk, AppError> 
where TErr : Display
{
    match value {
        Err(err) => {
            let app_error = app_error!("{error_message}. {err}");
            log::error!("{app_error}");
            Err(app_error)
        }
        Ok(res) => {
            Ok(res)
        }
    }
}