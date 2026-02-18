use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    body::BoxBody,
    cookie::{Cookie, Key},
    dev::{Service, ServiceResponse},
    test,
    web::Data,
    App,
};
use rusqlite::Connection;
use std::path::PathBuf;
use tera::Tera;

use crate::handlers::{
    admin_service_config, exhibition_service_config, project_service_config, public_service_config,
};

use crate::{
    db::init_db,
    models::{AppState, Exhibition, Project},
};

pub struct TestEnv {
    _tmp: tempfile::TempDir,
    pub state: AppState,
    pub secret_key: Key,
}

impl TestEnv {
    pub fn new() -> Self {
        let tmp = tempfile::tempdir().expect("failed to create tempdir");
        let root_dir = tmp.path().to_path_buf();

        std::fs::create_dir_all(root_dir.join("templates/static/images"))
            .expect("failed to create images directory tree");

        let db_path: PathBuf = root_dir.join("test.db");
        let conn = Connection::open(&db_path).expect("failed to open test db");
        init_db(&conn).expect("failed to init db schema");

        let tera_glob = format!("{}/templates/**/*.html", env!("CARGO_MANIFEST_DIR"));
        let tera = Tera::new(&tera_glob).expect("failed to load templates");

        let state = AppState {
            tera,
            pwd: "test-password".to_string(),
            db: db_path,
            root_dir,
        };

        Self {
            _tmp: tmp,
            state,
            secret_key: Key::from(&[0u8; 64]),
        }
    }
}

pub async fn init_service(
    env: &TestEnv,
) -> impl Service<actix_http::Request, Response = ServiceResponse<BoxBody>, Error = actix_web::Error>
{
    test::init_service(
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                env.secret_key.clone(),
            ))
            .app_data(Data::new(env.state.clone()))
            .configure(public_service_config)
            .configure(admin_service_config)
            .configure(project_service_config)
            .configure(exhibition_service_config),
    )
    .await
}

pub async fn login_cookies<S>(app: &S, password: &str) -> Vec<Cookie<'static>>
where
    S: Service<actix_http::Request, Response = ServiceResponse<BoxBody>, Error = actix_web::Error>,
{
    let req = test::TestRequest::post()
        .uri("/login")
        .insert_header(("Content-Type", "application/x-www-form-urlencoded"))
        .set_payload(format!("password={}", password))
        .to_request();

    let resp = test::call_service(app, req).await;
    assert!(
        resp.status().is_redirection(),
        "status was {}",
        resp.status()
    );

    let cookies: Vec<_> = resp.response().cookies().map(|c| c.into_owned()).collect();
    assert!(!cookies.is_empty(), "expected at least one cookie");
    cookies
}

pub fn open_conn(env: &TestEnv) -> Connection {
    Connection::open(&env.state.db).expect("failed to open test db")
}

pub fn seed_project(env: &TestEnv, project: &Project) -> u16 {
    let conn = open_conn(env);
    conn.execute(
        "INSERT INTO projects (title, pictures, video, concept, dir, medium, duration, release)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params![
            project.title,
            serde_json::to_string(&project.saved_files).expect("failed to serialize saved_files"),
            project.video_link.clone().unwrap_or_default(),
            project.concept,
            project.dir,
            project.medium.clone().unwrap_or_default(),
            project.duration.clone().unwrap_or_default(),
            project.date
        ],
    )
    .expect("failed to seed project");

    conn.last_insert_rowid() as u16
}

pub fn seed_exhibition(env: &TestEnv, exhibition: &Exhibition) -> u16 {
    let conn = open_conn(env);
    conn.execute(
        "INSERT INTO exhibitions (title, start_date, till, location, link, type)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![
            exhibition.title,
            exhibition.start_date,
            exhibition.till,
            exhibition.location.clone().unwrap_or_default(),
            exhibition.link.clone().unwrap_or_default(),
            exhibition.r#type
        ],
    )
    .expect("failed to seed exhibition");

    conn.last_insert_rowid() as u16
}
