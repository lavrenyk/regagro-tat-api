//! src/routes/api/analytics/animals/get_animal_by_kind_for_period.rs

use crate::helpers::{all_districts_filter, animals_filter_query, district_filter_query};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::MySqlPool;
use std::fs;
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseItem {
    pub count: i64,
    pub kind_id: u32,
    pub view: String,
    pub name: String,
}
impl ResponseItem {
    pub fn new() -> Self {
        Self {
            count: 0,
            kind_id: 0,
            view: "".to_string(),
            name: "".to_string(),
        }
    }
}

/// Generate response JSON with data for any asked kind of animal in separated data
/// with filtered by given period and districts.
///
/// Data for any given kind of animal is returned in the fallowing format:
/// * `[
/// *    {
/// *        "count": 31990, // u32 - amount of the filtered animal kinds in db
/// *        "kind_id": 1,  // u32 - animal kind number
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

            for (_i, kind) in animal_kinds.iter().enumerate() {
                let mut responseItem = ResponseItem::new();

                for animal_kind in animal_kinds_data.as_array().unwrap() {
                    if &animal_kind["regagro_code"].as_str().unwrap() == kind {
                        responseItem.kind_id = kind.parse().unwrap();
                        responseItem.name = animal_kind["name"].to_string();
                        responseItem.view = animal_kind["view"].to_string();
                    }
                }

                let kind: u32 = kind.parse().unwrap();
                let query = format!("SELECT COUNT(*) FROM animals AS a LEFT JOIN enterprises AS e ON a.enterprise_id=e.id LEFT JOIN enterprise_addresses AS ea ON ea.enterprise_id=e.id WHERE a.kind_id={} AND ({});", kind, districts_filter);

                let result: Result<(i64,), _> =
                    sqlx::query_as(query.as_str()).fetch_one(&pool).await;

                let result = result.unwrap().0;

                if result > 0 {
                    responseItem.count = result;
                } else {
                    break;
                }

                response_data.push(responseItem);
            }
        }
    }

    dbg!(&response_data);

    HttpResponse::Ok().finish()
}
