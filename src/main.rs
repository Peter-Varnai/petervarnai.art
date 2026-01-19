mod error;
mod handlers;
mod helpers;
mod models;

use actix_files::Files;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web::Data, App, HttpServer};
use handlers::{admin_service_config, project_service_config, public_service_config};
use helpers::server_config;
use models::AppState;
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Template parsing error(s): {}", e);
            std::process::exit(1);
        }
    };

    let server_config = match server_config() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error loading enviroment variables: {}", e);
            std::process::exit(1);
        }
    };

    let (host, port, db, pwd, root_dir, secret_key) = {
        (
            server_config.get("host").unwrap().as_str(),
            server_config.get("port").unwrap().as_num(),
            server_config.get("db").unwrap().as_path_buf(),
            server_config.get("pwd").unwrap().to_string(),
            server_config.get("root_dir").unwrap().as_path_buf(),
            Key::generate(),
        )
    };

    let state = AppState {
        tera,
        pwd,
        db,
        root_dir,
    };

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .service(Files::new("/f", "./templates").show_files_listing())
            .app_data(Data::new(state.clone()))
            .configure(public_service_config)
            .configure(admin_service_config)
            .configure(project_service_config)
    })
    .bind((host, port))?
    .run()
    .await
}
