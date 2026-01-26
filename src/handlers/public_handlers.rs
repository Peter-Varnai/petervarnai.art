use crate::{
    error::AppError,
    helpers::return_project,
    models::{
        AppState, DeleteExhibitionAdminTemp, DeleteProjectAdminTemp, EditProjectAdminTemp,
        Exhibition, Project, ProjectQueryRequest, ProjectsIndexTemp,
    },
};
use actix_identity::Identity;
use actix_web::{
    get,
    web::{self, Data, Query},
    HttpResponse,
};
use rusqlite::Connection;
use std::collections::HashMap;
use tera::Context;

pub fn public_service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(index).service(project).service(admin);
}

#[get("/")]
async fn index(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let tera = &state.tera;

    let db_path = &state.db;
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

// TODO: design a better url builder system where the title of the project is included as wel, not
// the project id alone!
// consider re-desigining the front page so urls lead to different projects, but selectin a project
// from the main page doesnt while changes the url, doesnt trigger a full page reload!
// the fluid uninterrupted background of the webstie is a key design elemetn!
#[get("/prj")]
async fn project(
    state: Data<AppState>,
    project: Query<ProjectQueryRequest>,
) -> Result<HttpResponse, AppError> {
    let tera = &state.tera;
    let project_id: u16 = project.no;
    println!("getting project with id: {}", project_id);

    let db_path = &state.db;
    let conn = Connection::open(db_path)?;
    let project = return_project(conn, &project_id).await?;

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

    // if false {
    if identity.is_none() {
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
                Ok(EditProjectAdminTemp {
                    project_id: row.get(0)?,
                    project_title: row.get(1)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut stmt_edit_project =
            conn.prepare("SELECT * FROM projects ORDER BY id DESC LIMIT 1")?;
        let edit_project: Project = stmt_edit_project.query_row([], |row| {
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
        let delete_exhibitions: Vec<DeleteExhibitionAdminTemp> = stmt_exhibitions
            .query_map([], |row| {
                Ok(DeleteExhibitionAdminTemp {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    start_date: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut delete_exhibitions_by_year: HashMap<String, Vec<DeleteExhibitionAdminTemp>> =
            HashMap::new();

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
                Ok(DeleteProjectAdminTemp {
                    id: row.get(0)?,
                    folder_path: row.get(1)?,
                    name: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<DeleteProjectAdminTemp>, _>>()?;

        let mut context = Context::new();
        context.insert("edit_project", &edit_project);
        context.insert("edit_project_list", &edit_project_list);
        context.insert("delete_exhibitions", &delete_exhibitions_by_year);
        context.insert("delete_exhibition_years", &years);
        context.insert("delete_projects", &delete_projects);
        let html = tera.render("admin/admin.html", &context)?;

        Ok(HttpResponse::Ok().body(html))
    }
}
