use crate::{
    error::AppError,
    models::{
        DeleteExhibitionAdminTemp, DeleteProjectAdminTemp, EditProjectAdminTemp, Exhibition,
        Project, ProjectsIndexTemp,
    },
};
use rusqlite::{Connection, Row};
use std::path::PathBuf;

/// Generic helper for executing list queries
fn query_list<T, F>(db_path: &PathBuf, sql: &str, mapper: F) -> Result<Vec<T>, AppError>
where
    F: Fn(&Row) -> Result<T, rusqlite::Error>,
{
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare(sql)?;
    let results = stmt.query_map([], mapper)?.collect::<Result<Vec<_>, _>>()?;
    Ok(results)
}

pub fn list_projects_index(db_path: &PathBuf) -> Result<Vec<ProjectsIndexTemp>, AppError> {
    query_list(db_path, "SELECT id, title FROM projects", |row| {
        Ok(ProjectsIndexTemp {
            id: row.get(0)?,
            title: row.get(1)?,
        })
    })
}

pub fn list_exhibitions(db_path: &PathBuf) -> Result<Vec<Exhibition>, AppError> {
    query_list(
        db_path,
        "SELECT id, title, location, link, type, start_date, till FROM exhibitions",
        |row| {
            Ok(Exhibition {
                id: row.get(0)?,
                title: row.get(1)?,
                location: row.get(2)?,
                link: row.get(3)?,
                r#type: row.get(4)?,
                start_date: row.get(5)?,
                till: row.get(6)?,
            })
        },
    )
}

pub fn admin_edit_project_list(db_path: &PathBuf) -> Result<Vec<EditProjectAdminTemp>, AppError> {
    query_list(db_path, "SELECT id, title FROM projects", |row| {
        Ok(EditProjectAdminTemp {
            project_id: row.get(0)?,
            project_title: row.get(1)?,
        })
    })
}

pub fn admin_latest_project(db_path: &PathBuf) -> Result<Project, AppError> {
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare("SELECT * FROM projects ORDER BY id DESC LIMIT 1")?;
    let project: Project = stmt.query_row([], |row| {
        Ok(Project {
            id: row.get("id")?,
            title: row.get("title")?,
            date: row.get("release")?,
            video_link: row.get("video")?,
            dir: row.get("dir")?,
            concept: row.get("concept")?,
            medium: row.get("medium")?,
            duration: row.get("duration")?,
            saved_files: serde_json::from_str(&row.get::<_, String>("pictures")?)
                .unwrap_or_default(),
        })
    })?;
    Ok(project)
}

pub fn admin_delete_exhibitions(
    db_path: &PathBuf,
) -> Result<Vec<DeleteExhibitionAdminTemp>, AppError> {
    query_list(
        db_path,
        "SELECT id, title, start_date FROM exhibitions",
        |row| {
            Ok(DeleteExhibitionAdminTemp {
                id: row.get(0)?,
                name: row.get(1)?,
                start_date: row.get(2)?,
            })
        },
    )
}

pub fn admin_delete_projects(db_path: &PathBuf) -> Result<Vec<DeleteProjectAdminTemp>, AppError> {
    query_list(db_path, "SELECT id, dir, title FROM projects", |row| {
        Ok(DeleteProjectAdminTemp {
            id: row.get(0)?,
            folder_path: row.get(1)?,
            name: row.get(2)?,
        })
    })
}
