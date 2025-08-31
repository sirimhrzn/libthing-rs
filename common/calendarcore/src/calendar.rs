use crate::{db::get_default_db_connection, error::AppError};

#[uniffi::export]
pub fn create_calendar(name: String, description: String) -> Result<(), AppError> {
    let conn = get_default_db_connection()?;
    let mut stmt = conn.prepare("INSERT INTO calendars (name, description) VALUES (?, ?)")?;
    stmt.execute(&[&name, &description])?;
    Ok(())
}

#[uniffi::export]
pub fn get_calendars() -> Result<String, AppError> {
    let conn = get_default_db_connection()?;
    let mut stmt = conn.prepare("SELECT id, name, description FROM calendars order by id asc")?;

    struct Calendar {
        id: i32,
        name: String,
        data: String,
    }

    let calendar_iter = stmt
        .query_map([], |row| {
            Ok(Calendar {
                id: row.get(0)?,
                name: row.get(1)?,
                data: row.get(2)?,
            })
        })
        .unwrap();

    let calendars = calendar_iter
        .filter_map(|calendar| calendar.ok())
        .map(|calendar| calendar.name)
        .collect::<Vec<String>>()
        .join("\n");
    Ok(calendars)
}
