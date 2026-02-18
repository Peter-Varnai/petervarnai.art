use crate::{error::AppError, models::Exhibition};
use rusqlite::{params, Connection};
use std::path::PathBuf;

pub fn add_exhibition(db_path: &PathBuf, exhib: Exhibition) -> Result<(), AppError> {
    let conn = Connection::open(db_path)?;
    conn.execute(
        "INSERT INTO exhibitions (title, start_date, till, location, link, type)\
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            exhib.title,
            exhib.start_date,
            exhib.till,
            exhib.location,
            exhib.link,
            exhib.r#type,
        ],
    )?;
    Ok(())
}

pub fn delete_exhibition(db_path: &PathBuf, id: u16) -> Result<(), AppError> {
    let conn = Connection::open(db_path)?;
    conn.execute("DELETE FROM exhibitions WHERE id = ?1", params![id])?;
    Ok(())
}
