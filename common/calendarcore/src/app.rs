use crate::db::db_init;
use once_cell::sync::OnceCell;
use rusqlite::Connection;
use std::sync::Arc;

use super::error::AppError;

pub static APP_CONFIG: OnceCell<Arc<AppConfig>> = OnceCell::new();

#[uniffi::export]
pub fn lib_init(db_path: String) -> Result<(), AppError> {
    let app = Arc::new(AppConfig::new(db_path));
    db_init(&app)?;
    let _ = APP_CONFIG.set(app);
    Ok(())
}

#[derive(uniffi::Object)]
pub struct AppConfig {
    pub(crate) db_path: String,
}

impl AppConfig {
    #[uniffi::constructor]
    pub fn new(db_path: String) -> Self {
        AppConfig { db_path }
    }
}

pub fn get_app_config() -> Result<Arc<AppConfig>, AppError> {
    Ok(APP_CONFIG
        .get()
        .ok_or(AppError::LibraryUninitialized)?
        .clone())
}
