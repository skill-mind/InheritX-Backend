use crate::controller::activity_controller;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/activities")
            .route(
                "",
                web::post().to(activity_controller::create_user_activity),
            )
            .route(
                "/{user_id}",
                web::get().to(activity_controller::get_user_activities_endpoint),
            ),
    );
}
