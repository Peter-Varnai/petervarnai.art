use crate::{
    error::ApiError,
    models::{AppState, DeleteExhibitionRequest, Exhibition},
};
use actix_identity::Identity;
use actix_web::{
    delete, post,
    web::{self, Data, Json},
    HttpResponse,
};
use rusqlite::{params, Connection};
use tera::Context;

pub fn exhibition_service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(add_exhibition).service(delete_exhibition);
}

// fn delete_folder_with_contents(path: &PathBuf) -> Result<(), ApiError> {
//     fs::remove_dir_all(path)?;
//     println!("Folder and all its contents deleted \n {}", path.display());
//     Ok(())
// }

// EXHIBITION HANDLERS
#[post("/exhibition")]
async fn add_exhibition(
    state: Data<AppState>,
    form: Json<Exhibition>,
    identity: Option<Identity>,
) -> Result<HttpResponse, ApiError> {
    let tera = &state.tera;

    if identity.is_none() {
        // if false {
        let context = Context::new();
        let login = tera.render("login.html", &context)?;

        Ok(HttpResponse::Ok().content_type("text/html").body(login))
    } else {
        let db_path = &state.db;
        let conn = Connection::open(db_path)?;

        println!("add exhibition route called");

        conn.execute(
            "INSERT INTO exhibitions (name, start_date, till, location, link, type) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                form.title,
                form.start_date,
                form.till,
                form.location,
                form.link,
                form.r#type,
            ],
        )?;

        Ok(HttpResponse::Ok().json("succesfully added exhibition"))
    }
}

#[delete("/exhibition")]
async fn delete_exhibition(
    state: Data<AppState>,
    id: Json<DeleteExhibitionRequest>,
    identity: Option<Identity>,
) -> Result<HttpResponse, ApiError> {
    let tera = &state.tera;

    if identity.is_none() {
        // if false {
        let context = Context::new();
        let login = tera.render("login.html", &context)?;

        Ok(HttpResponse::Ok().content_type("text/html").body(login))
    } else {
        let db_path = &state.db;
        let conn = Connection::open(db_path)?;

        println!("requested to delete exhibition with the id: {}", id.id);

        conn.execute("DELETE FROM exhibitions WHERE id = ?1", params![id.id])?;

        Ok(HttpResponse::Ok().json("successfully deleted exhibition"))
    }
}
