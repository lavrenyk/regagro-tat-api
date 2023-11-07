//! src/routes/api/analytics/animals/get_animal_by_kind_for_period.rs

use crate::helpers::{all_districts_filter, animals_filter_query, district_filter_query};
use actix_web::{web, HttpResponse};
use async_std::stream::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{query_builder::QueryBuilder, Column, Execute, MySqlConnection, MySqlPool, Row};
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

    // ЭТАП 1: Переформатируем QueryData в необходимые данные для работы
    // 1. Определяем количество видов в запросе. Для этого переводим данные QueryData.kinds
    // из [`String`] в [`Vec<u8>`]. Дополнительно проверяем, были ли данные по в запросе по
    // видам животных, если не было, то выбираем все типы животных
    let mut animal_kinds: Vec<&str> = vec![];

    match &data.kinds {
        Some(kinds) => {
            animal_kinds = kinds.split(",").collect();
        }
        // If no parameter was send selecting all animals kinds
        None => {
            animal_kinds = "1,2,3,4,5,6,7,8,9,10,11,12,13".split(",").collect();
        }
    }

    // 2. Переводим QueryData.districts из [`String`] в [`Vec<String>`] с параллельной конвертацией
    // числового значения в GUID региона и готовой части фильтра для запроса. Если в запросе нет данных,
    // то выбираем все регионы
    let mut districts_filter = "".to_string();
    match &data.districts {
        Some(districts) => {
            districts_filter = district_filter_query(districts.as_str());
        }
        None => {
            districts_filter = all_districts_filter();
        }
    }

    let query = format!("SELECT id FROM animals LIMIT 10");

    let result = sqlx::query(
        r#"
            SELECT COUNT(*) as count FROM animals;
        "#,
    )
    .fetch_all(pool.as_ref())
    .await;

    let result2: Result<(i64,), _> = sqlx::query_as(
        r#"
            SELECT COUNT(*) as count FROM animals;
        "#,
    )
    .fetch_one(pool.as_ref())
    .await;

    // let new_res: u64 = &result.unwrap()[0].try_get("count");
    // dbg!(&result.unwrap()[0].try_get::<u64, _>("count"));
    dbg!(&result2.unwrap().0);

    // match sqlx::query(&query)
    //     .fetch_all(pool.as_ref())
    //     // .instrument(query_span)
    //     .await
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
