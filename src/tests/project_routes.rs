use actix_web::test;
use rusqlite::params;

use super::helpers::{init_service, login_cookies, open_conn, seed_project, TestEnv};
use crate::models::Project;

fn multipart_body(
    fields: &[(&str, &str)],
    files: &[(&str, &str, &str, &[u8])],
) -> (String, Vec<u8>) {
    let boundary = "BOUNDARY-TEST".to_string();
    let mut body: Vec<u8> = Vec::new();

    for (name, value) in fields {
        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        body.extend_from_slice(
            format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", name).as_bytes(),
        );
        body.extend_from_slice(value.as_bytes());
        body.extend_from_slice(b"\r\n");
    }

    for (field, filename, content_type, data) in files {
        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        body.extend_from_slice(
            format!(
                "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                field, filename
            )
            .as_bytes(),
        );
        body.extend_from_slice(format!("Content-Type: {}\r\n\r\n", content_type).as_bytes());
        body.extend_from_slice(data);
        body.extend_from_slice(b"\r\n");
    }

    body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());
    (boundary, body)
}

#[actix_web::test]
async fn get_project_returns_json() {
    let env = TestEnv::new();
    let id = seed_project(
        &env,
        &Project {
            id: 0,
            title: "Json Project".to_string(),
            date: "2026-01".to_string(),
            video_link: Some("".to_string()),
            dir: "json_dir".to_string(),
            concept: "Json Concept".to_string(),
            medium: Some("".to_string()),
            duration: Some("".to_string()),
            saved_files: vec![],
        },
    );

    let app = init_service(&env).await;
    let req = test::TestRequest::get()
        .uri(&format!("/project?no={}", id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let v: serde_json::Value = serde_json::from_slice(&body).expect("invalid json");
    assert_eq!(v["title"], "Json Project");
    assert_eq!(v["concept"], "Json Concept");
    assert_eq!(v["dir"], "json_dir");
}

#[actix_web::test]
async fn create_project_requires_auth() {
    let env = TestEnv::new();
    let app = init_service(&env).await;

    let (boundary, body) = multipart_body(
        &[
            ("title", "New Project"),
            ("date", "2026-01"),
            ("video_link", ""),
            ("medium", ""),
            ("duration", ""),
            ("concept", "New concept"),
        ],
        &[],
    );

    let req = test::TestRequest::post()
        .uri("/project")
        .insert_header((
            "Content-Type",
            format!("multipart/form-data; boundary={}", boundary),
        ))
        .set_payload(body)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_redirection());

    let conn = open_conn(&env);
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM projects", [], |row| row.get(0))
        .unwrap();
    assert_eq!(count, 0);
}

#[actix_web::test]
async fn create_project_inserts_row() {
    let env = TestEnv::new();
    let app = init_service(&env).await;
    let cookies = login_cookies(&app, "test-password").await;

    let (boundary, body) = multipart_body(
        &[
            ("title", "Created Project"),
            ("date", "2026-01"),
            ("video_link", ""),
            ("medium", "Digital"),
            ("duration", "1:23"),
            ("concept", "Created concept"),
        ],
        &[],
    );

    let mut req = test::TestRequest::post().uri("/project").insert_header((
        "Content-Type",
        format!("multipart/form-data; boundary={}", boundary),
    ));
    for c in &cookies {
        req = req.cookie(c.clone());
    }
    let resp = test::call_service(&app, req.set_payload(body).to_request()).await;
    assert!(resp.status().is_success());
    let resp_body = test::read_body(resp).await;
    assert_eq!(String::from_utf8_lossy(&resp_body), "Success");

    let conn = open_conn(&env);
    let (id, dir): (i64, String) = conn
        .query_row(
            "SELECT id, dir FROM projects WHERE title = ?1",
            params!["Created Project"],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .expect("project row not found");
    assert!(id > 0);

    let created_dir = env.state.root_dir.join("templates/static/images").join(dir);
    assert!(created_dir.exists());
}

#[actix_web::test]
async fn update_project_updates_row() {
    let env = TestEnv::new();
    let id = seed_project(
        &env,
        &Project {
            id: 0,
            title: "Old Title".to_string(),
            date: "2026-01".to_string(),
            video_link: Some("".to_string()),
            dir: "old_dir".to_string(),
            concept: "Old Concept".to_string(),
            medium: Some("".to_string()),
            duration: Some("".to_string()),
            saved_files: vec![],
        },
    );

    let app = init_service(&env).await;
    let cookies = login_cookies(&app, "test-password").await;

    let id_s = id.to_string();
    let (boundary, body) = multipart_body(
        &[
            ("id", &id_s),
            ("title", "New Title"),
            ("date", "2026-02"),
            ("video_link", "https://v.example"),
            ("medium", "New Medium"),
            ("duration", "9:99"),
            ("concept", "New Concept"),
        ],
        &[],
    );

    let mut req = test::TestRequest::put().uri("/project").insert_header((
        "Content-Type",
        format!("multipart/form-data; boundary={}", boundary),
    ));
    for c in &cookies {
        req = req.cookie(c.clone());
    }
    let resp = test::call_service(&app, req.set_payload(body).to_request()).await;
    assert!(resp.status().is_redirection());

    let conn = open_conn(&env);
    let title: String = conn
        .query_row(
            "SELECT title FROM projects WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(title, "New Title");
}

#[actix_web::test]
async fn delete_project_removes_row() {
    let env = TestEnv::new();
    let id = seed_project(
        &env,
        &Project {
            id: 0,
            title: "Delete Me".to_string(),
            date: "2026-01".to_string(),
            video_link: Some("".to_string()),
            dir: "del_dir".to_string(),
            concept: "Delete Concept".to_string(),
            medium: Some("".to_string()),
            duration: Some("".to_string()),
            saved_files: vec![],
        },
    );

    let app = init_service(&env).await;
    let cookies = login_cookies(&app, "test-password").await;

    let mut req = test::TestRequest::delete().uri("/project");
    for c in &cookies {
        req = req.cookie(c.clone());
    }
    let resp = test::call_service(
        &app,
        req.set_json(
            &serde_json::json!({"id": id, "folder_path": "templates/static/images/del_dir"}),
        )
        .to_request(),
    )
    .await;
    assert!(resp.status().is_success());

    let conn = open_conn(&env);
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM projects WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(count, 0);
}

#[actix_web::test]
async fn upload_and_delete_project_image_updates_db_and_filesystem() {
    let env = TestEnv::new();
    let dir = "img_dir";
    seed_project(
        &env,
        &Project {
            id: 0,
            title: "Img Project".to_string(),
            date: "2026-01".to_string(),
            video_link: Some("".to_string()),
            dir: dir.to_string(),
            concept: "Img Concept".to_string(),
            medium: Some("".to_string()),
            duration: Some("".to_string()),
            saved_files: vec![],
        },
    );

    let app = init_service(&env).await;
    let cookies = login_cookies(&app, "test-password").await;

    // upload
    let (boundary, body) = multipart_body(&[], &[("images", "test.jpg", "image/jpeg", b"abc")]);
    let mut req = test::TestRequest::post()
        .uri(&format!("/projects/pic_update/{}/images", dir))
        .insert_header((
            "Content-Type",
            format!("multipart/form-data; boundary={}", boundary),
        ));
    for c in &cookies {
        req = req.cookie(c.clone());
    }
    let resp = test::call_service(&app, req.set_payload(body).to_request()).await;
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let saved = v["saved_files"].as_array().unwrap();
    assert!(saved.iter().any(|x| x == "test.jpg"));

    let img_path = env
        .state
        .root_dir
        .join("templates/static/images")
        .join(dir)
        .join("test.jpg");
    assert!(img_path.exists());

    let conn = open_conn(&env);
    let pics: String = conn
        .query_row(
            "SELECT pictures FROM projects WHERE dir = ?1",
            params![dir],
            |row| row.get(0),
        )
        .unwrap();
    assert!(pics.contains("test.jpg"));

    // delete
    let mut req = test::TestRequest::delete().uri(&format!("/projects/pic_update/{}/images", dir));
    for c in &cookies {
        req = req.cookie(c.clone());
    }
    let resp = test::call_service(
        &app,
        req.set_json(&serde_json::json!({"filename":"test.jpg"}))
            .to_request(),
    )
    .await;
    assert!(resp.status().is_success());
    assert!(!img_path.exists());

    let pics: String = conn
        .query_row(
            "SELECT pictures FROM projects WHERE dir = ?1",
            params![dir],
            |row| row.get(0),
        )
        .unwrap();
    assert!(!pics.contains("test.jpg"));
}
