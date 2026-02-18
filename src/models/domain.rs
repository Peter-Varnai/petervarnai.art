use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Exhibition {
    pub id: Option<u16>,
    pub title: String,
    pub location: Option<String>,
    pub link: Option<String>,
    pub r#type: i8,
    pub start_date: String,
    pub till: String,
}

#[derive(Serialize, Debug)]
pub struct Project {
    pub id: u16,
    pub title: String,
    pub date: String,
    pub video_link: Option<String>,
    pub dir: String,
    pub concept: String,
    pub medium: Option<String>,
    pub duration: Option<String>,
    pub saved_files: Vec<String>,
}
