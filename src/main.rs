mod controller;
mod db;
mod models;
mod repositories;
mod routes;

use actix_web::{App, HttpServer, web};
use controller::{claim_controller, notification_controller, kyc_controller};
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
            // Use the routes module to configure all application routes
            .configure(routes::configure)
            .configure(notification_controller::config)
            .configure(claim_controller::config)
            .configure(kyc_controller::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
