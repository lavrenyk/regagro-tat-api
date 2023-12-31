//! src/routes/subscriptions.rs

use actix_web::{web, HttpResponse};
// use chrono::Utc;
use sqlx::MySqlPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );
    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!("Saving new subscriber details in the database.");
    // let timestamp = Utc::now();
    // match sqlx::query!(
    //     r#"
    //     INSERT INTO subscriptions (id, email, name, subscribed_at)
    //     VALUES ($1, $2, $3, $4)
    //     "#,
    //     Uuid::new_v4(),
    //     form.email,
    //     form.name,
    //     timestamp
    // )
    match sqlx::query(
        r#"
            SELECT * FROM animals;
            "#,
    )
    // We use `get_ref` to get an immutable reference to the PgPool
    // wrapped by `web::Data`
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("{} - Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
