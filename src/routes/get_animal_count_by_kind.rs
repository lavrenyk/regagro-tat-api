//! src/routes/api/analytics/animals/get_animal_by_kind.rs
#![allow(unused_assignments)]
use crate::helpers::{get_all_kind_ids, get_kind_name_by_id};
use crate::structs::QueryData;
use actix_web::{web, HttpResponse};
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
pub async fn get_animal_count_by_kind(
    data: web::Query<QueryData>,
    _pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let _request_id = Uuid::new_v4();

    dbg!(&data);

    // ЭТАП 1: Переформатируем QueryData в необходимые данные для работы
    let kind_ids = get_all_kind_ids();

    // ЭТАП 2: Запрашиваем информацию из БД
    let sql_query = format!(
        r#"SELECT 
        CAST(a.kind_id AS UNSIGNED) AS kind_id, 
        CAST(COUNT(a.id) AS UNSIGNED) AS count

        FROM animals AS a 
        LEFT JOIN enterprises AS e ON a.enterprise_id = e.id
        LEFT JOIN enterprise_addresses AS ea ON e.id = ea.enterprise_id

        WHERE ea.region_code = "0c089b04-099e-4e0e-955a-6bf1ce525f1a" 
        AND a.kind_id IN ({})

        GROUP BY a.kind_id"#,
        kind_ids
    );

    let connection =
        MySqlPool::connect("mysql://mp_analytic:8Nlr7fDQNwmniu6h@vo.regagro.ru:33633/regagro_3_0")
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
