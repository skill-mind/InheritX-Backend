use crate::controller::withdrawal_history_controller;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/withdrawal_history")
            .route(
                "/create",
                web::post().to(withdrawal_history_controller::record_withdrawal),
            )
            .route(
                "",
                web::get().to(withdrawal_history_controller::get_withdrawal_history),
            )
            .route(
                "/delete",
                web::delete().to(withdrawal_history_controller::delete_withdrawal),
            )
            .route(
                "find_one/{id}",
                web::get().to(withdrawal_history_controller::get_single_withdrawal),
            )
            .route(
                "/update",
                web::patch().to(withdrawal_history_controller::update_withdrawal),
            )
            .route(
                "user_history/{id}",
                web::get().to(withdrawal_history_controller::get_withdrawal_history_by_user),
            ),
    );
}
