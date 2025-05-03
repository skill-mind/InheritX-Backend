use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KycRecord {
    pub id: i32,
    pub user_id: i32,
    pub full_name: String,
    pub date_of_birth: String,
    pub id_type: String,
    pub id_number: String,
    pub address: String,
    pub verification_status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateKycRequest {
    pub user_id: i32,
    pub full_name: String,
    pub date_of_birth: String,
    pub id_type: String,
    pub id_number: String,
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KycRecordResponse {
    pub id: i32,
    pub user_id: i32,
    pub full_name: String,
    pub date_of_birth: String,
    pub id_type: String,
    pub id_number: String,
    pub address: String,
    pub verification_status: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KycVerificationRequest {
    pub id: i32,
    pub verification_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KycQueryById {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KycQueryByUserId {
    pub user_id: i32,
}
