use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::plan_models::Plan;

async fn create_plan(plan: web::Json<Plan>) -> impl Responder {
    HttpResponse::Created().json(plan.into_inner())
}

async fn get_plan_by_id(plan_id: web::Path<Uuid>) -> impl Responder {
    let mock_plan = Plan {
        id: *plan_id,
        name: "Sample Plan".to_string(),
        description: "This is a sample plan.".to_string(),
    };

    HttpResponse::Ok().json(mock_plan)
}

async fn update_plan(plan_id: web::Path<Uuid>, updated_plan: web::Json<Plan>) -> impl Responder {
    let mut plan = updated_plan.into_inner();
    plan.id = *plan_id;
    HttpResponse::Ok().json(plan)
}

async fn delete_plan(_plan_id: web::Path<Uuid>) -> impl Responder {
    HttpResponse::NoContent().finish()
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/plans")
        .route(web::post().to(create_plan))
    )
    .service(web::resource("/plans/{plan_id}")
        .route(web::get().to(get_plan_by_id))
        .route(web::put().to(update_plan))  
        .route(web::delete().to(delete_plan))
    );
}
