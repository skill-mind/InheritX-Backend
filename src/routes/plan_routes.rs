use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::plan_models::{Plan, CreatePlanRequest};

async fn create_plan(plan: web::Json<CreatePlanRequest>) -> impl Responder {
    let plan = Plan {
        id: Uuid::new_v4(),
        name: plan.name.clone(),
        description: plan.description.clone(),
        multi_signature_approval: plan.multi_signature_approval,
        required_approvals: plan.required_approvals.unwrap_or(0),
        current_approvals: 0,
    };

    HttpResponse::Created().json(plan)
}

async fn get_plan_by_id(plan_id: web::Path<Uuid>) -> impl Responder {
    let mock_plan = Plan {
        id: *plan_id,
        name: "Sample Plan".to_string(),
        description: "This is a sample plan.".to_string(),
        multi_signature_approval: false,
        required_approvals: 3,
        current_approvals: 0,
    };

    HttpResponse::Ok().json(mock_plan)
}

async fn update_plan(plan_id: web::Path<Uuid>, updated_plan: web::Json<CreatePlanRequest>) -> impl Responder {
    let plan = Plan {
        id: *plan_id,
        name: updated_plan.name.clone(),
        description: updated_plan.description.clone(),
        multi_signature_approval: updated_plan.multi_signature_approval,
        required_approvals: updated_plan.required_approvals.unwrap_or(0),
        current_approvals: 0,
    };

    HttpResponse::Ok().json(plan)
}

async fn delete_plan(_plan_id: web::Path<Uuid>) -> impl Responder {
    HttpResponse::NoContent().finish()
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/plans")
        .route(web::post().to(create_plan))
    )
    .service(web::resource("/plans/{plan_id}")
        .route(web::get().to(get_plan_by_id))
        .route(web::put().to(update_plan))  
        .route(web::delete().to(delete_plan))
    );
}
