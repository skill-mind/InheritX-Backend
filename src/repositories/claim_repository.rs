use crate::models::claim::{Claim, ClaimStatus, CreateClaim, UpdateClaim};
use deadpool_postgres::Client;
use tokio_postgres::Error;

pub async fn get_all(client: &Client) -> Result<Vec<Claim>, Error> {
    let stmt = client
        .prepare(
            "
        SELECT id, user_id, amount, status, description, created_at, updated_at 
        FROM claims
    ",
        )
        .await?;

    let rows = client.query(&stmt, &[]).await?;

    Ok(rows
        .iter()
        .map(|row| Claim {
            id: row.get(0),
            user_id: row.get(1),
            amount: row.get(2),
            status: row.get(3),
            description: row.get(4),
            created_at: row.get(5),
            updated_at: row.get(6),
        })
        .collect())
}

pub async fn get_by_user_id(client: &Client, user_id: i32) -> Result<Vec<Claim>, Error> {
    let stmt = client
        .prepare(
            "
        SELECT id, user_id, amount, status, description, created_at, updated_at 
        FROM claims 
        WHERE user_id = $1
    ",
        )
        .await?;

    let rows = client.query(&stmt, &[&user_id]).await?;

    Ok(rows
        .iter()
        .map(|row| Claim {
            id: row.get(0),
            user_id: row.get(1),
            amount: row.get(2),
            status: row.get(3),
            description: row.get(4),
            created_at: row.get(5),
            updated_at: row.get(6),
        })
        .collect())
}

pub async fn get_by_status(client: &Client, status: ClaimStatus) -> Result<Vec<Claim>, Error> {
    let stmt = client
        .prepare(
            "
        SELECT id, user_id, amount, status, description, created_at, updated_at 
        FROM claims 
        WHERE status = $1
    ",
        )
        .await?;

    let rows = client.query(&stmt, &[&status]).await?;

    Ok(rows
        .iter()
        .map(|row| Claim {
            id: row.get(0),
            user_id: row.get(1),
            amount: row.get(2),
            status: row.get(3),
            description: row.get(4),
            created_at: row.get(5),
            updated_at: row.get(6),
        })
        .collect())
}

pub async fn get_by_user_and_status(
    client: &Client,
    user_id: i32,
    status: ClaimStatus,
) -> Result<Vec<Claim>, Error> {
    let stmt = client
        .prepare(
            "
        SELECT id, user_id, amount, status, description, created_at, updated_at 
        FROM claims 
        WHERE user_id = $1 AND status = $2
    ",
        )
        .await?;

    let rows = client.query(&stmt, &[&user_id, &status]).await?;

    Ok(rows
        .iter()
        .map(|row| Claim {
            id: row.get(0),
            user_id: row.get(1),
            amount: row.get(2),
            status: row.get(3),
            description: row.get(4),
            created_at: row.get(5),
            updated_at: row.get(6),
        })
        .collect())
}

pub async fn create(client: &Client, claim: &CreateClaim) -> Result<Claim, Error> {
    let stmt = client
        .prepare(
            "
        INSERT INTO claims (user_id, amount, status, description) 
        VALUES ($1, $2, $3, $4) 
        RETURNING id, user_id, amount, status, description, created_at, updated_at
    ",
        )
        .await?;

    let row = client
        .query_one(
            &stmt,
            &[
                &claim.user_id,
                &claim.amount,
                &ClaimStatus::Pending,
                &claim.description,
            ],
        )
        .await?;

    Ok(Claim {
        id: row.get(0),
        user_id: row.get(1),
        amount: row.get(2),
        status: row.get(3),
        description: row.get(4),
        created_at: row.get(5),
        updated_at: row.get(6),
    })
}

pub async fn update(client: &Client, id: i32, claim: &UpdateClaim) -> Result<Claim, Error> {
    let stmt = client
        .prepare(
            "
        UPDATE claims 
        SET 
            status = COALESCE($1, status),
            description = COALESCE($2, description),
            updated_at = NOW()
        WHERE id = $3
        RETURNING id, user_id, amount, status, description, created_at, updated_at
    ",
        )
        .await?;

    let row = client
        .query_one(&stmt, &[&claim.status, &claim.description, &id])
        .await?;

    Ok(Claim {
        id: row.get(0),
        user_id: row.get(1),
        amount: row.get(2),
        status: row.get(3),
        description: row.get(4),
        created_at: row.get(5),
        updated_at: row.get(6),
    })
}
