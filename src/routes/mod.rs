pub mod activity_log_routes;
pub mod activity_routes;
pub mod withdrawal_history_routes;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    activity_routes::configure(cfg);
}
