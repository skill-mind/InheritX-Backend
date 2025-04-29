use actix_web::{test, web, App};
use uuid::Uuid;
use inheritx_backend::{
    config, controller, database, middlewares, models, routes,
};
use sqlx::{Pool, Postgres};
use chrono::Utc;

async fn setup_test_app() -> (
    test::TestRequest,
    Pool<Postgres>,
    Uuid, // User ID for testing
) {
    // Set up test database
    let pool = database::setup_test_db().await;
    
    // Create a test user
    let user_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO users (id, email, password_hash, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $4)
        "#,
        user_id,
        "test@example.com",
        "hashed_password",
        Utc::now()
    )
    .execute(&pool)
    .await
    .unwrap();
    
    // Create JWT token for the test user
    let token = middlewares::auth::create_token(user_id).unwrap();
    
    // Set up test request with authorization header
    let req = test::TestRequest::default()
        .insert_header(("Authorization", format!("Bearer {}", token)));
    
    (req, pool, user_id)
}

#[actix_web::test]
async fn test_create_and_get_plan() {
    let app = test::init_service(App::new().configure(routes::plan_routes::configure)).await;

    let id = Uuid::new_v4();
    let payload = serde_json::json!( {
        "id": id,
        "name": "Test Plan",
        "description": "This is a test",
        "required_approvals": 2,
        "current_approvals": 0
    });

    let req = test::TestRequest::post()
        .uri("/plans")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let req = test::TestRequest::get()
        .uri(&format!("/plans/{}", id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_update_and_delete_plan() {
    let app = test::init_service(App::new().configure(routes::plan_routes::configure)).await;

    let id = Uuid::new_v4();
    let payload = serde_json::json!( {
        "id": id,
        "name": "To Update",
        "description": "Will update soon",
        "required_approvals": 1,
        "current_approvals": 0
    });

    // Create a plan
    let req = test::TestRequest::post()
        .uri("/plans")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Update the plan
    let updated_payload = serde_json::json!( {
        "id": id,
        "name": "Updated Plan",
        "description": "Updated description",
        "required_approvals": 3,
        "current_approvals": 0
    });

    let req = test::TestRequest::put()
        .uri(&format!("/plans/{}", id))
        .set_json(&updated_payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Delete the plan
    let req = test::TestRequest::delete()
        .uri(&format!("/plans/{}", id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_create_approval_request() {
    let (req, pool, user_id) = setup_test_app().await;

    // Create a test plan
    let plan_id = Uuid::new_v4();
    let now = Utc::now();
    sqlx::query!(
        r#"
        INSERT INTO plans (id, user_id, name, multi_signature_approval, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $5)
        "#,
        plan_id,
        user_id,
        "Test Plan",
        false,
        now
    )
    .execute(&pool)
    .await
    .unwrap();

    // Create approval request
    let approval_req = ApprovalRequest {
        plan_id,
        approver_emails: vec![
            "approver1@example.com".to_string(),
            "approver2@example.com".to_string(),
        ],
        required_approvals: 2,
    };

    let app = test::init_service(App::new().configure(routes::approval::configure_routes)).await;

    // Send request
    let req = test::TestRequest::post()
        .uri("/api/approvals")
        .set_json(&approval_req)
        .to_request();

    let result: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    
    assert_eq!(result.get("plan_id").unwrap(), plan_id.to_string());
    assert_eq!(result.get("required_approvals").unwrap(), 2);
    assert_eq!(result.get("approver_count").unwrap(), 2);

    // Verify plan was updated
    let plan = sqlx::query!(
        r#"
        SELECT multi_signature_approval, required_approvals 
        FROM plans 
        WHERE id = $1
        "#,
        plan_id
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert!(plan.multi_signature_approval);
    assert_eq!(plan.required_approvals, 2);
}
