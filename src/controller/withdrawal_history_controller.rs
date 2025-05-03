use actix_web::{HttpResponse, Responder, web};
use deadpool_postgres::Pool;
use serde_json::json;

use crate::models::withdrawal_history_models::{
    CreateWithdrawalRecordRequest, SingleWithdrawalRecordRequest, WithdrawalRecord,
    WithdrawalRecordResponse, WithdrawalRecordsResponse,
};
use crate::repositories::{withdrawal_history_repository, kyc_repository};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/withdrawals")
            .route("/record", web::post().to(record_withdrawal))
            .route("/history", web::get().to(get_withdrawal_history))
            .route("/delete", web::post().to(delete_withdrawal))
            .route("/single", web::get().to(get_single_withdrawal))
            .route("/update", web::post().to(update_withdrawal))
            .route("/user", web::get().to(get_withdrawal_history_by_user)),
    );
}

pub async fn record_withdrawal(
    db_pool: web::Data<Pool>,
    withdrawal_history_request: web::Json<CreateWithdrawalRecordRequest>,
) -> impl Responder {
    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    // Check KYC verification status before allowing withdrawal
    let user_id = &withdrawal_history_request.user_id.parse::<i32>().unwrap_or(0); // Convert string to i32
    match kyc_repository::is_kyc_verified(&client, user_id).await {
        Ok(is_verified) => {
            if !is_verified {
                return HttpResponse::Forbidden().json(json!({
                    "error": "KYC verification required",
                    "message": "You must complete KYC verification before making withdrawals."
                }));
            }
        },
        Err(e) => {
            eprintln!("Failed to check KYC verification status: {:?}", e);
            return HttpResponse::InternalServerError().json(json!({
                "error": "Failed to verify KYC status"
            }));
        }
    }

    // Proceed with withdrawal if KYC is verified
    match withdrawal_history_repository::record_withdrawal(
        &client,
        &withdrawal_history_request.into_inner(),
    )
    .await
    {
        Ok(activity) => HttpResponse::Created().json(activity),
        Err(e) => {
            eprintln!("Failed to create activity: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"error": "Failed to create activity"}))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(serde::Deserialize)]
pub struct GetSingleParams {
    pub id: Option<i64>,
}

pub async fn get_withdrawal_history(
    db_pool: web::Data<Pool>,
    query: web::Query<PaginationParams>,
) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);

    if page < 1 || page_size < 1 {
        return HttpResponse::BadRequest()
            .json(json!({"error": "Page and page_size must be positive integers"}));
    }

    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    match withdrawal_history_repository::get_withdrawal_history(&client, page, page_size).await {
        Ok((withdrawals, total)) => {
            let withdrawals_response: Vec<WithdrawalRecordResponse> = withdrawals
                .into_iter()
                .map(|withdrawal| WithdrawalRecordResponse {
                    id: withdrawal.id,
                    plan_id: withdrawal.plan_id,
                    wallet_id: withdrawal.wallet_id,
                    amount: withdrawal.amount,
                    payer_name: withdrawal.payer_name,
                    created_at: withdrawal.created_at.format("%d-%m-%Y").to_string(),
                })
                .collect();

            let response = WithdrawalRecordsResponse {
                records: withdrawals_response,
                total,
                page,
                page_size,
            };

            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            eprintln!("Failed to get user activities: {:?}", e);
            HttpResponse::InternalServerError()
                .json(json!({"error": "Failed to get user activities"}))
        }
    }
}

pub async fn delete_withdrawal(
    db_pool: web::Data<Pool>,
    withdrawal_history_request: web::Json<SingleWithdrawalRecordRequest>,
) -> impl Responder {
    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    match withdrawal_history_repository::delete_withdrawal(&client, withdrawal_history_request.id)
        .await
    {
        Ok(activity) => HttpResponse::Ok().json(activity),
        Err(e) => {
            eprintln!("Failed to create activity: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"error": "Failed to create activity"}))
        }
    }
}

pub async fn get_single_withdrawal(
    db_pool: web::Data<Pool>,
    query: web::Query<GetSingleParams>,
) -> impl Responder {
    let id = query.id.unwrap_or(0);
    if id < 1 {
        return HttpResponse::BadRequest().json(json!({"error": "ID must be a positive integer"}));
    }
    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    match withdrawal_history_repository::get_withdrawal_by_id(&client, id).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => {
            eprintln!("Failed to create activity: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"error": "Failed to create activity"}))
        }
    }
}

pub async fn update_withdrawal(
    db_pool: web::Data<Pool>,
    withdrawal_history_request: web::Json<WithdrawalRecord>,
) -> impl Responder {
    // let record: WithdrawalRecord = withdrawal_history_request.into_inner();

    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    match withdrawal_history_repository::update_withdrawal(
        &client,
        &withdrawal_history_request,
        // &record,
    )
    .await
    {
        Ok(activity) => HttpResponse::Ok().json(activity),
        Err(e) => {
            eprintln!("Failed to create activity: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"error": "Failed to create activity"}))
        }
    }
}

pub async fn get_withdrawal_history_by_user(
    db_pool: web::Data<Pool>,
    query: web::Query<GetSingleParams>,
) -> impl Responder {
    let uid = query.id.unwrap_or(0);
    if uid < 1 {
        return HttpResponse::BadRequest().json(json!({"error": "ID must be a positive integer"}));
    }

    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };
    match withdrawal_history_repository::get_withdrawal_history_by_user_id(&client, &uid).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => {
            eprintln!("Failed to create activity: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"error": "Failed to create activity"}))
        }
    }
}
