//! src/routes/get_animal_by_period.rs

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::MySqlPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub region_id: u32,
    pub date_reg_from: String,
    pub date_reg_to: String,
    pub kinds: String,
    pub districts: String,
}

pub async fn get_animal_by_period(
    data: web::Query<Field>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    // let connection =
    //     MySqlPool::connect("mysql://mp_analytic:8Nlr7fDQNwmniu6h@vo.regagro.ru:33633/regagro_3_0")
    //         .await;
    let request_id = Uuid::new_v4();

    // let connection = MySqlPool::connect("mysql://root:kakeepoo@127.0.0.1:3306/regagro_3_0").await;

    let query_span = tracing::info_span!("Requesting animals count in the database.");

    let district_code = "3b67dc8e-79b1-43f4-8f9e-2b4990a1a922".to_string();
    let query = format!(
      "SELECT a.id FROM animals AS a LEFT JOIN enterprises AS e ON a.enterprise_id=e.id LEFT JOIN enterprise_addresses AS ea ON ea.enterprise_id=e.id WHERE ea.district_code='{}';", district_code
    );

    // dbg!(&connection);
    dbg!(&data);

    match sqlx::query(
        r#"
            SELECT * FROM animals;
            "#,
    )
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("{} - Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }

    // match connection {
    //     Err(err) => {
    //         println!("Cannot connect to database [{}]", err.to_string());
    //     }
    //     Ok(pool) => {
    //         println!("Connected to database successfully.");
    //         let query_result = sqlx::query(query.as_str()).fetch_all(&pool).await.unwrap();

    //         println!("Number of Animals selected: {:?}", query_result.len());
    //     }
    // }

    // let body = json!(
    //     [
    //       {
    //         "count": 31990,
    //         "kind_id": 1,
    //         "view": "КРС",
    //         "name": "Крупный рогатый скот"
    //       },
    //       {
    //         "count": 23,
    //         "kind_id": 2,
    //         "view": "Свинья",
    //         "name": "Свинья"
    //       },
    //       {
    //         "count": 17,
    //         "kind_id": 3,
    //         "view": "Лошади",
    //         "name": "Лошади"
    //       },
    //       {
    //         "count": 10,
    //         "kind_id": 7,
    //         "view": "Олени",
    //         "name": "Олени"
    //       },
    //       {
    //         "count": 49,
    //         "kind_id": 12,
    //         "view": "Овцы",
    //         "name": "Овцы"
    //       },
    //       {
    //         "count": 138,
    //         "kind_id": 13,
    //         "view": "Козы",
    //         "name": "Козы"
    //       }
    //     ]
    //   );

    let json_message = json!({
        "region_id": data.region_id,
        "date_reg_from": data.date_reg_from,
        "date_reg_to": data.date_reg_to,
        "kinds": data.kinds,
        "districts": data.districts
    });

    // println!("path data: {:?}", json_cmessage);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json_message)
}
