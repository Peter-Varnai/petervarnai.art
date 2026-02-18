use crate::{
    error::AppError,
    models::{AppState, DeleteExhibitionAdminTemp, Exhibition, ProjectQueryRequest},
    services::{project as project_service, site as site_service},
};
use actix_identity::Identity;
use actix_web::{
    get,
    web::{self, Data, Query},
    HttpResponse,
};
use std::collections::HashMap;
use tera::Context;

pub fn public_service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(index).service(project).service(admin);
}

#[get("/")]
async fn index(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let tera = &state.tera;

    let projects = site_service::list_projects_index(&state.db)?;
    let exhibitions = site_service::list_exhibitions(&state.db)?;

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

    let project = project_service::get_project_by_id(&state.db, project_id)?;

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
        let edit_project_list = site_service::admin_edit_project_list(&state.db)?;
        let edit_project = site_service::admin_latest_project(&state.db)?;
        let delete_exhibitions = site_service::admin_delete_exhibitions(&state.db)?;

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

        let delete_projects = site_service::admin_delete_projects(&state.db)?;

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
