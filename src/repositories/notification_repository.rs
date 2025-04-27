use deadpool_postgres::Client;
use tokio_postgres::Error;
use crate::models::notification::{Notification, CreateNotification, UpdateNotification};
// use chrono::Utc;

pub async fn get_all(client: &Client) -> Result<Vec<Notification>, Error> {
    let stmt = client.prepare("SELECT id, title, body, is_read, created_at, updated_at FROM notifications").await?;
    let rows = client.query(&stmt, &[]).await?;
    
    Ok(rows.iter().map(|row| Notification {
        id: row.get(0),
        title: row.get(1),
        body: row.get(2),
        is_read: row.get(3),
        created_at: row.get(4),
        updated_at: row.get(5),
    }).collect())
}

pub async fn get_by_id(client: &Client, id: i32) -> Result<Notification, Error> {
    let stmt = client.prepare("SELECT id, title, body, is_read, created_at, updated_at FROM notifications WHERE id = $1").await?;
    let row = client.query_one(&stmt, &[&id]).await?;
    
    Ok(Notification {
        id: row.get(0),
        title: row.get(1),
        body: row.get(2),
        is_read: row.get(3),
        created_at: row.get(4),
        updated_at: row.get(5),
    })
}

pub async fn create(client: &Client, notification: &CreateNotification) -> Result<Notification, Error> {
    let stmt = client.prepare("
        INSERT INTO notifications (title, body) 
        VALUES ($1, $2) 
        RETURNING id, title, body, is_read, created_at, updated_at
    ").await?;
    
    let row = client.query_one(&stmt, &[&notification.title, &notification.body]).await?;
    
    Ok(Notification {
        id: row.get(0),
        title: row.get(1),
        body: row.get(2),
        is_read: row.get(3),
        created_at: row.get(4),
        updated_at: row.get(5),
    })
}

pub async fn update(client: &Client, id: i32, notification: &UpdateNotification) -> Result<Notification, Error> {
    let stmt = client.prepare("
        UPDATE notifications 
        SET 
            title = COALESCE($1, title), 
            body = COALESCE($2, body), 
            is_read = COALESCE($3, is_read),
            updated_at = NOW()
        WHERE id = $4
        RETURNING id, title, bod, is_read, created_at, updated_at
    ").await?;
    
    let row = client.query_one(&stmt, &[
        &notification.title, 
        &notification.body, 
        &notification.is_read, 
        &id
    ]).await?;
    
    Ok(Notification {
        id: row.get(0),
        title: row.get(1),
        body: row.get(2),
        is_read: row.get(3),
        created_at: row.get(4),
        updated_at: row.get(5),
    })
}

pub async fn delete(client: &Client, id: i32) -> Result<(), Error> {
    let stmt = client.prepare("DELETE FROM notifications WHERE id = $1").await?;
    client.execute(&stmt, &[&id]).await?;
    Ok(())
}

pub async fn mark_as_read(client: &Client, id: i32) -> Result<Notification, Error> {
    let stmt = client.prepare("
        UPDATE notifications 
        SET is_read = TRUE 
        WHERE id = $1 
        RETURNING id, title, body, is_read, created_at, updated_at
    ").await?;
    
    let row = client.query_one(&stmt, &[&id]).await?;
    
    Ok(Notification {
        id: row.get(0),
        title: row.get(1),
        body: row.get(2),
        is_read: row.get(3),
        created_at: row.get(4),
        updated_at: row.get(5),
    })
}