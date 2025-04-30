use crate::models::activity_models::{CreateUserActivityRequest, UserActivity};
use deadpool_postgres::Client;
use tokio_postgres::Error;

pub async fn create_activity(
    client: &Client,
    create_activity_request: &CreateUserActivityRequest,
) -> Result<UserActivity, Error> {
    let statement = client
        .prepare(
            "INSERT INTO user_activities (user_id, activity_type, details, action_type, action_link)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id, user_id, date, activity_type, details, action_type, action_link, created_at",
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

    Ok(UserActivity {
        id: row.get(0),
        user_id: row.get(1),
        date: row.get(2),
        activity_type: row.get(3),
        details: row.get(4),
        action_type: row.get(5),
        action_link: row.get(6),
        created_at: row.get(7),
    })
}

pub async fn get_user_activities(
    client: &Client,
    user_id: &str,
    page: i64,
    page_size: i64,
) -> Result<(Vec<UserActivity>, i64), Error> {
    // Get total count of activities for pagination
    let count_stmt = client
        .prepare("SELECT COUNT(*) FROM user_activities WHERE user_id = $1")
        .await?;

    let total_row = client.query_one(&count_stmt, &[&user_id]).await?;

    let total: i64 = total_row.get(0);

    // Get paginated activities
    let offset = (page - 1) * page_size;

    let stmt = client
        .prepare(
            "SELECT id, user_id, date, activity_type, details, action_type, action_link, created_at
             FROM user_activities
             WHERE user_id = $1
             ORDER BY date DESC
             LIMIT $2 OFFSET $3",
        )
        .await?;

    let rows = client
        .query(&stmt, &[&user_id, &page_size, &offset])
        .await?;

    let mut activities = Vec::new();

    for row in rows {
        activities.push(UserActivity {
            id: row.get(0),
            user_id: row.get(1),
            date: row.get(2),
            activity_type: row.get(3),
            details: row.get(4),
            action_type: row.get(5),
            action_link: row.get(6),
            created_at: row.get(7),
        });
    }

    Ok((activities, total))
}
