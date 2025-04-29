use actix_web::{web, App, HttpServer};
use crate::routes;
use sqlx::PgPool;
use inheritx_backend;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect("postgres://user:password@localhost/database_name").await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::plan::configure_routes)
            .configure(routes::approval::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}