use crate::{
    error::AppError,
    middleware::RequireAdmin,
    models::{AppState, DeleteProjectRequest, PicDeleteRequest, ProjectQueryRequest},
    services::project as project_service,
};

use actix_multipart::Multipart;
use actix_web::{
    get,
    web::{self, Data, Json, Query},
    HttpResponse, Result,
};

pub fn project_service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_project);

    cfg.service(
        web::resource("/project")
            .wrap(RequireAdmin)
            .route(web::post().to(add_project))
            .route(web::put().to(update_project))
            .route(web::delete().to(delete_project)),
    );

    cfg.service(
        web::resource("/projects/pic_update/{dir}/images")
            .wrap(RequireAdmin)
            .route(web::post().to(upload_project_images))
            .route(web::delete().to(delete_project_image)),
    );
}

#[get("/project")]
async fn get_project(
    state: Data<AppState>,
    project: Query<ProjectQueryRequest>,
) -> Result<HttpResponse, AppError> {
    let project_id: u16 = project.no;
    let project = project_service::get_project_by_id(&state.db, project_id)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(project))
}

async fn add_project(state: Data<AppState>, payload: Multipart) -> Result<HttpResponse, AppError> {
    project_service::create_project(&state, payload).await?;
    Ok(HttpResponse::Ok().body("Success"))
}

async fn update_project(
    state: Data<AppState>,
    payload: Multipart,
) -> Result<HttpResponse, AppError> {
    project_service::update_project(&state, payload).await?;
    Ok(HttpResponse::Ok().json(""))
}

async fn delete_project(
    state: Data<AppState>,
    delete_project: Json<DeleteProjectRequest>,
) -> Result<HttpResponse, AppError> {
    project_service::delete_project(&state, &delete_project)?;
    Ok(HttpResponse::Ok().json(""))
}

pub async fn upload_project_images(
    state: web::Data<AppState>,
    path: web::Path<String>,
    payload: Multipart,
) -> Result<HttpResponse, AppError> {
    let project_dir = path.into_inner();

    let resp = project_service::upload_project_images(&state, project_dir, payload).await?;
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn delete_project_image(
    state: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<PicDeleteRequest>,
) -> Result<HttpResponse, AppError> {
    let dir = path.into_inner();
    let resp = project_service::delete_project_image(&state, dir, &body)?;
    Ok(HttpResponse::Ok().json(resp))
}
