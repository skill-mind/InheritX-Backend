<<<<<<< HEAD
pub mod activity_log_routes;
=======
pub mod activity_routes;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    activity_routes::configure(cfg);
}
>>>>>>> 56c6a5374ab178105fe72dad5bcccfaba559832a
