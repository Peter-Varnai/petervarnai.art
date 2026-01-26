use crate::models::{AppState, LoginForm};
use actix_identity::Identity;
use actix_web::{
    post,
    web::{self},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
pub fn admin_service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(logout);
}

#[post("/login")]
async fn login(
    state: web::Data<AppState>,
    form: web::Form<LoginForm>,
    request: HttpRequest,
) -> impl Responder {
    println!("login called");
    if form.password == state.pwd {
        Identity::login(&request.extensions(), "admin".into()).expect("Failed to log in");
        HttpResponse::Found()
            .append_header(("Location", "/admin"))
            .finish()
    } else {
        HttpResponse::Unauthorized().body("Invalid credentials")
    }
}

#[post("/logout")]
async fn logout(user: Identity) -> impl Responder {
    user.logout();
    println!("Log out called");
    HttpResponse::Ok().body("Logged out!")
}
