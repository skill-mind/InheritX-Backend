pub fn activity_log_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/activity-log")
            .route("", web::post().to(create))
            .route("", web::get().to(get_all))
            .route("/{id}", web::put().to(update))
            .route("/{id}", web::delete().to(delete))
    );
}
