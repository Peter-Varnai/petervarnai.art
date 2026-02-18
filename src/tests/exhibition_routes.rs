use actix_web::test;
use rusqlite::params;

use crate::models::Exhibition;

use super::helpers::{init_service, login_cookies, open_conn, TestEnv};

#[actix_web::test]
async fn add_and_delete_exhibition_updates_database() {
    let env = TestEnv::new();
    let app = init_service(&env).await;
    let cookies = login_cookies(&app, "test-password").await;

    let exhib = Exhibition {
        id: None,
        title: "Test Exhibition".to_string(),
        location: Some("Somewhere".to_string()),
        link: Some("https://example.com".to_string()),
        r#type: 1,
        start_date: "2026-01-01".to_string(),
        till: "2026-02-01".to_string(),
    };

    let mut req = test::TestRequest::post().uri("/exhibition");
    for c in &cookies {
        req = req.cookie(c.clone());
    }
    let resp = test::call_service(&app, req.set_json(&exhib).to_request()).await;
    assert!(resp.status().is_success(), "status was {}", resp.status());

    let conn = open_conn(&env);
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM exhibitions", [], |row| row.get(0))
        .expect("failed to count exhibitions");
    assert_eq!(count, 1);

    let id: i64 = conn
        .query_row(
            "SELECT id FROM exhibitions WHERE title = ?1",
            params!["Test Exhibition"],
            |row| row.get(0),
        )
        .expect("failed to load inserted exhibition id");

    let mut req = test::TestRequest::delete().uri("/exhibition");
    for c in &cookies {
        req = req.cookie(c.clone());
    }
    let resp = test::call_service(
        &app,
        req.set_json(&serde_json::json!({"id": id})).to_request(),
    )
    .await;
    assert!(resp.status().is_success());

    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM exhibitions", [], |row| row.get(0))
        .expect("failed to count exhibitions");
    assert_eq!(count, 0);
}

#[actix_web::test]
async fn exhibition_routes_require_auth() {
    let env = TestEnv::new();
    let app = init_service(&env).await;

    let exhib = Exhibition {
        id: None,
        title: "Nope".to_string(),
        location: Some("".to_string()),
        link: Some("".to_string()),
        r#type: 0,
        start_date: "2026-01-01".to_string(),
        till: "2026-01-02".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/exhibition")
        .set_json(&exhib)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_redirection());

    let conn = open_conn(&env);
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM exhibitions", [], |row| row.get(0))
        .unwrap();
    assert_eq!(count, 0);
}
