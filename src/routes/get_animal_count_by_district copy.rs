//! src/routes/api/analytics/animals/get_animal_count_by_district.rs
#![allow(unused_assignments)]
use crate::helpers::{all_districts_filter, district_filter_query};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::MySqlPool;
use std::fs;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseItem {
    pub id: i32,
    pub name: String,
    pub count: i64,
    pub krs_count: i32,
    pub pig_count: i32,
    pub goat_count: i32,
    pub sheep_count: i32,
    pub horse_count: i32,
    pub bee_count: i32,
    pub dog_count: i32,
    pub cat_count: i32,
    pub deer_count: i32,
    pub maral_count: i32,
    pub camel_count: i32,
    pub donkey_count: i32,
    pub bison_count: i32,
}
impl ResponseItem {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            count: 0,
            krs_count: 0,
            pig_count: 0,
            goat_count: 0,
            sheep_count: 0,
            horse_count: 0,
            bee_count: 0,
            dog_count: 0,
            cat_count: 0,
            deer_count: 0,
            maral_count: 0,
            camel_count: 0,
            donkey_count: 0,
            bison_count: 0,
        }
    }
}

/// Generate response JSON with data for any asked district of amount animals
/// with filtered by given period.
pub async fn get_animal_count_by_districts(
    data: web::Query<QueryData>,
    _pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let _request_id = Uuid::new_v4();

    // ЭТАП 1: Переформатируем QueryData в необходимые данные для работы
    // 1. Определяем количество районов в запросе. Для этого переводим данные QueryData.disctricts
    // из [`String`] в [`Vec<String>`]. Дополнительно проверяем, были ли данные по районам в запросе,
    // если не было, то выбираем все районы
    let mut regions_filter: Vec<&str> = vec![];
    // let mut animal_kinds_filter: Vec<&str> = vec![];

    let mut districts_list = "".to_string();

    match &data.districts {
        Some(districts) => {
            districts_list = district_filter_query(&districts.as_str()).clone();
            regions_filter = districts_list.split(",").collect();
        }
        // If no parameter was send select all regions
        None => {
            districts_list = all_districts_filter();
            regions_filter = districts_list.split(",").collect();
        }
    }

    dbg!(regions_filter);

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

    // 3. Подготавливаем данные с информацией о типах животных и формируем JSON
    // данные будут использованы для формирования ответного JSON
    let file_path = "src/data/animals.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    let contents = contents.as_str();
    let animal_kinds_data: Value = serde_json::from_str(contents).unwrap();

    let connection =
        MySqlPool::connect("mysql://mp_analytic:8Nlr7fDQNwmniu6h@vo.regagro.ru:33633/regagro_3_0")
            .await;

    let mut response_data: Vec<ResponseItem> = vec![];

    match connection {
        Err(err) => {
            println!("Cannot connect to database [{}]", err.to_string());
        }
        Ok(pool) => {
            println!("Connected to database successfully.");

            // for (_i, kind_filter) in animal_kinds_filter.iter().enumerate() {
            //     let mut response_item = ResponseItem::new();

            //     for animal_kind in animal_kinds_data.as_array().unwrap() {
            //         // dbg!(&kind_filter);
            //         // dbg!(&animal_kind);
            //         // if &animal_kind["regagro_code_v3"].as_str().unwrap() == kind_filter {
            //         //     response_item.kind_id = kind_filter.parse().unwrap();
            //         //     response_item.name =
            //         //         animal_kind["name"].as_str().to_owned().unwrap().to_string();
            //         //     response_item.view =
            //         //         animal_kind["view"].as_str().to_owned().unwrap().to_string();
            //         //     // dbg!(&response_item);
            //         //     break;
            //     }
            // }

            // let kind_id: u32 = kind_filter.parse().unwrap();
            let query = format!(
                    "SELECT COUNT(*) FROM animals AS a LEFT JOIN enterprises AS e ON a.enterprise_id=e.id LEFT JOIN enterprise_addresses AS ea ON ea.enterprise_id=e.id WHERE a.kind_id={} AND;",
                    // kind_id,
                    districts_filter
                );

            let result: Result<(i64,), _> = sqlx::query_as(query.as_str()).fetch_one(&pool).await;

            let result = result.unwrap().0;

            if result > 0 {
                // response_item.count = result;
                // dbg!(&response_item);
            } else {
                // break;
            }

            // dbg!(&response_item);

            // response_data.push(response_item);
        }
    }

    // dbg!(&response_data);

    // let json_response = json!(response_data);

    // dbg!(json_response);

    HttpResponse::Ok().finish()

    // .content_type("application/json")
    // .json(json_response)
}
