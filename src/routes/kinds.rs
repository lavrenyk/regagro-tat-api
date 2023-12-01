//! src/routes/kinds.rs
//! /api/kinds
#![allow(unused_assignments)]
use crate::{helpers::load_json_file, structs::QueryData};
use actix_web::{web, HttpResponse};
use sqlx::MySqlPool;
use uuid::Uuid;

/// Generate response JSON with data for any asked district of amount animals
/// with filtered by given period.
pub async fn get_kinds(_data: web::Query<QueryData>, _pool: web::Data<MySqlPool>) -> HttpResponse {
    let _request_id = Uuid::new_v4();

    let animal_kinds = load_json_file("animals");

    let json_response = animal_kinds;

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json_response)
}
