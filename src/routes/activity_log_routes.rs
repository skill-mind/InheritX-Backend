use actix_web::{web, HttpResponse};
use crate::controller::activity_log_controller::ActivityLog;


pub fn activity_log_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/activity-log")
            .route(web::post().to(async |log: web::Json<ActivityLog>| {
                HttpResponse::Ok().body(format!("Activity log created: {:?}", log))
            }))
            .route(web::get().to(async || {
                let logs: Vec<ActivityLog> = vec![]; // Explicitly specify the type
                HttpResponse::Ok().json(logs) // Return the empty list as JSON
            }))
            .route(web::put().to(async |(id, log): (web::Path<u32>, web::Json<ActivityLog>)| {
                HttpResponse::Ok().body(format!("Activity log with ID {} updated: {:?}", id, log))
            }))
            .route(web::delete().to(async |id: web::Path<u32>| {
                HttpResponse::Ok().body(format!("Activity log with ID {} deleted", id))
            })),
    );
}