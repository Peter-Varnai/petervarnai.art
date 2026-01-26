use crate::{error::ApiError, models::Project};
use actix_multipart::Multipart;
use futures_util::TryStreamExt;
use nanoid::nanoid;
use rusqlite::Error as RusqliteError;
use rusqlite::{params, Connection};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

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

pub async fn handle_project_form(
    edit: bool,
    mut payload: Multipart,
    root_dir: &PathBuf,
) -> Result<Project, ApiError> {
    let mut id: u16 = 000;
    let mut title = String::new();
    let mut date = String::new();
    let mut video_link = None;
    let mut concept = String::new();
    let mut medium = None;
    let mut duration = None;
    let mut dir = nanoid!(6);
    let mut saved_files = Vec::new();

    let folder_name = &root_dir.join(format!("templates/static/images/{}", dir));

    if !edit {
        fs::create_dir(folder_name)?;
    }

    while let Some(item) = payload.try_next().await? {
        let mut field = item;
        let content_disposition = field
            .content_disposition()
            .expect("No field content disposition!");

        if let Some(name) = content_disposition.get_name() {
            match name {
                "title" => {
                    let mut value = Vec::new();
                    while let Some(chunk) = field.try_next().await? {
                        value.extend_from_slice(&chunk);
                    }
                    title = String::from_utf8_lossy(&value).into_owned();
                    println!("title:  {}", title);
                }
                "dir" => {
                    let mut value = Vec::new();
                    while let Some(chunk) = field.try_next().await? {
                        value.extend_from_slice(&chunk);
                    }
                    dir = String::from_utf8(value).unwrap();
                    println!("dir:  {}", dir);
                }
                "id" => {
                    let mut value = Vec::new();
                    while let Some(chunk) = field.try_next().await? {
                        value.extend_from_slice(&chunk);
                    }
                    id = String::from_utf8(value).unwrap().parse::<u16>().unwrap();
                    println!("id:  {}", id);
                }
                "medium" => {
                    let mut value = Vec::new();
                    while let Some(chunk) = field.try_next().await? {
                        value.extend_from_slice(&chunk);
                    }
                    medium = Some(String::from_utf8_lossy(&value).into_owned());
                    println!("medium: {:?}", medium);
                }
                "duration" => {
                    let mut value = Vec::new();
                    while let Some(chunk) = field.try_next().await? {
                        value.extend_from_slice(&chunk);
                    }
                    duration = Some(String::from_utf8(value).unwrap());
                    println!("duration:  {:?}", duration);
                }
                "date" => {
                    let mut value = Vec::new();
                    while let Some(chunk) = field.try_next().await? {
                        value.extend_from_slice(&chunk);
                    }
                    date = String::from_utf8(value).unwrap();
                    println!("date:  {}", date);
                }
                "video_link" => {
                    let mut value = Vec::new();
                    while let Some(chunk) = field.try_next().await? {
                        value.extend_from_slice(&chunk);
                    }
                    video_link = Some(String::from_utf8(value).unwrap());
                    println!("video link:  {:?}", video_link);
                }
                "concept" => {
                    let mut value = Vec::new();
                    while let Some(chunk) = field.try_next().await? {
                        value.extend_from_slice(&chunk);
                    }
                    concept = String::from_utf8_lossy(&value).into_owned();
                    println!("concept:  {:?}", concept);
                }
                "files" if !edit => {
                    let filename =
                        String::from(content_disposition.get_filename().unwrap_or("pic"));
                    let filepath = folder_name.join(&filename);
                    let mut f = fs::File::create(filepath)?;
                    while let Some(chunk) = field.try_next().await? {
                        f.write_all(&chunk)?;
                    }
                    println!("filename:  {}", filename);
                    saved_files.push(filename)
                }
                _ => {
                    println!("Other type of fieldname submitted!");
                }
            }
        }
    }

    println!(
        "id: {:?}\n 
        dir: {:?}\n 
        title: {:?}\n 
        video_link: {:?}\n 
        concept: {:?}\n
        medium: {:?}\n
        duration: {:?}\n
        date: {:?}\n
        saved files: {:?}\n",
        id, dir, title, video_link, concept, medium, duration, date, saved_files
    );

    Ok(Project {
        id,
        dir,
        title,
        video_link,
        concept,
        medium,
        duration,
        date,
        saved_files,
    })
}

pub fn resolve_filename_collision(dir: &Path, filename: &str) -> PathBuf {
    let path = Path::new(filename);

    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");

    let ext = path.extension().and_then(|e| e.to_str());

    let mut candidate = match ext {
        Some(ext) => dir.join(format!("{stem}.{ext}")),
        None => dir.join(stem),
    };

    let mut counter = 1;
    while candidate.exists() {
        candidate = match ext {
            Some(ext) => dir.join(format!("{stem}_{counter}.{ext}")),
            None => dir.join(format!("{stem}_{counter}")),
        };
        counter += 1;
    }

    candidate
}

pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '.' || *c == '_' || *c == '-')
        .collect()
}
