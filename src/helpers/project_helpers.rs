use crate::models::Project;
use rusqlite::Error as RusqliteError;
use rusqlite::{params, Connection};

pub async fn return_project(conn: Connection, id: &u16) -> Result<Project, RusqliteError> {
    let mut proj_list_stmt = conn.prepare(
        "SELECT title, pictures, video, concept, 
        dir, medium, duration, release, id FROM projects WHERE id = ?1",
    )?;
    return Ok(proj_list_stmt.query_row(params![id], |row| {
        let pictures_json: String = row.get(1)?;
        let saved_files: Vec<String> =
            serde_json::from_str(&pictures_json).expect("Failed to parse pictures JSON");

        Ok(Project {
            title: row.get(0)?,
            saved_files,
            video_link: row.get(2)?,
            concept: row.get(3)?,
            dir: row.get(4)?,
            medium: row.get(5)?,
            duration: row.get(6)?,
            date: row.get(7)?,
            id: row.get(8)?,
        })
    })?);
}
