use crate::{
    error::ApiError,
    helpers::return_project,
    models::{AppState, DeleteProject, Project, ProjectNo},
};

use actix_identity::Identity;
use actix_multipart::Multipart;
use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Query},
    HttpResponse,
};
use futures_util::TryStreamExt;
use nanoid::nanoid;
use rusqlite::{params, Connection};
use std::{fs, io::Write, path::PathBuf};
use tera::Context;

pub fn project_service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(add_project)
        .service(update_project)
        .service(get_project)
        .service(delete_project);
}

#[get("/project")]
async fn get_project(
    state: Data<AppState>,
    project: Query<ProjectNo>,
) -> Result<HttpResponse, ApiError> {
    let tera = &state.tera;
    let project_id: u16 = project.no;
    println!("getting project with id: {}", project_id);

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

    // if identity.is_none() {
    if false {
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
        } = handle_project_form(payload, &state.root_dir).await?;

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

    // if identity.is_none() {
    if false {
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
            id,
            dir: _,
        } = handle_project_form(payload, &state.root_dir).await?;

        let db_path = &state.db;
        let conn = Connection::open(db_path)?;

        conn.execute(
            "UPDATE projects
            (title, concept, medium, duration, release, video, pictures)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            WHERE id = ?8",
            params![
                &title,
                &concept,
                &medium,
                &duration,
                &date,
                &video_link,
                serde_json::to_string(&saved_files)?,
                id
            ],
        )?;

        Ok(HttpResponse::Found()
            .append_header(("Location", "/admin"))
            .finish())
    }
}

#[delete("/project")]
async fn delete_project(
    state: Data<AppState>,
    delete_project: Json<DeleteProject>,
    identity: Option<Identity>,
) -> Result<HttpResponse, ApiError> {
    let tera = &state.tera;
    println!("delete project called");

    // if identity.is_none() {
    if false {
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

        Ok(HttpResponse::Ok().json("Project deleted succesfully!"))
    }
}

async fn handle_project_form(
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
    let mut saved_files = Vec::new();
    let mut dir = nanoid!(6);

    let folder_name = &root_dir.join(format!("templates/static/images/{}", dir));
    fs::create_dir(folder_name)?;

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
                    println!("titile:  {}", title);
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
                "files" => {
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
