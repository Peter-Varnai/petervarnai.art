use serde::Serialize;

#[derive(Serialize)]
pub struct PicUpdateResponse {
    pub saved_files: Vec<String>,
}

#[derive(Serialize)]
pub struct PicDeleteResponse {
    pub saved_files: Vec<String>,
}
