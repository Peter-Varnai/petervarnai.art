use std::path::PathBuf;
use tera::Tera;

#[derive(Clone)]
pub struct AppState {
    pub tera: Tera,
    pub pwd: String,
    pub db: PathBuf,
    pub root_dir: PathBuf,
}
