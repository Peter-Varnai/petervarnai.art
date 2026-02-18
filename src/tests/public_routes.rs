use actix_web::test;

use super::helpers::{init_service, login_cookies, seed_exhibition, seed_project, TestEnv};
use crate::models::{Exhibition, Project};

#[actix_web::test]
async fn admin_without_login_shows_login_page() {
    let env = TestEnv::new();
    let app = init_service(&env).await;

    let req = test::TestRequest::get().uri("/admin").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let body = String::from_utf8_lossy(&body);
    assert!(body.contains("SYSTEM ACCESS"));
}

#[actix_web::test]
async fn index_renders_seeded_projects_and_exhibitions() {
    let env = TestEnv::new();
    seed_project(
        &env,
        &Project {
            id: 0,
            title: "Seed Project".to_string(),
            date: "2026-01".to_string(),
            video_link: Some("".to_string()),
            dir: "seed_dir".to_string(),
            concept: "Seed concept".to_string(),
            medium: Some("".to_string()),
            duration: Some("".to_string()),
            saved_files: vec![],
        },
    );
    seed_exhibition(
        &env,
        &Exhibition {
            id: None,
            title: "Seed Exhibition".to_string(),
            location: Some("Vienna".to_string()),
            link: Some("".to_string()),
            r#type: 0,
            start_date: "2026-01-01".to_string(),
            till: "2026-02-01".to_string(),
        },
    );

    let app = init_service(&env).await;
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let body = String::from_utf8_lossy(&body);
    assert!(body.contains("Seed Project"));
    assert!(body.contains("Seed Exhibition"));
}

#[actix_web::test]
async fn project_page_renders_project_details() {
    let env = TestEnv::new();
    let id = seed_project(
        &env,
        &Project {
            id: 0,
            title: "Proj Title".to_string(),
            date: "2026-01".to_string(),
            video_link: Some("".to_string()),
            dir: "proj_dir".to_string(),
            concept: "Proj Concept".to_string(),
            medium: Some("".to_string()),
            duration: Some("".to_string()),
            saved_files: vec![],
        },
    );

    let app = init_service(&env).await;
    let req = test::TestRequest::get()
        .uri(&format!("/prj?no={}", id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let body = String::from_utf8_lossy(&body);
    assert!(body.contains("Proj Title"));
    assert!(body.contains("Proj Concept"));
}

#[actix_web::test]
async fn admin_with_login_renders_admin_panel() {
    let env = TestEnv::new();
    seed_project(
        &env,
        &Project {
            id: 0,
            title: "Admin Project".to_string(),
            date: "2026-01".to_string(),
            video_link: Some("".to_string()),
            dir: "admin_dir".to_string(),
            concept: "Admin Concept".to_string(),
            medium: Some("".to_string()),
            duration: Some("".to_string()),
            saved_files: vec![],
        },
    );

    let app = init_service(&env).await;
    let cookies = login_cookies(&app, "test-password").await;

    let mut req = test::TestRequest::get().uri("/admin");
    for c in &cookies {
        req = req.cookie(c.clone());
    }
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let body = String::from_utf8_lossy(&body);
    assert!(body.contains("Admin Panel"));
    assert!(body.contains("Admin Project"));
}
