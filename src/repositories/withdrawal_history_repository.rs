use crate::models::withdrawal_history_models::{CreateWithdrawalRecordRequest, WithdrawalRecord};
use deadpool_postgres::Client;
use tokio_postgres::Error;

pub async fn record_withdrawal(
    client: &Client,
    create_activity_request: &CreateWithdrawalRecordRequest,
) -> Result<WithdrawalRecord, Error> {
    let statement = client
        .prepare(
            "INSERT INTO withdrawal_history (plan_id, wallet_id, amount, payer_name)
             VALUES ($1, $2, $3, $4)
             RETURNING id, plan_id, wallet_id, amount, payer_name, created_at",
        )
        .await?;

    let row = client
        .query_one(
            &statement,
            &[
                &create_activity_request.user_id,
                &create_activity_request.activity_type,
                &create_activity_request.details,
                &create_activity_request.action_type,
                &create_activity_request.action_link,
            ],
        )
        .await?;

    Ok(WithdrawalRecord {
        id: row.get(0),
        plan_id: row.get(1),
        wallet_id: row.get(2),
        amount: row.get(3),
        payer_name: row.get(4),
        created_at: row.get(5),
    })
}

pub async fn get_withdrawal_history(
    client: &Client,
    page: i64,
    page_size: i64,
) -> Result<(Vec<WithdrawalRecord>, i64), Error> {
    // Get total count of records for pagination
    let count_stmt = client
        .prepare("SELECT COUNT(*) FROM withdrawal_history")
        .await?;

    // let total_row = client.query_one(&count_stmt, &[&user_id]).await?;
    let total_row = client.query_one(&count_stmt, &[]).await?;

    let total: i64 = total_row.get(0);

    let offset = (page - 1) * page_size;

    let stmt = client
        .prepare(
            "SELECT id, plan_id, wallet_id, amount, payer_name, created_at
             FROM withdrawal_history
             ORDER BY date DESC
             LIMIT $2 OFFSET $3",
        )
        .await?;

    let rows = client.query(&stmt, &[&page_size, &offset]).await?;

    let mut activities = Vec::new();

    for row in rows {
        activities.push(WithdrawalRecord {
            id: row.get(0),
            plan_id: row.get(1),
            wallet_id: row.get(2),
            amount: row.get(3),
            payer_name: row.get(4),
            created_at: row.get(5),
        });
    }

    Ok((activities, total))
}

pub async fn delete_withdrawal(client: &Client, id: i64) -> Result<(), Error> {
    let stmt = client
        .prepare("DELETE FROM withdrawal_history WHERE id = $1")
        .await?;

    client.execute(&stmt, &[&id]).await?;

    Ok(())
}

pub async fn get_withdrawal_by_id(client: &Client, id: i64) -> Result<WithdrawalRecord, Error> {
    let stmt = client
        .prepare("SELECT id, plan_id, wallet_id, amount, payer_name, created_at FROM withdrawal_history WHERE id = $1")
        .await?;

    let row = client.query_one(&stmt, &[&id]).await?;

    Ok(WithdrawalRecord {
        id: row.get(0),
        plan_id: row.get(1),
        wallet_id: row.get(2),
        amount: row.get(3),
        payer_name: row.get(4),
        created_at: row.get(5),
    })
}

pub async fn update_withdrawal(
    client: &Client,
    // id: i64,
    withdrawal_record: &WithdrawalRecord,
) -> Result<WithdrawalRecord, Error> {
    let stmt = client
        .prepare(
            "UPDATE withdrawal_history
             SET plan_id = $1, wallet_id = $2, amount = $3, payer_name = $4
             WHERE id = $5
             RETURNING id, plan_id, wallet_id, amount, payer_name, created_at",
        )
        .await?;

    let row = client
        .query_one(
            &stmt,
            &[
                &withdrawal_record.plan_id,
                &withdrawal_record.wallet_id,
                &withdrawal_record.amount,
                &withdrawal_record.payer_name,
                &withdrawal_record.id,
            ],
        )
        .await?;

    Ok(WithdrawalRecord {
        id: row.get(0),
        plan_id: row.get(1),
        wallet_id: row.get(2),
        amount: row.get(3),
        payer_name: row.get(4),
        created_at: row.get(5),
    })
}

pub async fn get_withdrawal_history_by_user_id(
    client: &Client,
    user_id: &i64,
) -> Result<Vec<WithdrawalRecord>, Error> {
    let stmt = client
        .prepare("SELECT id, plan_id, wallet_id, amount, payer_name, created_at FROM withdrawal_history WHERE user_id = $1")
        .await?;

    let rows = client.query(&stmt, &[&user_id]).await?;

    let mut activities = Vec::new();

    for row in rows {
        activities.push(WithdrawalRecord {
            id: row.get(0),
            plan_id: row.get(1),
            wallet_id: row.get(2),
            amount: row.get(3),
            payer_name: row.get(4),
            created_at: row.get(5),
        });
    }

    Ok(activities)
}
