use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawalRecord {
    pub id: i64,
    pub plan_id: String,
    // pub date: DateTime<Utc>,
    pub wallet_id: String,
    pub amount: i64,
    pub payer_name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWithdrawalRecordRequest {
    pub user_id: String,
    pub activity_type: String,
    pub details: String,
    pub action_type: String,
    pub action_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleWithdrawalRecordRequest {
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawalRecordResponse {
    pub id: i64,
    pub plan_id: String,
    pub wallet_id: String,
    pub amount: i64,
    pub payer_name: String,
    pub created_at: String, // Formatted as dd-mm-yyyy
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawalRecordsResponse {
    pub records: Vec<WithdrawalRecordResponse>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}
