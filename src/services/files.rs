use crate::{error::AppError, models::Project};
use actix_multipart::Multipart;
use futures_util::TryStreamExt;
use nanoid::nanoid;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

/// Helper to extract a text field from multipart form data
async fn extract_text_field(field: &mut actix_multipart::Field) -> Result<String, AppError> {
    let mut value = Vec::new();
    while let Some(chunk) = field.try_next().await? {
        value.extend_from_slice(&chunk);
    }
    Ok(String::from_utf8_lossy(&value).into_owned())
}

/// Helper to extract an optional text field (returns None if empty)
async fn extract_optional_field(
    field: &mut actix_multipart::Field,
) -> Result<Option<String>, AppError> {
    let text = extract_text_field(field).await?;
    Ok(if text.is_empty() { None } else { Some(text) })
}

pub async fn handle_project_form(
    edit: bool,
    mut payload: Multipart,
    root_dir: &PathBuf,
) -> Result<Project, AppError> {
    let mut id: u16 = 0;
    let mut title = String::new();
    let mut date = String::new();
    let mut video_link = None;
    let mut concept = String::new();
    let mut medium = None;
    let mut duration = None;
    let mut dir = nanoid!(6);
    let mut saved_files = Vec::new();

    let folder_name = &root_dir.join(format!("templates/static/images/{dir}"));

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
                "title" => title = extract_text_field(&mut field).await?,
                "dir" => dir = extract_text_field(&mut field).await?,
                "id" => {
                    id = extract_text_field(&mut field)
                        .await?
                        .parse::<u16>()
                        .expect("Invalid id format");
                }
                "medium" => medium = extract_optional_field(&mut field).await?,
                "duration" => duration = extract_optional_field(&mut field).await?,
                "date" => date = extract_text_field(&mut field).await?,
                "video_link" => video_link = extract_optional_field(&mut field).await?,
                "concept" => concept = extract_text_field(&mut field).await?,
                "files" if !edit => {
                    let filename =
                        String::from(content_disposition.get_filename().unwrap_or("pic"));
                    let filepath = folder_name.join(&filename);
                    let mut f = fs::File::create(filepath)?;
                    while let Some(chunk) = field.try_next().await? {
                        f.write_all(&chunk)?;
                    }
                    saved_files.push(filename)
                }
                _ => {}
            }
        }
    }

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
