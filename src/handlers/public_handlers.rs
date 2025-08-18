use actix_web::{get, post, web::{self, Query, Data}, HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use tera::Context;
use rusqlite::{params, Connection};
use crate::{
    error::AppError,
    models::{AppState, Exhibition, Id, LoginForm, Project, ProjectList, ProjectNo},
};

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
    let projects = stmt.query_map([], |row| {
        Ok(ProjectList {
            id: row.get(0)?,
            title: row.get(1)?,
        })
    })?.collect::<Result<Vec<_>, rusqlite::Error>>()?;

    let mut exhib_stmt = conn.prepare("SELECT id, title, location, link, big_row, date FROM about")?;

    let exhibitions = exhib_stmt.query_map([],
        |row| {
            Ok(Exhibition{
                id: row.get(0)?,
                title: row.get(1)?,
                location: row.get(2)?,
                link: row.get(3)?,
                big_row: row.get(4)?,
                date: row.get(5)?,
            })
        })?.collect::<Result<Vec<_>, rusqlite::Error>>()?;

    // projects.collect();
    let mut context = Context::new();
    context.insert("projects_list", &projects);
    context.insert("exhibitions_table", &exhibitions);
   
    let rendered = tera.render("index.html", &context).expect("error rendering index template");

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}


#[get("/prj")]
async fn project(
    state: Data<AppState>, 
    project: Query<ProjectNo>
) -> Result<HttpResponse, AppError> {
    let tera = &state.tera;
    let project_id: u16 = project.no;
    println!("project id: {}", project_id);

    let db_path = &state.db;
    let conn = Connection::open(db_path)?;
    let mut proj_list_stmt = conn.prepare("SELECT title, pictures, video, concept, 
        collaborators, medium, duration, release, id FROM projects WHERE id = ?1")?;
    let project = proj_list_stmt.query_row(
        params![project_id],
        |row| {
            let pictures_json: String = row.get(1)?;
            let pictures: Vec<String> = serde_json::from_str(&pictures_json)
                .expect("Failed to parse pictures JSON");

            Ok(Project {
                title: row.get(0)?,
                pictures,
                video_link: row.get(2)?,
                concept: row.get(3)?,
                collaborators: row.get(4)?,
                medium: row.get(5)?,
                duration: row.get(6)?,
                date: row.get(7)?,
                id: row.get(8)?,
            })
        }
    )?;

    let mut context = Context::new();
    context.insert("project", &project);

    let rendered = tera.render("project.html", &context).expect("error rendering index template");

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}


#[get("/admin")]
async fn admin(
    identity: Option<Identity>,
    id: Query<Id>,
    state: Data<AppState>
) -> impl Responder {
    let tera = &state.tera;

    if identity.is_none() {
        let context = Context::new();
        return match tera.render("login.html", &context) {
            Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
            Err(e) => {
                eprintln!("Template error: {}", e);
                HttpResponse::InternalServerError().body("Template error")
            }
        };
    }


    let context = Context::new();
    match tera.render("admin.html", &context) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => {
            eprintln!("Template error: {}", err);
            HttpResponse::InternalServerError().body("Template error")
        }
    }
}


#[post("/login")]
async fn login(
    state: web::Data<AppState>, 
    form: web::Form<LoginForm>, 
    request: HttpRequest
) -> impl Responder {
    println!("login called");
    if form.password == state.pwd {
        Identity::login(&request.extensions(), "admin".into()).expect("Failed to log in");
        HttpResponse::Found().append_header(("Location", "/admin")).finish()
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
