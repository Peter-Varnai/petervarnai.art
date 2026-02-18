use actix_web::test;

use super::helpers::{init_service, login_cookies, seed_project, TestEnv};
use crate::models::Project;

#[actix_web::test]
async fn login_failure_returns_401() {
    let env = TestEnv::new();
    let app = init_service(&env).await;

    let req = test::TestRequest::post()
        .uri("/login")
        .insert_header(("Content-Type", "application/x-www-form-urlencoded"))
        .set_payload("password=wrong")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

#[actix_web::test]
async fn logout_without_identity_returns_401() {
    let env = TestEnv::new();
    let app = init_service(&env).await;

    let req = test::TestRequest::post().uri("/logout").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

#[actix_web::test]
async fn logout_clears_access_to_admin() {
    let env = TestEnv::new();
    seed_project(
        &env,
        &Project {
            id: 0,
            title: "Admin Seed".to_string(),
            date: "2026-01".to_string(),
            video_link: Some("".to_string()),
            dir: "seed_dir".to_string(),
            concept: "Seed concept".to_string(),
            medium: Some("".to_string()),
            duration: Some("".to_string()),
            saved_files: vec![],
        },
    );

    let app = init_service(&env).await;
    let cookies = login_cookies(&app, "test-password").await;

    let mut req = test::TestRequest::post().uri("/logout");
    for c in &cookies {
        req = req.cookie(c.clone());
    }
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success());

    let logout_cookies: Vec<_> = resp.response().cookies().map(|c| c.into_owned()).collect();

    let mut admin_req = test::TestRequest::get().uri("/admin");
    if logout_cookies.is_empty() {
        // Fallback: no cookie updates were emitted.
    } else {
        for c in &logout_cookies {
            admin_req = admin_req.cookie(c.clone());
        }
    }
    let admin_resp = test::call_service(&app, admin_req.to_request()).await;
    assert!(admin_resp.status().is_success());
    let body = test::read_body(admin_resp).await;
    let body = String::from_utf8_lossy(&body);
    assert!(body.contains("SYSTEM ACCESS"));
}
