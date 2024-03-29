//! src/routes/api/analytics/animals/get_animal_by_kind.rs
#![allow(unused_assignments)]
use crate::helpers::{get_all_kind_ids, get_kind_name_by_id, get_region_guid};
use crate::structs::QueryData;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, MySqlPool};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct ResponseItem {
    count: u64,
    kind_id: u64,
    view: String,
    name: String,
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
pub async fn get_animal_count_by_kind(
    data: web::Query<QueryData>,
    _pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Requesting animals count in region by kind",
        %request_id,
    );

    let _request_span_guard = request_span.enter();

    // ЭТАП 1: Переформатируем QueryData в необходимые данные для работы
    let kind_ids = get_all_kind_ids();

    //TODO: Перенести проверку в функцию
    // Обработка данных региона `id` и `guid`
    let region_id: u32 = {
        match data.region_id {
            Some(data) => data,
            None => 0,
        }
    };

    let region_guid = get_region_guid(region_id); // получаем GUID региона

    // ЭТАП 2: Запрашиваем информацию из БД
    let sql_query = format!(
        r#"
        SELECT 
        CAST(a.kind_id AS UNSIGNED) AS kind_id, 
        CAST(COUNT(a.id) AS UNSIGNED) AS count

        FROM animals AS a 
        LEFT JOIN enterprises AS e ON a.enterprise_id = e.id
        LEFT JOIN enterprise_addresses AS ea ON e.id = ea.enterprise_id

        WHERE ea.region_code = "{}" 
        AND a.kind_id IN ({})
        AND a.is_super_group=0
        AND a.deleted_at is null

        GROUP BY a.kind_id
        "#,
        region_guid, kind_ids
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
            let result_all: Result<Vec<SqlResponse>, _> =
                sqlx::query_as(&sql_query).fetch_all(&pool).await;
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
