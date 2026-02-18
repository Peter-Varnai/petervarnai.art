use crate::{
    error::AppError,
    middleware::RequireAdmin,
    models::{AppState, DeleteExhibitionRequest, Exhibition},
    services::exhibition as exhibition_service,
};
use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};

pub fn exhibition_service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/exhibition")
            .wrap(RequireAdmin)
            .route(web::post().to(add_exhibition))
            .route(web::delete().to(delete_exhibition)),
    );
}

// EXHIBITION HANDLERS
async fn add_exhibition(
    state: Data<AppState>,
    form: Json<Exhibition>,
) -> Result<HttpResponse, AppError> {
    println!("add exhibition route called");

    exhibition_service::add_exhibition(&state.db, form.into_inner())
        .expect("error saving exhib to db");

    Ok(HttpResponse::Ok().json("succesfully added exhibition"))
}

async fn delete_exhibition(
    state: Data<AppState>,
    id: Json<DeleteExhibitionRequest>,
) -> Result<HttpResponse, AppError> {
    println!("requested to delete exhibition with the id: {}", id.id);

    exhibition_service::delete_exhibition(&state.db, id.id)?;

    Ok(HttpResponse::Ok().json("successfully deleted exhibition"))
}
