use actix_web::{App, HttpServer, web};
use std::sync::Mutex;

mod controller;
mod routes;

use crate::routes::activity_log_routes;
use crate::controller::activity_log_controller::ActivityLog;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let logs = web::Data::new(Mutex::new(Vec::<ActivityLog>::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(logs.clone()) // inject shared state
            .configure(activity_log_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
