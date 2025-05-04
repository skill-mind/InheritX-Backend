use crate::controller::faq_controller;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/faqs")
            .route("", web::get().to(faq_controller::get_faqs))
            .route("", web::post().to(faq_controller::create_faq))
            .route("/{id}", web::put().to(faq_controller::update_faq))
            .route("/{id}", web::delete().to(faq_controller::delete_faq)),
    );
}
