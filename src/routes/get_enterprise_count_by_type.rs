//! src/routes/get_enterprise_count_by_type.rs
//! /api/analytics/enterprises/getEnterpriseCountByType?region_id=16

use actix_web::{web, HttpResponse};
use serde_json::json;
use sqlx::MySqlPool;

use crate::structs::QueryData;

pub async fn get_enterprise_count_by_type(
    _data: web::Query<QueryData>,
    _pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let json_response = json!([]);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json_response)
}
