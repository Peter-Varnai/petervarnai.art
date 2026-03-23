use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ProjectQueryRequest {
    pub no: u16,
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub password: String,
}

#[derive(Deserialize)]
pub struct DeleteExhibitionRequest {
    pub id: u16,
}

#[derive(Deserialize)]
pub struct DeleteProjectRequest {
    pub id: u16,
    #[allow(dead_code)]
    pub folder_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct PicDeleteRequest {
    pub filename: String,
}
