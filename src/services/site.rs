use crate::{
    error::AppError,
    models::{
        DeleteExhibitionAdminTemp, DeleteProjectAdminTemp, EditProjectAdminTemp, Exhibition,
        Project, ProjectsIndexTemp,
    },
};
use rusqlite::Connection;
use std::path::PathBuf;

pub fn list_projects_index(db_path: &PathBuf) -> Result<Vec<ProjectsIndexTemp>, AppError> {
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare("SELECT id, title FROM projects")?;
    let projects = stmt
        .query_map([], |row| {
            Ok(ProjectsIndexTemp {
                id: row.get(0)?,
                title: row.get(1)?,
            })
        })?
        .collect::<Result<Vec<_>, rusqlite::Error>>()?;
    Ok(projects)
}

pub fn list_exhibitions(db_path: &PathBuf) -> Result<Vec<Exhibition>, AppError> {
    let conn = Connection::open(db_path)?;
    let mut stmt =
        conn.prepare("SELECT id, title, location, link, type, start_date, till FROM exhibitions")?;
    let exhibitions = stmt
        .query_map([], |row| {
            Ok(Exhibition {
                id: row.get(0)?,
                title: row.get(1)?,
                location: row.get(2)?,
                link: row.get(3)?,
                r#type: row.get(4)?,
                start_date: row.get(5)?,
                till: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(exhibitions)
}

pub fn admin_edit_project_list(db_path: &PathBuf) -> Result<Vec<EditProjectAdminTemp>, AppError> {
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare("SELECT id, title FROM projects")?;
    let list = stmt
        .query_map([], |row| {
            Ok(EditProjectAdminTemp {
                project_id: row.get(0)?,
                project_title: row.get(1)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(list)
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
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare("SELECT id, title, start_date FROM exhibitions")?;
    let rows: Vec<DeleteExhibitionAdminTemp> = stmt
        .query_map([], |row| {
            Ok(DeleteExhibitionAdminTemp {
                id: row.get(0)?,
                name: row.get(1)?,
                start_date: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(rows)
}

pub fn admin_delete_projects(db_path: &PathBuf) -> Result<Vec<DeleteProjectAdminTemp>, AppError> {
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare("SELECT id, dir, title FROM projects")?;
    let rows = stmt
        .query_map([], |row| {
            Ok(DeleteProjectAdminTemp {
                id: row.get(0)?,
                folder_path: row.get(1)?,
                name: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<DeleteProjectAdminTemp>, _>>()?;
    Ok(rows)
}
