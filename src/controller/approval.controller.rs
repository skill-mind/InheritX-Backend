use crate::models::approval::{
    Approval, ApprovalRequest, ApprovalResponse, ApprovalStatus, ApprovalStatusResponse,
    ApproverStatus, SubmitApprovalRequest,
};
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

// Request approvals for a plan
pub async fn request_approvals(
    pool: web::Data<Pool<Postgres>>,
    user_id: web::ReqData<Uuid>,
    req: web::Json<ApprovalRequest>,
) -> impl Responder {
    // Check if plan exists and belongs to user
    let plan_result = sqlx::query!(
        r#"
        SELECT id, multi_signature_approval FROM plans 
        WHERE id = $1 AND user_id = $2
        "#,
        req.plan_id,
        *user_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match plan_result {
        Ok(Some(plan)) => {
            // Enable multi-signature approval for the plan
            let update_result = sqlx::query!(
                r#"
                UPDATE plans 
                SET multi_signature_approval = true, 
                    required_approvals = $1,
                    updated_at = $2
                WHERE id = $3
                "#,
                req.required_approvals,
                Utc::now(),
                req.plan_id
            )
            .execute(pool.get_ref())
            .await;

            if let Err(_) = update_result {
                return HttpResponse::InternalServerError().json("Failed to update plan");
            }

            // Create approval requests for each approver
            let mut tx = match pool.begin().await {
                Ok(tx) => tx,
                Err(_) => return HttpResponse::InternalServerError().json("Failed to start transaction"),
            };

            // First, delete any existing approvals
            let delete_result = sqlx::query!(
                r#"
                DELETE FROM approvals 
                WHERE plan_id = $1
                "#,
                req.plan_id
            )
            .execute(&mut tx)
            .await;

            if let Err(_) = delete_result {
                let _ = tx.rollback().await;
                return HttpResponse::InternalServerError().json("Failed to clear existing approvals");
            }

            // Create new approval requests
            let now = Utc::now();
            for email in &req.approver_emails {
                let approval_id = Uuid::new_v4();
                let result = sqlx::query!(
                    r#"
                    INSERT INTO approvals (id, plan_id, approver_email, status, created_at, updated_at)
                    VALUES ($1, $2, $3, $4, $5, $5)
                    "#,
                    approval_id,
                    req.plan_id,
                    email,
                    ApprovalStatus::Pending.to_string(),
                    now
                )
                .execute(&mut tx)
                .await;

                if let Err(_) = result {
                    let _ = tx.rollback().await;
                    return HttpResponse::InternalServerError().json("Failed to create approval request");
                }
            }

            // Commit transaction
            if let Err(_) = tx.commit().await {
                return HttpResponse::InternalServerError().json("Failed to commit transaction");
            }

            HttpResponse::Ok().json(serde_json::json!({
                "message": "Approval requests sent successfully",
                "plan_id": req.plan_id,
                "required_approvals": req.required_approvals,
                "approver_count": req.approver_emails.len()
            }))
        }
        Ok(None) => HttpResponse::NotFound().json("Plan not found"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch plan"),
    }
}

// Get approval status for a plan
pub async fn get_approval_status(
    pool: web::Data<Pool<Postgres>>,
    user_id: web::ReqData<Uuid>,
    plan_id: web::Path<Uuid>,
) -> impl Responder {
    // Check if plan exists and belongs to user
    let plan_result = sqlx::query!(
        r#"
        SELECT id, multi_signature_approval, required_approvals 
        FROM plans 
        WHERE id = $1 AND user_id = $2
        "#,
        *plan_id,
        *user_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match plan_result {
        Ok(Some(plan)) => {
            if !plan.multi_signature_approval {
                return HttpResponse::BadRequest().json("Multi-signature approval not enabled for this plan");
            }

            // Get all approvals for the plan
            let approvals_result = sqlx::query!(
                r#"
                SELECT approver_email, status, approved_at 
                FROM approvals 
                WHERE plan_id = $1
                "#,
                *plan_id
            )
            .fetch_all(pool.get_ref())
            .await;

            match approvals_result {
                Ok(approvals) => {
                    // Count approved approvals
                    let approved_count = approvals
                        .iter()
                        .filter(|a| a.status == ApprovalStatus::Approved.to_string())
                        .count() as i32;

                    // Create response
                    let approvers: Vec<ApproverStatus> = approvals
                        .into_iter()
                        .map(|a| ApproverStatus {
                            email: a.approver_email,
                            status: match a.status.as_str() {
                                "approved" => ApprovalStatus::Approved,
                                "rejected" => ApprovalStatus::Rejected,
                                _ => ApprovalStatus::Pending,
                            },
                            approved_at: a.approved_at,
                        })
                        .collect();

                    let can_execute = approved_count >= plan.required_approvals;

                    let response = ApprovalStatusResponse {
                        plan_id: *plan_id,
                        required_approvals: plan.required_approvals,
                        current_approvals: approved_count,
                        approvers,
                        can_execute,
                    };

                    HttpResponse::Ok().json(response)
                }
                Err(_) => HttpResponse::InternalServerError().json("Failed to fetch approvals"),
            }
        }
        Ok(None) => HttpResponse::NotFound().json("Plan not found"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch plan"),
    }
}

// Get approval by ID (for approvers to view the request)
pub async fn get_approval(
    pool: web::Data<Pool<Postgres>>,
    approval_id: web::Path<Uuid>,
) -> impl Responder {
    let approval_result = sqlx::query_as!(
        Approval,
        r#"
        SELECT id, plan_id, approver_id, approver_email, 
               status as "status: _", approved_at, created_at, updated_at
        FROM approvals 
        WHERE id = $1
        "#,
        *approval_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match approval_result {
        Ok(Some(approval)) => {
            // Get plan details
            let plan_result = sqlx::query!(
                r#"
                SELECT name, description 
                FROM plans 
                WHERE id = $1
                "#,
                approval.plan_id
            )
            .fetch_optional(pool.get_ref())
            .await;

            match plan_result {
                Ok(Some(plan)) => {
                    let response = serde_json::json!({
                        "approval": ApprovalResponse {
                            id: approval.id,
                            plan_id: approval.plan_id,
                            approver_email: approval.approver_email,
                            status: approval.status,
                            approved_at: approval.approved_at,
                            created_at: approval.created_at,
                        },
                        "plan": {
                            "name": plan.name,
                            "description": plan.description,
                        }
                    });

                    HttpResponse::Ok().json(response)
                }
                _ => HttpResponse::InternalServerError().json("Failed to fetch plan details"),
            }
        }
        Ok(None) => HttpResponse::NotFound().json("Approval request not found"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch approval"),
    }
}

// Submit approval (approve or reject)
pub async fn submit_approval(
    pool: web::Data<Pool<Postgres>>,
    req: web::Json<SubmitApprovalRequest>,
) -> impl Responder {
    let approval_result = sqlx::query!(
        r#"
        SELECT id, plan_id, approver_email 
        FROM approvals 
        WHERE id = $1
        "#,
        req.approval_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match approval_result {
        Ok(Some(approval)) => {
            let now = Utc::now();
            let status = if req.approve {
                ApprovalStatus::Approved
            } else {
                ApprovalStatus::Rejected
            };

            // Update approval status
            let update_result = sqlx::query!(
                r#"
                UPDATE approvals 
                SET status = $1, approved_at = $2, updated_at = $2
                WHERE id = $3
                RETURNING id
                "#,
                status.to_string(),
                now,
                req.approval_id
            )
            .fetch_one(pool.get_ref())
            .await;

            match update_result {
                Ok(_) => {
                    // If approved, update the current_approvals count in the plan
                    if req.approve {
                        let _ = sqlx::query!(
                            r#"
                            UPDATE plans 
                            SET current_approvals = (
                                SELECT COUNT(*) FROM approvals 
                                WHERE plan_id = $1 AND status = 'approved'
                            ),
                            updated_at = $2
                            WHERE id = $1
                            "#,
                            approval.plan_id,
                            now
                        )
                        .execute(pool.get_ref())
                        .await;
                    }

                    HttpResponse::Ok().json(serde_json::json!({
                        "message": if req.approve { "Plan approved successfully" } else { "Plan rejected" },
                        "approval_id": req.approval_id,
                        "plan_id": approval.plan_id,
                        "status": status.to_string()
                    }))
                }
                Err(_) => HttpResponse::InternalServerError().json("Failed to update approval status"),
            }
        }
        Ok(None) => HttpResponse::NotFound().json("Approval request not found"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch approval"),
    }
}