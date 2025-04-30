use actix_web::{HttpResponse, Responder, web};
use serde_json::json;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/faqs")
            .route("", web::get().to(get_faqs))
            .route("", web::post().to(create_faq))
            .route("/{id}", web::put().to(update_faq))
            .route("/{id}", web::delete().to(delete_faq)),
    );
}

pub async fn get_faqs() -> impl Responder {
    // Placeholder logic for fetching FAQs
    HttpResponse::Ok().json(json!({"message": "List of FAQs"}))
}

pub async fn create_faq(faq: web::Json<serde_json::Value>) -> impl Responder {
    // Placeholder logic for creating an FAQ
    HttpResponse::Created().json(json!({"message": "FAQ created", "data": faq.into_inner()}))
}

pub async fn update_faq(path: web::Path<i32>, faq: web::Json<serde_json::Value>) -> impl Responder {
    // Placeholder logic for updating an FAQ
    HttpResponse::Ok()
        .json(json!({"message": "FAQ updated", "id": path.into_inner(), "data": faq.into_inner()}))
}

pub async fn delete_faq(path: web::Path<i32>) -> impl Responder {
    // Placeholder logic for deleting an FAQ
    HttpResponse::NoContent().finish()
}
