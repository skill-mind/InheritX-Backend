use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Plan {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub multi_signature_approval: bool,
    pub required_approvals: i32,
    pub current_approvals: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreatePlanRequest {
    pub name: String,
    pub description: String,
    pub multi_signature_approval: bool,
    pub required_approvals: Option<i32>,
    pub approver_emails: Option<Vec<String>>,
}
