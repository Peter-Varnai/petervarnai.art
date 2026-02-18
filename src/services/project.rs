use crate::{
    error::AppError,
    models::{
        AppState, DeleteProjectRequest, PicDeleteRequest, PicDeleteResponse, PicUpdateResponse,
        Project,
    },
    services::files::{handle_project_form, resolve_filename_collision, sanitize_filename},
};

use actix_multipart::Multipart;
use futures_util::StreamExt;
use rusqlite::{params, Connection};
use std::{fs, io::Write, path::PathBuf};

pub fn fetch_project(conn: &Connection, id: u16) -> Result<Project, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT title, pictures, video, concept, dir, medium, duration, release, id \
         FROM projects WHERE id = ?1",
    )?;

    stmt.query_row(params![id], |row| {
        let pictures_json: String = row.get(1)?;
        let saved_files: Vec<String> = serde_json::from_str(&pictures_json).unwrap_or_default();

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
    })
}

pub fn get_project_by_id(db_path: &PathBuf, id: u16) -> Result<Project, rusqlite::Error> {
    let conn = Connection::open(db_path)?;
    fetch_project(&conn, id)
}

pub async fn create_project(state: &AppState, payload: Multipart) -> Result<(), AppError> {
    let Project {
        title,
        concept,
        medium,
        duration,
        date,
        video_link,
        saved_files,
        id: _,
        dir,
    } = handle_project_form(false, payload, &state.root_dir).await?;

    let conn = Connection::open(&state.db)?;
    conn.execute(
        "INSERT INTO projects (title, concept, medium, duration, release, video, pictures, dir)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            &title,
            &concept,
            &medium,
            &duration,
            &date,
            &video_link,
            serde_json::to_string(&saved_files)?,
            dir
        ],
    )?;

    Ok(())
}

pub async fn update_project(state: &AppState, payload: Multipart) -> Result<(), AppError> {
    let Project {
        title,
        concept,
        medium,
        duration,
        date,
        video_link,
        saved_files: _,
        id,
        dir: _,
    } = handle_project_form(true, payload, &state.root_dir).await?;

    let conn = Connection::open(&state.db)?;
    conn.execute(
        "UPDATE projects
             SET title = ?1,
                 concept = ?2,
                 medium = ?3,
                 duration = ?4,
                 release = ?5,
                 video = ?6
           WHERE id = ?7",
        params![&title, &concept, &medium, &duration, &date, &video_link, id],
    )?;

    Ok(())
}

pub fn delete_project(state: &AppState, req: &DeleteProjectRequest) -> Result<(), AppError> {
    let conn = Connection::open(&state.db)?;
    conn.execute("DELETE FROM projects WHERE id = ?1", params![req.id])?;
    Ok(())
}

pub async fn upload_project_images(
    state: &AppState,
    project_dir: String,
    mut payload: Multipart,
) -> Result<PicUpdateResponse, AppError> {
    // DB: fetch project info
    let (db_dir, mut saved_files): (String, Vec<String>) = {
        let conn = Connection::open(&state.db)?;
        let mut stmt = conn.prepare("SELECT dir, pictures FROM projects WHERE dir = ?1")?;

        stmt.query_row([&project_dir], |row| {
            let dir: String = row.get(0)?;
            let pictures: String = row.get(1)?;
            let parsed: Vec<String> = serde_json::from_str(&pictures).unwrap_or_default();
            Ok((dir, parsed))
        })?
    };

    // Resolve filesystem path
    let images_dir: PathBuf = state
        .root_dir
        .join("templates")
        .join("static")
        .join("images")
        .join(&db_dir);

    fs::create_dir_all(&images_dir)?;

    let mut newly_added = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = item?;

        if field.name() != Some("images") {
            continue;
        }

        let content_disposition = field.content_disposition().unwrap();
        let filename = content_disposition.get_filename().expect("File save failed");

        let sanitized = sanitize_filename(filename);
        let target_path = resolve_filename_collision(&images_dir, &sanitized);

        let final_name = target_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap()
            .to_string();

        let mut f = fs::File::create(&target_path)?;
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f.write_all(&data)?;
        }

        newly_added.push(final_name);
    }

    saved_files.extend(newly_added);
    let pictures_json = serde_json::to_string(&saved_files)?;

    {
        let conn = Connection::open(&state.db)?;
        conn.execute(
            "UPDATE projects SET pictures = ?1 WHERE dir = ?2",
            (&pictures_json, &db_dir),
        )?;
    }

    Ok(PicUpdateResponse { saved_files })
}

pub fn delete_project_image(
    state: &AppState,
    dir: String,
    body: &PicDeleteRequest,
) -> Result<PicDeleteResponse, AppError> {
    let filename = &body.filename;

    let (db_dir, pictures): (String, String) = {
        let conn = Connection::open(&state.db)?;
        let mut stmt = conn.prepare("SELECT dir, pictures FROM projects WHERE dir = ?1")?;

        stmt.query_row([&dir], |row| Ok((row.get(0)?, row.get(1)?)))?
    };

    let mut saved_files: Vec<String> = serde_json::from_str(&pictures)?;

    if !saved_files.contains(filename) {
        return Err(AppError::FileSystem(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Image not found in project",
        )));
    }

    let image_path = state
        .root_dir
        .join("templates")
        .join("static")
        .join("images")
        .join(&db_dir)
        .join(filename);

    fs::remove_file(&image_path)?;

    saved_files.retain(|f| f != filename);
    let pictures_json = serde_json::to_string(&saved_files)?;

    {
        let conn = Connection::open(&state.db)?;
        conn.execute(
            "UPDATE projects SET pictures = ?1 WHERE dir = ?2",
            (&pictures_json, &db_dir),
        )?;
    }

    Ok(PicDeleteResponse { saved_files })
}
