mod models;
pub mod routes;
use actix_web::{App, HttpServer};
use routes::plan_routes;

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(plan_routes::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
