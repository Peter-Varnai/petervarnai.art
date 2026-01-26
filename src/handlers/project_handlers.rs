use crate::{
    error::{api_err::ApiErrorKind, ApiError},
    helpers::{handle_project_form, resolve_filename_collision, return_project, sanitize_filename},
    models::{
        AppState, DeleteProjectAdminTemp, DeleteProjectRequest, Project, ProjectQueryRequest,
    },
};

use actix_identity::Identity;
use actix_multipart::Multipart;
use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Query},
    HttpResponse, Result,
};
use futures_util::StreamExt;
use futures_util::TryStreamExt;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf};
use tera::Context;

pub fn project_service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(add_project)
        .service(update_project)
        .service(get_project)
        .service(delete_project)
        .service(upload_project_images)
        .service(delete_project_image);
}

#[get("/project")]
async fn get_project(
    state: Data<AppState>,
    project: Query<ProjectQueryRequest>,
) -> Result<HttpResponse, ApiError> {
    let project_id: u16 = project.no;
    println!("GET : getting project with id: {}", project_id);

    let db_path = &state.db;
    let conn = Connection::open(db_path)?;
    let project = return_project(conn, &project_id).await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(project))
}

#[post("/project")]
async fn add_project(
    state: Data<AppState>,
    payload: Multipart,
    identity: Option<Identity>,
) -> Result<HttpResponse, ApiError> {
    let tera = &state.tera;

    if identity.is_none() {
        // if false {
        let context = Context::new();
        let login = tera.render("login.html", &context)?;

        Ok(HttpResponse::Ok().content_type("text/html").body(login))
    } else {
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

        let db_path = &state.db;
        let conn = Connection::open(db_path)?;

        conn.execute(
            "INSERT INTO projects 
            (title, concept, medium, duration, release, video, pictures, dir)
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

        Ok(HttpResponse::Ok().body("Success"))
    }
}

#[put("/project")]
async fn update_project(
    state: Data<AppState>,
    payload: Multipart,
    identity: Option<Identity>,
) -> Result<HttpResponse, ApiError> {
    let tera = &state.tera;

    if identity.is_none() {
        // if false {
        let context = Context::new();
        let login = tera.render("login.html", &context)?;

        Ok(HttpResponse::Ok().content_type("text/html").body(login))
    } else {
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

        let db_path = &state.db;
        let conn = Connection::open(db_path)?;
        dbg!(
            &title,
            &concept,
            &medium,
            &duration,
            &date,
            &video_link,
            &id
        );

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

        Ok(HttpResponse::Found()
            .append_header(("Location", "/admin"))
            .finish())
    }
}

#[delete("/project")]
async fn delete_project(
    state: Data<AppState>,
    delete_project: Json<DeleteProjectRequest>,
    identity: Option<Identity>,
) -> Result<HttpResponse, ApiError> {
    println!("delete project called");

    if identity.is_none() {
        // if false {
        let tera = &state.tera;
        let context = Context::new();
        let login = tera.render("login.html", &context)?;

        Ok(HttpResponse::Ok().content_type("text/html").body(login))
    } else {
        let db_path = &state.db;
        let conn = Connection::open(db_path)?;

        let delete_path = state.root_dir.join(&delete_project.folder_path);
        println!("deleting folder: {}", delete_path.display());

        conn.execute(
            "DELETE FROM projects WHERE id = ?1",
            params![delete_project.id],
        )?;

        let delete_projects = conn
            .prepare("SELECT id, dir, title FROM projects")?
            .query_map([], |row| {
                Ok(DeleteProjectAdminTemp {
                    id: row.get(0)?,
                    folder_path: row.get(1)?,
                    name: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<DeleteProjectAdminTemp>, _>>()?;

        Ok(HttpResponse::Ok().json(""))
    }
}

#[derive(Serialize)]
struct PicUpdateResponse {
    saved_files: Vec<String>,
}

#[post("/projects/pic_update/{dir}/images")]
pub async fn upload_project_images(
    state: web::Data<AppState>,
    path: web::Path<String>,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiError> {
    let project_dir = path.into_inner();

    // --- DB: fetch project info ---
    let (db_dir, mut saved_files): (String, Vec<String>) = {
        let db_path = &state.db;
        let conn = Connection::open(db_path)?;

        let mut stmt = conn.prepare("SELECT dir, pictures FROM projects WHERE dir = ?1")?;

        stmt.query_row([&project_dir], |row| {
            let dir: String = row.get(0)?;
            let pictures: String = row.get(1)?;
            let parsed: Vec<String> = serde_json::from_str(&pictures).unwrap_or_default();
            Ok((dir, parsed))
        })?
    };

    // --- Resolve filesystem path ---
    let images_dir: PathBuf = state
        .root_dir
        .join("templates")
        .join("static")
        .join("images")
        .join(&db_dir);

    fs::create_dir_all(&images_dir)?;

    let mut newly_added = Vec::new();

    // --- Process multipart ---
    while let Some(item) = payload.next().await {
        let mut field = item?;

        if field.name() != Some("images") {
            continue;
        }

        let content_disposition = field.content_disposition().unwrap();
        let filename = content_disposition
            .get_filename()
            .expect("File save failed");

        let sanitized = sanitize_filename(filename);
        let target_path = resolve_filename_collision(&images_dir, &sanitized);

        let final_name = target_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap()
            .to_string();

        // --- Write file ---
        let mut f = fs::File::create(&target_path)?;

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f.write_all(&data)?;
        }

        newly_added.push(final_name);
    }

    // --- Update DB (fail fast: only now) ---
    saved_files.extend(newly_added.clone());
    let pictures_json = serde_json::to_string(&saved_files)?;

    {
        let db_path = &state.db;
        let conn = Connection::open(db_path)?;
        conn.execute(
            "UPDATE projects SET pictures = ?1 WHERE dir = ?2",
            (&pictures_json, &db_dir),
        )?;
    }

    Ok(HttpResponse::Ok().json(PicUpdateResponse { saved_files }))
}

#[derive(Serialize)]
struct PicDeleteResponse {
    saved_files: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PicDeleteRequest {
    pub filename: String,
}

#[delete("/projects/pic_update/{dir}/images")]
pub async fn delete_project_image(
    state: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<PicDeleteRequest>,
) -> Result<HttpResponse, ApiError> {
    let dir = path.into_inner();
    let filename = &body.filename;

    // --- Load project from DB ---
    let (db_dir, pictures): (String, String) = {
        let conn = Connection::open(&state.db)?;
        let mut stmt = conn.prepare("SELECT dir, pictures FROM projects WHERE dir = ?1")?;

        stmt.query_row([&dir], |row| Ok((row.get(0)?, row.get(1)?)))?
    };

    let mut saved_files: Vec<String> = serde_json::from_str(&pictures)?;

    // --- Ensure image exists ---
    if !saved_files.contains(filename) {
        return Err(ApiError(ApiErrorKind::FileSystemError(
            std::io::Error::new(std::io::ErrorKind::NotFound, "Image not found in project"),
        )));
    }

    // --- Resolve filesystem path ---
    let image_path = state
        .root_dir
        .join("templates")
        .join("static")
        .join("images")
        .join(&db_dir)
        .join(filename);

    // --- Delete file ---
    fs::remove_file(&image_path)?;

    // --- Update DB ---
    saved_files.retain(|f| f != filename);
    let pictures_json = serde_json::to_string(&saved_files)?;

    {
        let conn = Connection::open(&state.db)?;
        conn.execute(
            "UPDATE projects SET pictures = ?1 WHERE dir = ?2",
            (&pictures_json, &db_dir),
        )?;
    }

    Ok(HttpResponse::Ok().json(PicDeleteResponse { saved_files }))
}
