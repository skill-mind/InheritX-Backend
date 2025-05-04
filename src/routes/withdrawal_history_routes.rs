use crate::controller::withdrawal_history_controller;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    withdrawal_history_controller::config(cfg);
}
