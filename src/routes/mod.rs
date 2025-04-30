
pub mod plan_routes;
pub mod activity_routes;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    activity_routes::configure(cfg);
}
