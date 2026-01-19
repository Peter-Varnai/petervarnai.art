use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tera::Tera;

// TERA TEMPLATING
#[derive(Deserialize)]
pub struct ProjectNo {
    pub no: u16,
}

#[derive(Serialize, Debug)]
pub struct ProjectList {
    pub id: u16,
    pub title: String,
}

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
pub struct EditProjectListItem {
    pub project_title: String,
    pub project_id: u16,
}

#[derive(Serialize, Debug, Clone)]
pub struct DeleteExhibition {
    pub id: u16,
    pub name: String,
    pub start_date: String,
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

impl Default for Project {
    fn default() -> Self {
        Self {
            id: 0,
            title: String::new(),
            date: String::new(),
            video_link: None,
            dir: String::new(),
            concept: String::new(),
            medium: None,
            duration: None,
            saved_files: Vec::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteProject {
    pub id: u16,
    pub name: Option<String>,
    pub folder_path: String,
}

//

//FORMS
// #[derive(Deserialize)]
// pub struct ExhibitionForm {
//     pub name: String,
//     pub start_date: String,
//     pub till: String,
//     pub location: String,
//     pub link: Option<String>,
// }

#[derive(Deserialize)]
pub struct LoginForm {
    pub password: String,
}

#[derive(Deserialize)]
pub struct AdminQuery {
    pub edit_project: Option<String>,
}

// pub struct ProjectForm {
//     pub id: u16,
//     pub title: String,
//     pub date: String,
//     pub video_link: Option<String>,
//     pub concept: String,
//     pub medium: Option<String>,
//     pub duration: Option<String>,
//     pub saved_files: Option<String>,
//     pub dir: String,
// }

//TODO: Transform this into a struct instead of a tuple struct
//for cleaner code when using its 0 field!
#[derive(Deserialize)]
pub struct Id {
    pub id: u16,
}

#[derive(Clone)]
pub struct TestAppState {
    pub pwd: String,
    pub db: PathBuf,
    pub root_dir: PathBuf,
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
