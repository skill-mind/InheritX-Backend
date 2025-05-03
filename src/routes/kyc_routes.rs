use actix_web::web;
use crate::controller::kyc_controller;

pub fn configure(cfg: &mut web::ServiceConfig) {
    kyc_controller::config(cfg);
}
