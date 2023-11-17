//! src/routes/get_livestock_count_by_companies.rs

use actix_web::{web, HttpResponse};
use serde_json::json;
use sqlx::MySqlPool;

use crate::structs::QueryData;

pub async fn get_livestock_count_by_companies(
    _data: web::Query<QueryData>,
    _pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let json_response = json!([]);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json_response)
}
