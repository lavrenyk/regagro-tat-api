//! src/routes/api/analytics/animals/get_animal_count_by_district.rs
#![allow(unused_assignments)]
use crate::structs::QueryData;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct ResponseItem {
    id: i32,
    name: String,
    count: i64,
    krs_count: i32,
    pig_count: i32,
    goat_count: i32,
    sheep_count: i32,
    horse_count: i32,
    bee_count: i32,
    dog_count: i32,
    cat_count: i32,
    deer_count: i32,
    maral_count: i32,
    camel_count: i32,
    donkey_count: i32,
    bison_count: i32,
}
impl ResponseItem {
    fn new() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            count: 0,
            krs_count: 0,
            pig_count: 0,
            goat_count: 0,
            sheep_count: 0,
            horse_count: 0,
            bee_count: 0,
            dog_count: 0,
            cat_count: 0,
            deer_count: 0,
            maral_count: 0,
            camel_count: 0,
            donkey_count: 0,
            bison_count: 0,
        }
    }
}

/// Generate response JSON with data for any asked district of amount animals
/// with filtered by given period.
pub async fn get_animal_count_by_district(
    data: web::Query<QueryData>,
    _pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let _request_id = Uuid::new_v4();

    dbg!(data);

    HttpResponse::Ok().finish()
}
