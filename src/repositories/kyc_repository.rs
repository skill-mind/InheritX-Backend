use chrono::Utc;
use deadpool_postgres::Client;
use tokio_postgres::error::Error;

use crate::models::kyc_models::{CreateKycRequest, KycRecord};

pub async fn create_kyc(client: &Client, kyc_request: &CreateKycRequest) -> Result<KycRecord, Error> {
    let statement = client
        .prepare(
            "INSERT INTO kyc_records (user_id, full_name, date_of_birth, id_type, id_number, address, verification_status, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, user_id, full_name, date_of_birth, id_type, id_number, address, verification_status, created_at, updated_at",
        )
        .await?;

    // Default verification status is "pending"
    let verification_status = "pending".to_string();
    let created_at = Utc::now();

    let row = client
        .query_one(
            &statement,
            &[
                &kyc_request.user_id,
                &kyc_request.full_name,
                &kyc_request.date_of_birth,
                &kyc_request.id_type,
                &kyc_request.id_number,
                &kyc_request.address,
                &verification_status,
                &created_at,
            ],
        )
        .await?;

    Ok(KycRecord {
        id: row.get(0),
        user_id: row.get(1),
        full_name: row.get(2),
        date_of_birth: row.get(3),
        id_type: row.get(4),
        id_number: row.get(5),
        address: row.get(6),
        verification_status: row.get(7),
        created_at: row.get(8),
        updated_at: row.get(9),
    })
}

pub async fn update_kyc_verification_status(
    client: &Client,
    kyc_id: i32,
    verification_status: &str,
) -> Result<KycRecord, Error> {
    let statement = client
        .prepare(
            "UPDATE kyc_records
            SET verification_status = $1, updated_at = $2
            WHERE id = $3
            RETURNING id, user_id, full_name, date_of_birth, id_type, id_number, address, verification_status, created_at, updated_at",
        )
        .await?;

    let updated_at = Utc::now();

    let row = client
        .query_one(&statement, &[&verification_status, &updated_at, &kyc_id])
        .await?;

    Ok(KycRecord {
        id: row.get(0),
        user_id: row.get(1),
        full_name: row.get(2),
        date_of_birth: row.get(3),
        id_type: row.get(4),
        id_number: row.get(5),
        address: row.get(6),
        verification_status: row.get(7),
        created_at: row.get(8),
        updated_at: row.get(9),
    })
}

pub async fn get_kyc_by_id(client: &Client, kyc_id: i32) -> Result<KycRecord, Error> {
    let statement = client
        .prepare(
            "SELECT id, user_id, full_name, date_of_birth, id_type, id_number, address, verification_status, created_at, updated_at
            FROM kyc_records
            WHERE id = $1",
        )
        .await?;

    let row = client.query_one(&statement, &[&kyc_id]).await?;

    Ok(KycRecord {
        id: row.get(0),
        user_id: row.get(1),
        full_name: row.get(2),
        date_of_birth: row.get(3),
        id_type: row.get(4),
        id_number: row.get(5),
        address: row.get(6),
        verification_status: row.get(7),
        created_at: row.get(8),
        updated_at: row.get(9),
    })
}

pub async fn get_kyc_by_user_id(client: &Client, user_id: i32) -> Result<KycRecord, Error> {
    let statement = client
        .prepare(
            "SELECT id, user_id, full_name, date_of_birth, id_type, id_number, address, verification_status, created_at, updated_at
            FROM kyc_records
            WHERE user_id = $1",
        )
        .await?;

    let row = client.query_one(&statement, &[&user_id]).await?;

    Ok(KycRecord {
        id: row.get(0),
        user_id: row.get(1),
        full_name: row.get(2),
        date_of_birth: row.get(3),
        id_type: row.get(4),
        id_number: row.get(5),
        address: row.get(6),
        verification_status: row.get(7),
        created_at: row.get(8),
        updated_at: row.get(9),
    })
}

pub async fn is_kyc_verified(client: &Client, user_id: &i32) -> Result<bool, Error> {
    let statement = client
        .prepare(
            "SELECT verification_status 
            FROM kyc_records 
            WHERE user_id = $1",
        )
        .await?;

    match client.query_opt(&statement, &[user_id]).await? {
        Some(row) => {
            let status: String = row.get(0);
            Ok(status == "verified")
        }
        None => Ok(false), // User has no KYC record, so not verified
    }
} 