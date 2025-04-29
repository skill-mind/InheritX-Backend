use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Approval {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub approver_id: Uuid,
    pub approver_email: String,
    pub status: ApprovalStatus,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
}

impl std::fmt::Display for ApprovalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApprovalStatus::Pending => write!(f, "pending"),
            ApprovalStatus::Approved => write!(f, "approved"),
            ApprovalStatus::Rejected => write!(f, "rejected"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApproverRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub plan_id: Uuid,
    pub approver_emails: Vec<String>,
    pub required_approvals: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApprovalResponse {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub approver_email: String,
    pub status: ApprovalStatus,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApprovalStatusResponse {
    pub plan_id: Uuid,
    pub required_approvals: i32,
    pub current_approvals: i32,
    pub approvers: Vec<ApproverStatus>,
    pub can_execute: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApproverStatus {
    pub email: String,
    pub status: ApprovalStatus,
    pub approved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitApprovalRequest {
    pub approval_id: Uuid,
    pub approve: bool,
}