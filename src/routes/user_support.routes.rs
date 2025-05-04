use crate::controller::user_support_controller;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/support")
            .route(
                "/tickets",
                web::get().to(user_support_controller::get_tickets),
            )
            .route(
                "/tickets",
                web::post().to(user_support_controller::create_ticket),
            )
            .route(
                "/tickets/{id}/reply",
                web::post().to(user_support_controller::reply_to_ticket),
            ),
    );
}
