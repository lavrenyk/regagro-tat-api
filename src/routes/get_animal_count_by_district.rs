//! src/routes/api/analytics/animals/get_animal_count_by_district.rs
#![allow(unused_assignments)]
use crate::{
    helpers::{all_districts_filter, district_filter_query, get_district_names},
    structs::QueryData,
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, MySqlPool};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct ResponseItem {
    id: u64,
    name: String,
    count: i64,
    krs_count: i32,
    pig_count: i32,
    goat_count: i32,
    sheep_count: i32,
    horse_count: i32,
    bee_count: i32,
    dog_count: i32,
    cat_count: i32,
    deer_count: i32,
    maral_count: i32,
    camel_count: i32,
    donkey_count: i32,
    bison_count: i32,
}

/// Generate response JSON with data for any asked district of amount animals
/// with filtered by given period.
pub async fn get_animal_count_by_district(
    data: web::Query<QueryData>,
    _pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let _request_id = Uuid::new_v4();

    dbg!(&data);

    let districts = data.districts.clone().unwrap();

    let mut districts_filter = "".to_string();
    match &data.districts {
        Some(districts) => {
            districts_filter = district_filter_query(districts.as_str());
        }
        None => {
            districts_filter = all_districts_filter();
        }
    }

    let connection =
        MySqlPool::connect("mysql://mp_analytic:8Nlr7fDQNwmniu6h@vo.regagro.ru:33633/regagro_3_0")
            .await;

    let test_query = format!(
        r#"
        SELECT 
            a.id AS id,
            (CASE WHEN district_code IS NULL THEN locality_code ELSE district_code END) as name,
            COUNT(a.id) AS count,
            CAST(SUM(a.kind_id = 1) AS INTEGER) AS krs_count,
            CAST(SUM(a.kind_id = 2) AS INTEGER) AS pig_count,
            CAST(SUM(a.kind_id = 3) AS INTEGER) AS goat_count,
            CAST(SUM(a.kind_id = 4) AS INTEGER) AS sheep_count,
            CAST(SUM(a.kind_id = 5) AS INTEGER) AS horse_count,
            CAST(SUM(a.kind_id = 6) AS INTEGER) AS bee_count,
            CAST(SUM(a.kind_id = 7) AS INTEGER) AS dog_count,
            CAST(SUM(a.kind_id = 8) AS INTEGER) AS cat_count,
            CAST(SUM(a.kind_id = 9) AS INTEGER) AS deer_count,
            CAST(SUM(a.kind_id = 10) AS INTEGER) AS maral_count,
            CAST(SUM(a.kind_id = 11) AS INTEGER) AS camel_count,
            CAST(SUM(a.kind_id = 12) AS INTEGER) AS donkey_count,
            CAST(SUM(a.kind_id = 13) AS INTEGER) AS bison_count

        FROM enterprise_addresses AS ea
        LEFT JOIN enterprises AS e ON ea.enterprise_id = e.id
        LEFT JOIN animals AS a ON e.id = a.enterprise_id

        WHERE ea.district_code IN ({})
        OR ea.locality_code IN ({})
        AND a.created_at > '2023-01-01'
        AND a.created_at < '2023-12-31'

        GROUP BY name
    "#,
        &districts_filter, &districts_filter
    );

    dbg!(&test_query);

    let mut response: Vec<ResponseItem> = vec![];

    match connection {
        Err(err) => {
            println!("Cannot connect to database [{}]", err.to_string());
        }
        Ok(pool) => {
            println!("Connected to database successfully.");
            let result_all: Result<Vec<ResponseItem>, _> =
                sqlx::query_as(&test_query).fetch_all(&pool).await;
            dbg!(&result_all);
            response = result_all.unwrap();
        }
    }

    // Обновляем id и имя в результатах запроса
    let district_codes: Vec<&str> = districts.split(",").collect();
    let district_names = get_district_names(&districts.as_str());
    let district_names: Vec<&str> = district_names.split(",").collect();

    for (index, item) in district_codes.iter().enumerate() {
        response[index].id = item.parse::<u64>().unwrap();
    }

    for (index, item) in district_names.iter().enumerate() {
        response[index].name = item.to_string();
    }

    let json_response = json!(response);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json_response)
}
