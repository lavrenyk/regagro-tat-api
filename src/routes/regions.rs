//! src/routes/regions.rs
//! /api/regions
#![allow(unused_assignments)]
use crate::helpers::load_json_file;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryData {
    pub id: Option<u32>,
    pub guid: Option<String>,
}

/// Generate response JSON with data for any asked district of amount animals
/// with filtered by given period.
pub async fn get_regions(data: web::Query<QueryData>) -> HttpResponse {
    let _request_id = Uuid::new_v4();
    let mut json_response = json!([]);

    let regions = load_json_file("regions");

    match &data.guid {
        Some(guid) => {
            for region in regions.as_array().unwrap() {
                if &(region["guid"].as_str().unwrap().to_string()) == guid {
                    json_response = json!([region.to_owned()]);
                }
            }
        }
        None => json_response = regions,
    }

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json_response)
}
