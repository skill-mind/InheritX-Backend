use actix_web::web;
use crate::controller::approval::{
    get_approval, get_approval_status, request_approvals, submit_approval,
};
use crate::middlewares::auth::Authentication;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/approvals")
            .route("", web::post().to(request_approvals))
            .route("/{plan_id}", web::get().to(get_approval_status))
            .wrap(Authentication)
    );
    

    cfg.service(
        web::scope("/api/public/approvals")
            .route("/{approval_id}", web::get().to(get_approval))
            .route("/submit", web::post().to(submit_approval))
    );
}