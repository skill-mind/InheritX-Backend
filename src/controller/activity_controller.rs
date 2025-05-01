use actix_web::{HttpResponse, Responder, web};
use deadpool_postgres::Pool;
use serde_json::json;

use crate::models::activity_models::{
    CreateUserActivityRequest, UserActivitiesResponse, UserActivityResponse,
};
use crate::repositories::activity_repository;

pub async fn create_user_activity(
    db_pool: web::Data<Pool>,
    activity_request: web::Json<CreateUserActivityRequest>,
) -> impl Responder {
    let client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to get DB client: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Database error"}));
        }
    };

    match activity_repository::create_activity(&client, &activity_request.into_inner()).await {
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

pub async fn get_user_activities_endpoint(
    db_pool: web::Data<Pool>,
    path: web::Path<String>,
    query: web::Query<PaginationParams>,
) -> impl Responder {
    let user_id = path.into_inner();
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

    match activity_repository::get_user_activities(&client, &user_id, page, page_size).await {
        Ok((activities, total)) => {
            let activities_response: Vec<UserActivityResponse> = activities
                .into_iter()
                .map(|activity| UserActivityResponse {
                    id: activity.id,
                    user_id: activity.user_id,
                    date: activity.date.format("%d-%m-%Y").to_string(),
                    activity_type: activity.activity_type,
                    details: activity.details,
                    action_type: activity.action_type,
                    action_link: activity.action_link,
                })
                .collect();

            let response = UserActivitiesResponse {
                activities: activities_response,
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
