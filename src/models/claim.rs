use bytes::BytesMut;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claim {
    pub id: i32,
    pub user_id: i32,
    pub amount: f64,
    pub status: ClaimStatus,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateClaim {
    pub user_id: i32,
    pub amount: f64,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateClaim {
    pub status: Option<ClaimStatus>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ClaimStatus {
    Pending,
    Approved,
    Rejected,
}

impl Display for ClaimStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ClaimStatus::Pending => write!(f, "pending"),
            ClaimStatus::Approved => write!(f, "approved"),
            ClaimStatus::Rejected => write!(f, "rejected"),
        }
    }
}

impl ToSql for ClaimStatus {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        let s = match self {
            ClaimStatus::Pending => "pending",
            ClaimStatus::Approved => "approved",
            ClaimStatus::Rejected => "rejected",
        };
        s.to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "claim_status"
    }

    fn to_sql_checked(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        self.to_sql(ty, out)
    }
}

impl<'a> FromSql<'a> for ClaimStatus {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<ClaimStatus, Box<dyn Error + Sync + Send>> {
        let s = <&str as FromSql>::from_sql(ty, raw)?;
        match s {
            "pending" => Ok(ClaimStatus::Pending),
            "approved" => Ok(ClaimStatus::Approved),
            "rejected" => Ok(ClaimStatus::Rejected),
            _ => Err(format!("invalid claim status: {}", s).into()),
        }
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "claim_status"
    }
}
