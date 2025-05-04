use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityLog {
    pub id: u32,
    pub user_id: String,
    pub action: String,
    pub timestamp: DateTime<Utc>,
    pub details: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserActivity {
    pub id: i32,
    pub user_id: String,
    pub date: DateTime<Utc>,
    pub activity_type: String,
    pub details: String,
    pub action_type: String,
    pub action_link: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserActivityRequest {
    pub user_id: String,
    pub activity_type: String,
    pub details: String,
    pub action_type: String,
    pub action_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserActivityResponse {
    pub id: i32,
    pub user_id: String,
    pub date: String, // Formatted as dd-mm-yyyy
    pub activity_type: String,
    pub details: String,
    pub action_type: String,
    pub action_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserActivitiesResponse {
    pub activities: Vec<UserActivityResponse>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}
