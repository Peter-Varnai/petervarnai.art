use serde::{Deserialize, Serialize};

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
