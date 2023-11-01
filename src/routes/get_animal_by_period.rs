//! src/routes/get_animal_by_period.rs

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub region_id: u32,
    pub date_reg_from: String,
    pub date_reg_to: String,
    pub kinds: String,
    pub districts: String,
}

pub async fn get_animal_by_period(data: web::Query<Field>) -> HttpResponse {
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

    let path = data.into_inner();

    println!("path data: {:?}", path);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(body)
}
