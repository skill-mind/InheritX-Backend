use actix_web::{App, HttpServer};
use crate::routes::activity_log_routes::activity_log_routes;

mod controller;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    HttpServer::new(|| {
        App::new()
            .configure(activity_log_routes) // Register the activity log routes
    })
    .bind("127.0.0.1:8080")? 
    .run()
    .await
}