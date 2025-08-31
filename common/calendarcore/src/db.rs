use crate::{
    app::{APP_CONFIG, AppConfig, get_app_config},
    error::AppError,
};
use rusqlite::{Connection, Result};

pub fn db_init(app: &AppConfig) -> Result<(), AppError> {
    let conn = Connection::open(&app.db_path).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS calendars (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT
        )",
        [],
    )?;

    Ok(())
}

pub fn get_default_db_connection() -> Result<Connection, AppError> {
    let app = get_app_config()?;
    Ok(Connection::open(&app.db_path)?)
}
