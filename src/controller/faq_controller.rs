use crate::models::faq::FAQ;
use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use deadpool_postgres::Pool;
use serde_json::json;
use tokio_postgres::Row;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/faqs")
            .route("", web::get().to(get_faqs))
            .route("", web::post().to(create_faq))
            .route("/{id}", web::put().to(update_faq))
            .route("/{id}", web::delete().to(delete_faq)),
    );
}

pub async fn get_faqs(pool: web::Data<Pool>) -> impl Responder {
    let client = pool.get().await.unwrap();

    let rows = client
        .query(
            "SELECT id, question, answer, created_at, updated_at FROM faqs",
            &[],
        )
        .await
        .unwrap();

    let faqs: Vec<FAQ> = rows.iter().map(|row| row_to_faq(row)).collect();

    HttpResponse::Ok().json(faqs)
}

pub async fn create_faq(pool: web::Data<Pool>, faq: web::Json<FAQ>) -> impl Responder {
    let client = pool.get().await.unwrap();

    let row = client
        .query_one(
            "INSERT INTO faqs (question, answer, created_at, updated_at) VALUES ($1, $2, $3, $4) RETURNING id, question, answer, created_at, updated_at",
            &[&faq.question, &faq.answer, &Utc::now(), &Utc::now()],
        )
        .await
        .unwrap();

    let new_faq = row_to_faq(&row);

    HttpResponse::Created().json(json!({"message": "FAQ created", "data": new_faq}))
}

pub async fn update_faq(
    pool: web::Data<Pool>,
    path: web::Path<i32>,
    faq: web::Json<FAQ>,
) -> impl Responder {
    let id = path.into_inner();
    let client = pool.get().await.unwrap();

    let row = client
        .query_one(
            "UPDATE faqs SET question = $1, answer = $2, updated_at = $3 WHERE id = $4 RETURNING id, question, answer, created_at, updated_at",
            &[&faq.question, &faq.answer, &Utc::now(), &id],
        )
        .await
        .unwrap();

    let updated_faq = row_to_faq(&row);

    HttpResponse::Ok().json(json!({"message": "FAQ updated", "data": updated_faq}))
}

pub async fn delete_faq(pool: web::Data<Pool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let client = pool.get().await.unwrap();

    client
        .execute("DELETE FROM faqs WHERE id = $1", &[&id])
        .await
        .unwrap();

    HttpResponse::Ok().json(json!({"message": format!("FAQ with id {} deleted", id)}))
}

fn row_to_faq(row: &Row) -> FAQ {
    FAQ {
        id: row.get("id"),
        question: row.get("question"),
        answer: row.get("answer"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}
