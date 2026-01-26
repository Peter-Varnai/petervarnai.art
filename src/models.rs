use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tera::Tera;

#[derive(Deserialize)]
pub struct ProjectQueryRequest {
    pub no: u16,
}

//TODO: Consider making a separate struct for each Tera templater, each Request and Response and
//separate the different structs to different files
// the Exhibitiona and the Project structs are both used
// for rendering templates and
// to validate input upon exhibition/project creation
#[derive(Serialize, Deserialize, Debug)]
pub struct Exhibition {
    pub id: Option<u16>,
    pub title: String,
    pub location: Option<String>,
    pub link: Option<String>,
    pub big_row: bool,
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

// TERA TEMPLATING
#[derive(Serialize, Debug)]
pub struct ProjectsIndexTemp {
    pub id: u16,
    pub title: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteProjectAdminTemp {
    pub id: u16,
    pub name: Option<String>,
    pub folder_path: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct DeleteExhibitionAdminTemp {
    pub id: u16,
    pub name: String,
    pub start_date: String,
}

#[derive(Serialize, Debug)]
pub struct EditProjectAdminTemp {
    pub project_title: String,
    pub project_id: u16,
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
    pub folder_path: String,
}

#[derive(Clone)]
pub struct AppState {
    pub tera: Tera,
    pub pwd: String,
    pub db: PathBuf,
    pub root_dir: PathBuf,
}

// SERVER WARMUP
#[derive(Debug)]
pub enum ConfigValue {
    StringValue(String),
    NumberValue(u16),
    PathValue(PathBuf),
}

impl ConfigValue {
    pub fn to_string(&self) -> String {
        match self {
            ConfigValue::StringValue(s) => s.to_string(),
            ConfigValue::NumberValue(n) => n.to_string(),
            ConfigValue::PathValue(p) => p.to_string_lossy().into_owned(),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ConfigValue::StringValue(s) => s,
            _ => panic!("Config value isn't str value!"),
        }
    }

    pub fn as_path_buf(&self) -> PathBuf {
        match self {
            ConfigValue::PathValue(p) => p.clone(),
            _ => panic!("Config value isn't PathBuf value!"),
        }
    }

    pub fn as_num(&self) -> u16 {
        match self {
            ConfigValue::NumberValue(n) => *n,
            _ => panic!("Config value isn't number value!"),
        }
    }
}
