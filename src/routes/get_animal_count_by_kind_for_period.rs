//! src/routes/api/analytics/animals/get_animal_by_kind_for_period.rs

use crate::helpers::animals_filter_query;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryData {
    pub region_id: Option<u32>,
    pub date_reg_from: Option<String>,
    pub date_reg_to: Option<String>,
    pub kinds: Option<String>,
    pub districts: Option<String>,
}

/// Generate response JSON with data for any asked kind of animal in separated data
/// with filtered by given period and districts.
///
/// Data for any given kind of animal is returned in the fallowing format:
/// * `[
/// *    {
/// *        "count": 31990, // u32 - amount of the filtered animal kinds in db
/// *        "kind_id": 1,  // u8 - animal kind number
/// *        "view": "КРС", // String - animal kind short name
/// *        "name": "Крупный рогатый скот" // String - animal kind full name
/// *    },
/// *    ... // other asked kinds of animals
/// * ]`
pub async fn get_animal_count_by_kind_for_period(
    data: web::Query<QueryData>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let query_span = tracing::info_span!("Requesting animals count by kinds for period");

    dbg!(&data);

    // ЭТАП 1: Переформатируем QueryData в необходимые данные для работы
    // 1. Определяем количество видов в запросе. Для этого переводим данные QueryData.kinds
    // из [`String`] в [`Vec<u8>`]

    match &data.kinds {
        Some(kinds) => {
            let filter = animals_filter_query(kinds);
            dbg!(filter);
        }
        None => println!("NOTHING!"),
    }

    // 2. Переводим QueryData.districts из [`String`] в [`Vec<String>`] с параллельной конвертацией
    // числового значения в GUID региона.

    match sqlx::query(
        r#"
        SELECT * FROM enterprises LIMIT 10;
        "#,
    )
    .fetch_all(pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(data) => {
            dbg!(data.len());
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("{} - Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }

    // HttpResponse::Ok().finish()
}
