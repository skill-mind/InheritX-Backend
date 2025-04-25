use actix_web::{test, App};
use inheritx_backend::routes::plan_routes::configure;
use uuid::Uuid;

#[actix_web::test]
async fn test_create_and_get_plan() {
    let app = test::init_service(App::new().configure(configure)).await;

    let id = Uuid::new_v4();
    let payload = serde_json::json!({
        "id": id,
        "name": "Test Plan",
        "description": "This is a test"
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
    let app = test::init_service(App::new().configure(configure)).await;

    let id = Uuid::new_v4();
    let payload = serde_json::json!({
        "id": id,
        "name": "To Update",
        "description": "Will update soon"
    });

    let req = test::TestRequest::post()
        .uri("/plans")
        .set_json(&payload)
        .to_request();
    test::call_service(&app, req).await;

    let update = serde_json::json!({
        "id": id,
        "name": "Updated Plan",
        "description": "Updated successfully"
    });

    let req = test::TestRequest::put() 
        .uri(&format!("/plans/{}", id))
        .set_json(&update)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let req = test::TestRequest::delete()
        .uri(&format!("/plans/{}", id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
