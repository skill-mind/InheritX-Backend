use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)] // Add Debug, Serialize, and Deserialize
pub struct ActivityLog {
    pub id: u32,
    pub token_from: String,
    pub token_to: String,
    pub amount: f64,
    pub timestamp: String,
}

pub fn create_activity_log(log: ActivityLog) -> String {
    // Logic to create a new activity log entry
    format!("Activity log created: {:?}", log)
}

pub fn get_activity_logs() -> Vec<ActivityLog> {
    // Logic to fetch all activity logs
    vec![]
}

pub fn update_activity_log(id: u32, log: ActivityLog) -> String {
    // Logic to update an activity log entry
    format!("Activity log with ID {} updated: {:?}", id, log)
}

pub fn delete_activity_log(id: u32) -> String {
    // Logic to delete an activity log entry
    format!("Activity log with ID {} deleted", id)
}