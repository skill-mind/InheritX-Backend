use actix_web::{web, HttpResponse, Responder};
use crate::model::ActivityLog;
use std::sync::Mutex;

pub async fn create(log: web::Json<ActivityLog>, data: web::Data<Mutex<Vec<ActivityLog>>>) -> impl Responder {
    let mut logs = data.lock().unwrap();
    logs.push(log.into_inner());
    HttpResponse::Created().json("Activity log created")
}

pub async fn get_all(data: web::Data<Mutex<Vec<ActivityLog>>>) -> impl Responder {
    let logs = data.lock().unwrap();
    HttpResponse::Ok().json(&*logs)
}

pub async fn update(path: web::Path<u32>, log: web::Json<ActivityLog>, data: web::Data<Mutex<Vec<ActivityLog>>>) -> impl Responder {
    let id = path.into_inner();
    let mut logs = data.lock().unwrap();
    if let Some(existing) = logs.iter_mut().find(|x| x.id == id) {
        *existing = log.into_inner();
        HttpResponse::Ok().json("Updated")
    } else {
        HttpResponse::NotFound().json("Not found")
    }
}

pub async fn delete(path: web::Path<u32>, data: web::Data<Mutex<Vec<ActivityLog>>>) -> impl Responder {
    let id = path.into_inner();
    let mut logs = data.lock().unwrap();
    let len_before = logs.len();
    logs.retain(|x| x.id != id);
    if logs.len() < len_before {
        HttpResponse::Ok().json("Deleted")
    } else {
        HttpResponse::NotFound().json("Not found")
    }
}
