// Using the figma file, create a route for faq and user supportuse actix_web::web;
use crate::controller::faq_controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/faqs")
            .route("", web::get().to(faq_controller::get_faqs))
            .route("", web::post().to(faq_controller::create_faq))
            .route("/{id}", web::put().to(faq_controller::update_faq))
            .route("/{id}", web::delete().to(faq_controller::delete_faq)),
    );
}
