use crate::models::notification::{CreateNotification, UpdateNotification};
use crate::repositories::notification_repository;
use actix_web::{HttpResponse, Responder, web};
use deadpool_postgres::Pool;
use serde_json::json;

pub async fn get_notifications(db_pool: web::Data<Pool>) -> impl Responder {
    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    match notification_repository::get_all(&client).await {
        Ok(notifications) => HttpResponse::Ok().json(notifications),
        Err(_) => HttpResponse::InternalServerError()
            .json(json!({"error": "Failed to fetch notifications"})),
    }
}

pub async fn get_notification(db_pool: web::Data<Pool>, path: web::Path<i32>) -> impl Responder {
    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    match notification_repository::get_by_id(&client, path.into_inner()).await {
        Ok(notification) => HttpResponse::Ok().json(notification),
        Err(_) => HttpResponse::NotFound().json(json!({"error": "Notification not found"})),
    }
}

pub async fn create_notification(
    db_pool: web::Data<Pool>,
    notification: web::Json<CreateNotification>,
) -> impl Responder {
    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    match notification_repository::create(&client, &notification.into_inner()).await {
        Ok(created) => HttpResponse::Created().json(created),
        Err(_) => HttpResponse::InternalServerError()
            .json(json!({"error": "Failed to create notification"})),
    }
}

pub async fn update_notification(
    db_pool: web::Data<Pool>,
    path: web::Path<i32>,
    notification: web::Json<UpdateNotification>,
) -> impl Responder {
    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    match notification_repository::update(&client, path.into_inner(), &notification.into_inner())
        .await
    {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(_) => HttpResponse::InternalServerError()
            .json(json!({"error": "Failed to update notification"})),
    }
}

pub async fn delete_notification(db_pool: web::Data<Pool>, path: web::Path<i32>) -> impl Responder {
    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    match notification_repository::delete(&client, path.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError()
            .json(json!({"error": "Failed to delete notification"})),
    }
}

pub async fn mark_notification_as_read(
    db_pool: web::Data<Pool>,
    path: web::Path<i32>,
) -> impl Responder {
    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    match notification_repository::mark_as_read(&client, path.into_inner()).await {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(_) => HttpResponse::InternalServerError()
            .json(json!({"error": "Failed to mark notification as read"})),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/notifications")
            .route("", web::get().to(get_notifications))
            .route("", web::post().to(create_notification))
            .route("/{id}", web::get().to(get_notification))
            .route("/{id}", web::put().to(update_notification))
            .route("/{id}", web::delete().to(delete_notification))
            .route("/{id}/read", web::post().to(mark_notification_as_read)),
    );
}
