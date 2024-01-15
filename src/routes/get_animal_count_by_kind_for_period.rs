//! src/routes/api/analytics/animals/get_animal_by_kind_for_period.rs
#![allow(unused_assignments)]
use crate::helpers::{
    all_districts_filter, district_filter_query, get_all_kind_ids, get_kind_name_by_id,
    get_region_districts, get_region_guid,
};
use crate::structs::QueryData;
use actix_web::{web, HttpResponse};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, MySqlPool};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseItem {
    pub count: u64,
    pub kind_id: u64,
    pub view: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct SqlResponse {
    kind_id: u64,
    count: u64,
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
    _pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let _request_id = Uuid::new_v4();
    // ЭТАП 1: Переформатируем QueryData в необходимые данные для работы

    //TODO: Перенести проверку в функцию
    // Обработка данных региона `id` и `guid`
    let region_id: u32 = {
        match data.region_id {
            Some(data) => data,
            None => 16,
        }
    };

    let region_guid = get_region_guid(region_id); // получаем GUID региона
                                                  // Грузим данные по районам в регионе
    let region_districts = get_region_districts(region_id).await;

    // Получаем список GUID районов, для дальнейшего запроса в БД
    let districts_filter: String = {
        match &data.districts {
            Some(data) => district_filter_query(&data.as_str(), &region_districts),
            None => all_districts_filter(&region_districts),
        }
    };

    let date_from: String = {
        match &data.date_reg_from {
            Some(date_from) => date_from.to_string(),
            None => "2020-01-01".to_string(),
        }
    };

    let date_to: String = {
        match &data.date_reg_to {
            Some(date_to) => date_to.to_string(),
            None => Local::now().format("%Y-%m-%d").to_string(),
        }
    };

    let kind_ids: String = {
        match &data.kinds {
            Some(data_kinds) => data_kinds.to_string(),
            None => get_all_kind_ids(),
        }
    };

    // ЭТАП 2: Запрашиваем информацию из БД
    let sql_query = format!(
        r#"SELECT 
        CAST(a.kind_id AS UNSIGNED) AS kind_id, 
        CAST(COUNT(a.id) AS UNSIGNED) AS count

        FROM animals AS a 
        LEFT JOIN enterprises AS e ON a.enterprise_id = e.id
        LEFT JOIN enterprise_addresses AS ea ON e.id = ea.enterprise_id

        WHERE ea.region_code = "{}" 
        AND ea.district_code IN ({}) OR ea.locality_code IN ({})
        AND a.kind_id IN ({})
        AND a.created_at >= '{}'
        AND a.created_at <= '{}'

        GROUP BY a.kind_id"#,
        region_guid, &districts_filter, &districts_filter, kind_ids, date_from, date_to
    );

    let connection =
        MySqlPool::connect("mysql://mp_analytic:8Nlr7fDQNwmniu6h@vo.regagro.ru:33636/regagro_3_0")
            .await;

    let mut sql_response: Vec<SqlResponse> = vec![];

    match connection {
        Err(err) => {
            println!("Cannot connect to database [{}]", err.to_string());
        }
        Ok(pool) => {
            println!("Connected to database successfully.");
            let result_all: Result<Vec<SqlResponse>, _> =
                sqlx::query_as(&sql_query).fetch_all(&pool).await;
            dbg!(&result_all);
            sql_response = result_all.unwrap();
        }
    }

    let mut response: Vec<ResponseItem> = vec![];

    for sql_item in sql_response {
        let (name, view) = get_kind_name_by_id(&sql_item.kind_id);
        let response_item = ResponseItem {
            kind_id: sql_item.kind_id,
            count: sql_item.count,
            name,
            view,
        };

        response.push(response_item);
    }

    let json_response = json!(response);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json_response)
}
