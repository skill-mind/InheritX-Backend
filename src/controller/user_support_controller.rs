use actix_web::{HttpResponse, Responder, web};
use serde_json::json;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/support")
            .route("/tickets", web::get().to(get_tickets))
            .route("/tickets", web::post().to(create_ticket))
            .route("/tickets/{id}/reply", web::post().to(reply_to_ticket)),
    );
}

pub async fn get_tickets() -> impl Responder {
    // Placeholder logic for fetching tickets
    HttpResponse::Ok().json(json!({"message": "List of tickets"}))
}

pub async fn create_ticket(ticket: web::Json<serde_json::Value>) -> impl Responder {
    // Placeholder logic for creating a ticket
    HttpResponse::Created().json(json!({"message": "Ticket created", "data": ticket.into_inner()}))
}

pub async fn reply_to_ticket(
    path: web::Path<i32>,
    reply: web::Json<serde_json::Value>,
) -> impl Responder {
    // Placeholder logic for replying to a ticket
    HttpResponse::Ok().json(json!({"message": "Reply added to ticket", "id": path.into_inner(), "data": reply.into_inner()}))
}
