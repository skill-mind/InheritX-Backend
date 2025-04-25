use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Plan {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}
