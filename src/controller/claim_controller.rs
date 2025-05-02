use crate::models::claim::{ClaimStatus, CreateClaim, UpdateClaim};
use crate::repositories::claim_repository;
use actix_web::{HttpResponse, Responder, web};
use deadpool_postgres::Pool;
use serde_json::json;

pub async fn get_claims(db_pool: web::Data<Pool>) -> impl Responder {
    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    match claim_repository::get_all(&client).await {
        Ok(claims) => HttpResponse::Ok().json(claims),
        Err(_) => {
            HttpResponse::InternalServerError().json(json!({"error": "Failed to fetch claims"}))
        }
    }
}

pub async fn get_user_claims(db_pool: web::Data<Pool>, path: web::Path<i32>) -> impl Responder {
    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    match claim_repository::get_by_user_id(&client, path.into_inner()).await {
        Ok(claims) => HttpResponse::Ok().json(claims),
        Err(_) => HttpResponse::InternalServerError()
            .json(json!({"error": "Failed to fetch user claims"})),
    }
}

pub async fn get_claims_by_status(
    db_pool: web::Data<Pool>,
    path: web::Path<String>,
) -> impl Responder {
    let status = match path.into_inner().to_lowercase().as_str() {
        "pending" => ClaimStatus::Pending,
        "approved" => ClaimStatus::Approved,
        "rejected" => ClaimStatus::Rejected,
        _ => return HttpResponse::BadRequest().json(json!({"error": "Invalid status"})),
    };

    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    match claim_repository::get_by_status(&client, status).await {
        Ok(claims) => HttpResponse::Ok().json(claims),
        Err(_) => HttpResponse::InternalServerError()
            .json(json!({"error": "Failed to fetch claims by status"})),
    }
}

pub async fn get_user_claims_by_status(
    db_pool: web::Data<Pool>,
    path: web::Path<(i32, String)>,
) -> impl Responder {
    let (user_id, status_str) = path.into_inner();
    let status = match status_str.to_lowercase().as_str() {
        "pending" => ClaimStatus::Pending,
        "approved" => ClaimStatus::Approved,
        "rejected" => ClaimStatus::Rejected,
        _ => return HttpResponse::BadRequest().json(json!({"error": "Invalid status"})),
    };

    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    match claim_repository::get_by_user_and_status(&client, user_id, status).await {
        Ok(claims) => HttpResponse::Ok().json(claims),
        Err(_) => HttpResponse::InternalServerError()
            .json(json!({"error": "Failed to fetch user claims by status"})),
    }
}

pub async fn create_claim(
    db_pool: web::Data<Pool>,
    claim: web::Json<CreateClaim>,
) -> impl Responder {
    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    match claim_repository::create(&client, &claim.into_inner()).await {
        Ok(created) => HttpResponse::Created().json(created),
        Err(_) => {
            HttpResponse::InternalServerError().json(json!({"error": "Failed to create claim"}))
        }
    }
}

pub async fn update_claim(
    db_pool: web::Data<Pool>,
    path: web::Path<i32>,
    claim: web::Json<UpdateClaim>,
) -> impl Responder {
    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    match claim_repository::update(&client, path.into_inner(), &claim.into_inner()).await {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(_) => {
            HttpResponse::InternalServerError().json(json!({"error": "Failed to update claim"}))
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/claims")
            .route("", web::get().to(get_claims))
            .route("", web::post().to(create_claim))
            .route("/user/{user_id}", web::get().to(get_user_claims))
            .route("/status/{status}", web::get().to(get_claims_by_status))
            .route(
                "/user/{user_id}/status/{status}",
                web::get().to(get_user_claims_by_status),
            )
            .route("/{id}", web::put().to(update_claim)),
    );
}
