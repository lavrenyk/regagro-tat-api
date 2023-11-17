//! src/routes/api/analytics/animals/get_animal_count_by_district.rs
#![allow(unused_assignments)]
use crate::{
    helpers::{all_districts_filter, district_filter_query, get_district_name_by_id},
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

    // dbg!(&data);

    let mut districts_filter = "".to_string();
    match &data.districts {
        Some(data) => {
            districts_filter = district_filter_query(&data.as_str());
            // districts = data.to_string();
        }
        None => {
            districts_filter = all_districts_filter();
        }
    }

    let mut filter_date_from = "2023-01-01".to_string();
    let mut filter_date_to = "2023-12-31".to_string();

    match &data.date_reg_from {
        Some(date_from) => {
            filter_date_from = date_from.to_string();
        }
        None => (),
    }

    match &data.date_reg_to {
        Some(date_to) => {
            filter_date_to = date_to.to_string();
        }
        None => (),
    }

    let connection =
        MySqlPool::connect("mysql://mp_analytic:8Nlr7fDQNwmniu6h@vo.regagro.ru:33633/regagro_3_0")
            .await;

    let test_query = format!(
        r#"
        SELECT 
            ea.id AS id,
            (CASE WHEN district_code IS NULL THEN locality_code ELSE district_code END) as name,
            COUNT(a.id) AS count,
            CAST( SUM(CASE WHEN `a`.`kind_id` = 1 THEN `a`.`count` ELSE 0 END) AS INTEGER) as `krs_count`,
            CAST( SUM(CASE WHEN `a`.`kind_id` = 2 THEN `a`.`count` ELSE 0 END) AS INTEGER) as `pig_count`,
            CAST( SUM(CASE WHEN `a`.`kind_id` = 12 THEN `a`.`count` ELSE 0 END) AS INTEGER) as `goat_count`,
            CAST( SUM(CASE WHEN `a`.`kind_id` = 13 THEN `a`.`count` ELSE 0 END) AS INTEGER) as `sheep_count`,
            CAST( SUM(CASE WHEN `a`.`kind_id` = 3 THEN `a`.`count` ELSE 0 END) AS INTEGER) as `horse_count`,
            CAST( SUM(CASE WHEN `a`.`kind_id` = 4 THEN `a`.`count` ELSE 0 END) AS INTEGER) as `bee_count`,
            CAST( SUM(CASE WHEN `a`.`kind_id` = 5 THEN `a`.`count` ELSE 0 END) AS INTEGER) as `dog_count`,
            CAST( SUM(CASE WHEN `a`.`kind_id` = 6 THEN `a`.`count` ELSE 0 END) AS INTEGER) as `cat_count`,
            CAST( SUM(CASE WHEN `a`.`kind_id` = 7 THEN `a`.`count` ELSE 0 END) AS INTEGER) as `deer_count`,
            CAST( SUM(CASE WHEN `a`.`kind_id` = 8 THEN `a`.`count` ELSE 0 END) AS INTEGER) as `maral_count`,
            CAST( SUM(CASE WHEN `a`.`kind_id` = 9 THEN `a`.`count` ELSE 0 END) AS INTEGER) as `camel_count`,
            CAST( SUM(CASE WHEN `a`.`kind_id` = 10 THEN `a`.`count` ELSE 0 END) AS INTEGER) as `donkey_count`,
            CAST( SUM(CASE WHEN `a`.`kind_id` = 11 THEN `a`.`count` ELSE 0 END) AS INTEGER) as `bison_count`

        FROM enterprise_addresses AS ea
        LEFT JOIN enterprises AS e ON ea.enterprise_id = e.id
        LEFT JOIN animals AS a ON e.id = a.enterprise_id

        WHERE ea.region_code = "0c089b04-099e-4e0e-955a-6bf1ce525f1a" 
        AND ea.district_code IN ({}) OR ea.locality_code IN ({})
        AND a.created_at > '{}'
        AND a.created_at < '{}'

        GROUP BY name
    "#,
        &districts_filter, &districts_filter, filter_date_from, filter_date_to
    );

    let mut sql_response: Vec<ResponseItem> = vec![];

    match connection {
        Err(err) => {
            println!("Cannot connect to database [{}]", err.to_string());
        }
        Ok(pool) => {
            println!("Connected to database successfully.");
            let result_all: Result<Vec<ResponseItem>, _> =
                sqlx::query_as(&test_query).fetch_all(&pool).await;
            // dbg!(&result_all);
            sql_response = result_all.unwrap();
        }
    }

    let mut response: Vec<ResponseItem> = vec![];

    for sql_item in sql_response {
        let (id, name) = get_district_name_by_id(&sql_item.name);
        let response_item = ResponseItem {
            id: id.try_into().unwrap(),
            name,
            count: sql_item.count,
            krs_count: sql_item.krs_count,
            pig_count: sql_item.pig_count,
            goat_count: sql_item.goat_count,
            sheep_count: sql_item.sheep_count,
            horse_count: sql_item.horse_count,
            bee_count: sql_item.bee_count,
            dog_count: sql_item.dog_count,
            cat_count: sql_item.cat_count,
            deer_count: sql_item.deer_count,
            maral_count: sql_item.maral_count,
            camel_count: sql_item.camel_count,
            donkey_count: sql_item.donkey_count,
            bison_count: sql_item.bison_count,
        };

        response.push(response_item);
    }

    let json_response = json!(response);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json_response)
}
