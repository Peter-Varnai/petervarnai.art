pub mod domain;
pub mod requests;
pub mod responses;
pub mod state;
pub mod templating;

pub use domain::{Exhibition, Project};
pub use requests::{
    DeleteExhibitionRequest, DeleteProjectRequest, LoginForm, PicDeleteRequest, ProjectQueryRequest,
};
pub use responses::{PicDeleteResponse, PicUpdateResponse};
pub use state::AppState;
pub use templating::{
    DeleteExhibitionAdminTemp, DeleteProjectAdminTemp, EditProjectAdminTemp, ProjectsIndexTemp,
};
