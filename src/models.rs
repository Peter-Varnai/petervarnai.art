use std::path::PathBuf;

use serde::{Serialize, Deserialize,};
use tera::Tera;
use time::Date;

// TERA TEMPLATING 
#[derive(Deserialize)]
pub struct ProjectNo {
    pub no: u16
}

#[derive(Serialize, Debug)]
pub struct ProjectList {
    pub id: u16,
    pub title: String,
}

#[derive(Serialize, Debug)]
pub struct Exhibition {
    pub id: u16,
    pub title: String,
    pub location: Option<String>,
    pub link: Option<String>,
    pub big_row: bool,
    pub date: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct Project {
    pub id: u16,
    pub title: String,
    pub pictures: Vec<String>,  // Assuming pictures stores multiple paths
    pub video_link: Option<String>,  // Nullable field
    pub concept: String,
    pub collaborators: String,  // Assuming comma-separated values
    pub medium: String,
    pub duration: String,
    pub date: String,
}


//FORMS
#[derive(Deserialize)]
pub struct ExhibitionForm {
    pub name: String,
    pub start_date: Date,
    pub till: Date,
    pub location: String,
    pub link: String,
}


#[derive(Deserialize)]
pub struct LoginForm {
    pub password: String,
}



//TODO: Transform this into a struct instead of a tuple struct
//for cleaner code when using its 0 field!
#[derive(Deserialize)]
pub struct Id(pub u16);



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
