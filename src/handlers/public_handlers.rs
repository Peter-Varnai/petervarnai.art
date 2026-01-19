use crate::{
    error::AppError,
    helpers::return_project,
    models::{
        AppState, DeleteExhibition, DeleteProject, EditProjectListItem, Exhibition, LoginForm,
        Project, ProjectList, ProjectNo,
    },
};
use actix_identity::Identity;
use actix_web::{
    get, post,
    web::{self, Data, Query},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use rusqlite::Connection;
use std::collections::HashMap;
use tera::Context;

pub fn public_service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(project)
        .service(admin)
        .service(login)
        .service(logout);
}

#[get("/")]
async fn index(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let tera = &state.tera;

    let db_path = &state.db;
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare("SELECT id, title FROM projects")?;
    let projects = stmt
        .query_map([], |row| {
            Ok(ProjectList {
                id: row.get(0)?,
                title: row.get(1)?,
            })
        })?
        .collect::<Result<Vec<_>, rusqlite::Error>>()?;

    let mut exhib_stmt = conn
        .prepare("SELECT id, name, location, link, big_row, start_date, till FROM exhibitions")?;

    let exhibitions = exhib_stmt
        .query_map([], |row| {
            Ok(Exhibition {
                id: row.get(0)?,
                title: row.get(1)?,
                location: row.get(2)?,
                link: row.get(3)?,
                big_row: row.get(4)?,
                start_date: row.get(5)?,
                till: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    let mut exhibitions_by_year: HashMap<String, Vec<Exhibition>> = HashMap::new();

    for e in exhibitions {
        let year = e.start_date.chars().take(4).collect::<String>();

        exhibitions_by_year.entry(year).or_default().push(e);
    }

    let mut years: Vec<_> = exhibitions_by_year.keys().cloned().collect();
    years.sort();
    years.reverse();

    let mut context = Context::new();
    context.insert("projects_list", &projects);
    context.insert("exhibitions_table", &exhibitions_by_year);
    context.insert("years", &years);

    let rendered = tera
        .render("index.html", &context)
        .expect("error rendering index template");

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

#[get("/prj")]
async fn project(
    state: Data<AppState>,
    project: Query<ProjectNo>,
) -> Result<HttpResponse, AppError> {
    let tera = &state.tera;
    let project_id: u16 = project.no;
    println!("getting project with id: {}", project_id);

    let db_path = &state.db;
    let conn = Connection::open(db_path)?;
    let project = return_project(conn, &project_id).await?;
    // let mut proj_list_stmt = conn.prepare(
    //     "SELECT title, pictures, video, concept,
    //     dir, medium, duration, release, id FROM projects WHERE id = ?1",
    // )?;
    // let project = proj_list_stmt.query_row(params![project_id], |row| {
    //     let pictures_json: String = row.get(1)?;
    //     let saved_files: Vec<String> =
    //         serde_json::from_str(&pictures_json).expect("Failed to parse pictures JSON");
    //
    //     Ok(Project {
    //         title: row.get(0)?,
    //         saved_files,
    //         video_link: row.get(2)?,
    //         concept: row.get(3)?,
    //         dir: row.get(4)?,
    //         medium: row.get(5)?,
    //         duration: row.get(6)?,
    //         date: row.get(7)?,
    //         id: row.get(8)?,
    //     })
    // })?;

    let mut context = Context::new();
    context.insert("project", &project);

    let rendered = tera
        .render("project.html", &context)
        .expect("error rendering index template");

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

#[get("/admin")]
async fn admin(
    identity: Option<Identity>,
    state: Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let tera = &state.tera;

    if false {
        // if identity.is_none() {
        let context = Context::new();
        let login_template = tera.render("login.html", &context)?;

        Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body(login_template))
    } else {
        let db_path = &state.db;
        let conn = Connection::open(db_path)?;

        let mut stmt_edit_project_list = conn.prepare("SELECT id, title FROM projects")?;
        let edit_project_list = stmt_edit_project_list
            .query_map([], |row| {
                Ok(EditProjectListItem {
                    project_id: row.get(0)?,
                    project_title: row.get(1)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut stmt_edit_project =
            conn.prepare("SELECT * FROM projects ORDER BY id DESC LIMIT 1")?;
        let editProject: Project = stmt_edit_project.query_row([], |row| {
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

        let mut stmt_exhibitions = conn.prepare("SELECT id, name, start_date FROM exhibitions")?;
        let delete_exhibitions: Vec<DeleteExhibition> = stmt_exhibitions
            .query_map([], |row| {
                Ok(DeleteExhibition {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    start_date: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut delete_exhibitions_by_year: HashMap<String, Vec<DeleteExhibition>> = HashMap::new();

        for e in &delete_exhibitions {
            let year = e.start_date.chars().take(4).collect::<String>();

            delete_exhibitions_by_year
                .entry(year)
                .or_default()
                .push(e.clone());
        }

        let mut years: Vec<_> = delete_exhibitions_by_year.keys().cloned().collect();
        years.sort();
        years.reverse();

        let delete_projects = conn
            .prepare("SELECT id, dir, title FROM projects")?
            .query_map([], |row| {
                Ok(DeleteProject {
                    id: row.get(0)?,
                    folder_path: row.get(1)?,
                    name: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<DeleteProject>, _>>()?;

        let mut context = Context::new();
        context.insert("edit_project", &editProject);
        context.insert("edit_project_list", &edit_project_list);
        context.insert("delete_exhibitions", &delete_exhibitions_by_year);
        context.insert("delete_exhibition_years", &years);
        context.insert("delete_projects", &delete_projects);
        let html = tera.render("admin/admin.html", &context)?;

        Ok(HttpResponse::Ok().body(html))
    }
}

#[post("/login")]
async fn login(
    state: web::Data<AppState>,
    form: web::Form<LoginForm>,
    request: HttpRequest,
) -> impl Responder {
    println!("login called");
    if form.password == state.pwd {
        Identity::login(&request.extensions(), "admin".into()).expect("Failed to log in");
        HttpResponse::Found()
            .append_header(("Location", "/admin"))
            .finish()
    } else {
        HttpResponse::Unauthorized().body("Invalid credentials")
    }
}

#[post("/logout")]
async fn logout(user: Identity) -> impl Responder {
    user.logout();
    println!("Log out called");
    HttpResponse::Ok().body("Logged out!")
}
