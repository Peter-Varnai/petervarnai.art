use crate::{
    error::ApiError,
    helpers::{handle_project_form, return_project},
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
use std::{fs, io::Write, path::PathBuf, process::id};
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
