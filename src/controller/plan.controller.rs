use crate::models::plan_model::Plan;
use std::sync::Mutex;
use uuid::Uuid;
use lazy_static::lazy_static;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::postgres::Postgres;

lazy_static! {
    static ref PLAN_STORE: Mutex<Vec<Plan>> = Mutex::new(Vec::new());
}

pub fn create_plan(name: String, description: String) -> Plan {
    let plan = Plan {
        id: Uuid::new_v4(),
        name,
        description,
    };
    PLAN_STORE.lock().unwrap().push(plan.clone());
    plan
}

pub fn get_plan_by_id(id: Uuid) -> Option<Plan> {
    PLAN_STORE.lock().unwrap().iter().cloned().find(|p| p.id == id)
}

pub fn get_all_plans() -> Vec<Plan> {
    PLAN_STORE.lock().unwrap().clone()
}

pub fn update_plan(id: Uuid, name: String, description: String) -> Option<Plan> {
    let mut store = PLAN_STORE.lock().unwrap();
    if let Some(plan) = store.iter_mut().find(|p| p.id == id) {
        plan.name = name;
        plan.description = description;
        return Some(plan.clone());
    }
    None
}

pub fn delete_plan(id: Uuid) -> bool {
    let mut store = PLAN_STORE.lock().unwrap();
    let len_before = store.len();
    store.retain(|p| p.id != id);
    len_before != store.len()
}

pub async fn execute_plan(
    pool: web::Data<Pool<Postgres>>,
    user_id: web::ReqData<Uuid>,
    req: web::Json<ExecutePlanRequest>,
) -> impl Responder {
    let plan_result = sqlx::query!(
        r#"
        SELECT id, multi_signature_approval, required_approvals, current_approvals 
        FROM plans 
        WHERE id = $1 AND user_id = $2
        "#,
        req.plan_id,
        *user_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match plan_result {
        Ok(Some(plan)) => {
            if plan.multi_signature_approval {
                if plan.current_approvals < plan.required_approvals {
                    return HttpResponse::BadRequest().json(serde_json::json!({
                        "message": "Insufficient approvals to execute plan",
                        "required": plan.required_approvals,
                        "current": plan.current_approvals
                    }));
                }
            }

            let result = sqlx::query!(
                r#"
                UPDATE plans 
                SET status = 'executed', executed_at = $1, updated_at = $1
                WHERE id = $2
                "#,
                Utc::now(),
                req.plan_id
            )
            .execute(pool.get_ref())
            .await;

            match result {
                Ok(_) => HttpResponse::Ok().json(serde_json::json!({
                    "message": "Plan executed successfully"
                })),
                Err(_) => HttpResponse::InternalServerError().json("Failed to execute plan"),
            }
        }
        Ok(None) => HttpResponse::NotFound().json("Plan not found"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch plan"),
    }
}
