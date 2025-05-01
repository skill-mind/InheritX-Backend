use actix_web::{HttpResponse, Responder, web};
use deadpool_postgres::Pool;
use serde_json::json;

use crate::models::withdrawal_history_models::{
    CreateWithdrawalRecordRequest, WithdrawalRecordResponse, WithdrawalRecordsResponse,
};
use crate::repositories::withdrawal_history_repository;

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

pub async fn get_withdrawal_history(
    db_pool: web::Data<Pool>,
    path: web::Path<String>,
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
