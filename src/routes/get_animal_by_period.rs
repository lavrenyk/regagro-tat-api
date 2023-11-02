//! src/routes/get_animal_by_period.rs

use actix_web::{web, HttpResponse};
use async_std::task;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{Error, MySql, MySqlPool, Pool};

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub region_id: u32,
    pub date_reg_from: String,
    pub date_reg_to: String,
    pub kinds: String,
    pub districts: String,
}

// Testing connection to MySQL
///
/// It should be connect well
async fn connect() -> Result<Pool<MySql>, Error> {
    return MySqlPool::connect(
        "mysql://mp_analytic:8Nlr7fDQNwmniu6h@vo.regagro.ru:33633/regagro_3_0",
    )
    .await;
}

async fn do_run_query() {
    let result = task::block_on(connect());
    let district_code = "bb9c86eb-30de-4b8f-9ea8-9edc71fc0488".to_string();

    let query = format!(
      "SELECT a.id FROM animals AS a LEFT JOIN enterprises AS e ON a.enterprise_id=e.id LEFT JOIN enterprise_addresses AS ea ON ea.enterprise_id=e.id WHERE ea.district_code='{}';", district_code
  );

    dbg!(&query);

    match result {
        Err(err) => {
            println!("Cannot connect to database [{}]", err.to_string());
        }

        Ok(pool) => {
            let query_result = sqlx::query(query.as_str()).fetch_all(&pool).await.unwrap();

            println!("Number of Animals selected: {:?}", query_result.len());
        }
    }
}

pub async fn get_animal_by_period(data: web::Query<Field>) -> HttpResponse {
    task::block_on(do_run_query());

    let body = json!(
      [
        {
          "count": 31990,
          "kind_id": 1,
          "view": "КРС",
          "name": "Крупный рогатый скот"
        },
        {
          "count": 23,
          "kind_id": 2,
          "view": "Свинья",
          "name": "Свинья"
        },
        {
          "count": 17,
          "kind_id": 3,
          "view": "Лошади",
          "name": "Лошади"
        },
        {
          "count": 10,
          "kind_id": 7,
          "view": "Олени",
          "name": "Олени"
        },
        {
          "count": 49,
          "kind_id": 12,
          "view": "Овцы",
          "name": "Овцы"
        },
        {
          "count": 138,
          "kind_id": 13,
          "view": "Козы",
          "name": "Козы"
        }
      ]
    );

    let json_message = json!({
        "region_id": data.region_id,
        "date_reg_from": data.date_reg_from,
        "date_reg_to": data.date_reg_to,
        "kinds": data.kinds,
        "districts": data.districts
    });

    println!("path data: {:?}", json_message);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(body)
}
