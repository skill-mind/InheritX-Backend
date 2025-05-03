use actix_web::web;
use crate::controller::activity_log_controller;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/activity-log")
            .route("", web::post().to(activity_log_controller::create))
            .route("", web::get().to(activity_log_controller::get_all))
            .route("/{id}", web::put().to(activity_log_controller::update))
            .route("/{id}", web::delete().to(activity_log_controller::delete)),
    );
}
