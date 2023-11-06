//! src/routes/get_animal_by_period.rs

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub region_id: u32,
    pub date_reg_from: String,
    pub date_reg_to: String,
    pub kinds: String,
    pub districts: String,
}

pub async fn get_animal_by_period(
    data: web::Query<Field>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let query_span = tracing::info_span!("Requesting animals count in the database.");

    dbg!(&data);
    dbg!(&pool);

    // match sqlx::query(
    //     r#"
    //         SELECT * FROM animals;
    //         "#,
    // )
    // .execute(pool.get_ref())
    // .instrument(query_span)
    // .await
    // {
    //     Ok(data) => {
    //         dbg!(data);
    //         HttpResponse::Ok().finish()
    //     }
    //     Err(e) => {
    //         tracing::error!("{} - Failed to execute query: {:?}", request_id, e);
    //         HttpResponse::InternalServerError().finish()
    //     }
    // }

    HttpResponse::Ok().finish()
}
