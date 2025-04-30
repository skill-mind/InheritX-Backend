mod controller;
mod db;
mod models;
mod repositories;

use actix_web::{App, HttpServer, web};
use controller::{faq_controller, notification_controller, user_support_controller};
use db::create_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create database connection pool
    let pool = create_pool().await;

    // Run migrations
    db::run_migrations(&pool).await;

    println!("Starting server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(notification_controller::config)
            .configure(faq_controller::config)
            .configure(user_support_controller::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
