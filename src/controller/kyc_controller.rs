use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::Pool;
use serde_json::json;

use crate::models::kyc_models::{
    CreateKycRequest, KycQueryById, KycQueryByUserId, KycRecordResponse, KycVerificationRequest,
};
use crate::repositories::kyc_repository;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/kyc")
            .route("/create", web::post().to(create_kyc))
            .route("/verify", web::post().to(verify_kyc))
            .route("/status", web::get().to(get_kyc_status))
            .route("/user", web::get().to(get_kyc_by_user)),
    );
}

pub async fn create_kyc(
    db_pool: web::Data<Pool>,
    kyc_request: web::Json<CreateKycRequest>,
) -> impl Responder {
    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    match kyc_repository::create_kyc(&client, &kyc_request.into_inner()).await {
        Ok(kyc) => {
            let response = KycRecordResponse {
                id: kyc.id,
                user_id: kyc.user_id,
                full_name: kyc.full_name,
                date_of_birth: kyc.date_of_birth,
                id_type: kyc.id_type,
                id_number: kyc.id_number,
                address: kyc.address,
                verification_status: kyc.verification_status,
                created_at: kyc.created_at.format("%d-%m-%Y").to_string(),
                updated_at: kyc
                    .updated_at
                    .map(|dt| dt.format("%d-%m-%Y").to_string()),
            };
            HttpResponse::Created().json(response)
        }
        Err(e) => {
            eprintln!("Failed to create KYC record: {:?}", e);
            HttpResponse::InternalServerError()
                .json(json!({"error": "Failed to create KYC record"}))
        }
    }
}

pub async fn verify_kyc(
    db_pool: web::Data<Pool>,
    verification_request: web::Json<KycVerificationRequest>,
) -> impl Responder {
    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    match kyc_repository::update_kyc_verification_status(
        &client,
        verification_request.id,
        &verification_request.verification_status,
    )
    .await
    {
        Ok(kyc) => {
            let response = KycRecordResponse {
                id: kyc.id,
                user_id: kyc.user_id,
                full_name: kyc.full_name,
                date_of_birth: kyc.date_of_birth,
                id_type: kyc.id_type,
                id_number: kyc.id_number,
                address: kyc.address,
                verification_status: kyc.verification_status,
                created_at: kyc.created_at.format("%d-%m-%Y").to_string(),
                updated_at: kyc
                    .updated_at
                    .map(|dt| dt.format("%d-%m-%Y").to_string()),
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            eprintln!("Failed to update KYC verification status: {:?}", e);
            HttpResponse::InternalServerError()
                .json(json!({"error": "Failed to update KYC verification status"}))
        }
    }
}

pub async fn get_kyc_status(
    db_pool: web::Data<Pool>,
    query: web::Query<KycQueryById>,
) -> impl Responder {
    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    match kyc_repository::get_kyc_by_id(&client, query.id).await {
        Ok(kyc) => {
            let response = KycRecordResponse {
                id: kyc.id,
                user_id: kyc.user_id,
                full_name: kyc.full_name,
                date_of_birth: kyc.date_of_birth,
                id_type: kyc.id_type,
                id_number: kyc.id_number,
                address: kyc.address,
                verification_status: kyc.verification_status,
                created_at: kyc.created_at.format("%d-%m-%Y").to_string(),
                updated_at: kyc
                    .updated_at
                    .map(|dt| dt.format("%d-%m-%Y").to_string()),
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            eprintln!("Failed to get KYC record: {:?}", e);
            HttpResponse::NotFound().json(json!({"error": "KYC record not found"}))
        }
    }
}

pub async fn get_kyc_by_user(
    db_pool: web::Data<Pool>,
    query: web::Query<KycQueryByUserId>,
) -> impl Responder {
    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    match kyc_repository::get_kyc_by_user_id(&client, query.user_id).await {
        Ok(kyc) => {
            let response = KycRecordResponse {
                id: kyc.id,
                user_id: kyc.user_id,
                full_name: kyc.full_name,
                date_of_birth: kyc.date_of_birth,
                id_type: kyc.id_type,
                id_number: kyc.id_number,
                address: kyc.address,
                verification_status: kyc.verification_status,
                created_at: kyc.created_at.format("%d-%m-%Y").to_string(),
                updated_at: kyc
                    .updated_at
                    .map(|dt| dt.format("%d-%m-%Y").to_string()),
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            eprintln!("Failed to get KYC record: {:?}", e);
            HttpResponse::NotFound().json(json!({"error": "KYC record not found for this user"}))
        }
    }
}
