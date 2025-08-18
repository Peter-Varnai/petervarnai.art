use actix_identity::Identity;
use actix_web::{delete, get, post, put, web::{self, Data, Form, Json, Query}, HttpResponse, Responder};
use rusqlite::{params, Connection};
use time::format_description;
use actix_multipart::Multipart;
use futures_util::TryStreamExt;
use nanoid::nanoid;
use crate::{
    error::ApiError, 
    models::{AppState, ExhibitionForm, Id, Project}    
};
use tera::Context;
use std::fs::File;
use std::io::Write;

pub fn admin_service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(add_exhibition)
        ;
}

//   PROJECT ROUTES
//
// CREATE TABLE projects (
// id INTEGER PRIMARY KEY,
// title TEXT,
// pictures TEXT,
// video TEXT,
// concept TEXT,
// collaborators TEXT,
// medium TEXT,
// duration TEXT,
// release TEXT );

#[post("/project")]
async fn add_project(
    state: Data<AppState>,
    mut payload: Multipart,
    identity: Option<Identity>,
) -> Result<HttpResponse, ApiError> {
    let tera = &state.tera;

    if identity.is_none() {
        let context = Context::new();
        let login = tera.render("login.html", &context)?;

        Ok(HttpResponse::Ok().content_type("text/html").body(login))

    } else {
        let mut title = None;
        let mut date = None;
        let mut video_link = None;
        let mut text = None;
        let mut medium = None;
        let mut duration = None;
        let mut saved_files = Vec::new();

        while let Some(item) = payload.try_next().await? {
            let mut field = item;
            let content_disposition = field.content_disposition().expect("No field content disposition!");

            if let Some(name) = content_disposition.get_name() {
                match name {
                    "title" => {
                        let mut value = Vec::new();
                        while let Some(chunk) = field.try_next().await? {
                            value.extend_from_slice(&chunk);
                        }
                        title = Some(String::from_utf8(value).unwrap());
                    }
                    "medium" => {
                        let mut value = Vec::new();
                        while let Some(chunk) = field.try_next().await? {
                            value.extend_from_slice(&chunk);
                        }
                        medium = Some(String::from_utf8(value).unwrap());
                    }
                    "duration" => {
                        let mut value = Vec::new();
                        while let Some(chunk) = field.try_next().await? {
                            value.extend_from_slice(&chunk);
                        }
                        duration = Some(String::from_utf8(value).unwrap());
                    }
                    "date" => {
                        let mut value = Vec::new();
                        while let Some(chunk) = field.try_next().await? {
                            value.extend_from_slice(&chunk);
                        }
                        date = Some(String::from_utf8(value).unwrap());
                    }
                    "video_link" => {
                        let mut value = Vec::new();
                        while let Some(chunk) = field.try_next().await? {
                            value.extend_from_slice(&chunk);
                        }
                        video_link = Some(String::from_utf8(value).unwrap());
                    }
                    "concept" => {
                        let mut value = Vec::new();
                        while let Some(chunk) = field.try_next().await? {
                            value.extend_from_slice(&chunk);
                        }
                        text = Some(String::from_utf8(value).unwrap());
                    }
                    "files" => {
                        let filename = format!(
                            "{}_{}", nanoid!(6), content_disposition.get_filename().unwrap_or("pic"));
                        let filepath = &state.root_dir.join("uploads").join(&filename);
                        let mut f = File::create(filepath)?;
                        while let Some(chunk) = field.try_next().await? {
                            f.write_all(&chunk)?;
                        }
                        saved_files.push(filename);
                    }
                    _ => {println!("Other type of fieldname submitted!");}
                }
            }
        }

        let db_path = &state.db;
        let conn = Connection::open(db_path)?;

        conn.execute(
            "INSERT INTO projects 
            (title, concept, medium, duration, release, video, pictures)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                &title,
                &text,
                &medium,
                &duration,
                &date,
                &video_link,
                &saved_files.join(", "),
            ])?;


        Ok(HttpResponse::Ok().json(serde_json::json!({
            "title": title,
            "date": date,
            "video_link": video_link,
            "text": text,
            "files": saved_files,
        })))
    }  
}


#[put("/project")]
async fn update_project(
    state: Data<AppState>,
    project: Multipart,
    identity: Option<Identity>,
) -> Result<HttpResponse, ApiError> {
    let tera = &state.tera;

    if identity.is_none() {
        let context = Context::new();
        let login = tera.render("login.html", &context)?;

        Ok(HttpResponse::Ok().content_type("text/html").body(login))

    } else {

    }
}




#[delete("/project")]
async fn delete_project(
    state: Data<AppState>,
    id: Json<Id>,
    identity: Option<Identity>,
) -> Result<HttpResponse, ApiError> {
    let tera = &state.tera;

    if identity.is_none() {
        let context = Context::new();
        let login = tera.render("login.html", &context)?;

        Ok(HttpResponse::Ok().content_type("text/html").body(login))

    } else {
        let db_path = &state.db;
        let conn = Connection::open(db_path)?;

        conn.execute(
            "DELETE FROM projects WHERE id = ?1",
            params![id.into_inner().0])?;

        Ok(HttpResponse::Ok().json("Project deleted succesfully!"))

    }
}


// EXHIBITION HANDLERS
#[post("/exhibition")]
async fn add_exhibition(
    state: Data<AppState>,
    form: Json<ExhibitionForm>,
    identity: Option<Identity>,
) -> Result<HttpResponse, ApiError> {
    let tera = &state.tera;

    if identity.is_none() {
        let context = Context::new();
        let login = tera.render("login.html", &context)?;

        Ok(HttpResponse::Ok().content_type("text/html").body(login))

    } else {
        let db_path = &state.db;
        let conn = Connection::open(db_path)?;

        let format = format_description::parse("[year]-[month]-[day]")
            .expect("Failed to validate date format");

        conn.execute(
            "INSERT INTO exhibitions (name, from, till, location, link) 
            VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
               form.name,
                form.start_date.format(&format)?,
                form.till.format(&format)?,
                form.location,
                form.link,
        ])?;
        
        Ok(HttpResponse::Ok().json("Exhibition added succesfully!"))
    }
}


#[delete("/exhibition")]
async fn delete_exhibition(
    state: Data<AppState>,
    id: Json<Id>,
    identity: Option<Identity>,
) -> Result<HttpResponse, ApiError> {
    
    let tera = &state.tera;

    if identity.is_none() {
        let context = Context::new();
        let login = tera.render("login.html", &context)?;

        Ok(HttpResponse::Ok().content_type("text/html").body(login))

    } else {
        let db_path = &state.db;
        let conn = Connection::open(db_path)?;
    
        conn.execute(
            "DELETE FROM exhibitions WHERE id = ?1",
            params![id.into_inner().0])?;

        Ok(HttpResponse::Ok().json("Exhibition deleted succesfully!"))
    }
}
