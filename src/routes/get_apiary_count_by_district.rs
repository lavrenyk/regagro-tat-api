//! src/routes/get_apiary_count_by_district.rs
//! test URL: curl -v '127.0.0.1:8000/api/analytics/animals/getApiaryCountByDistrict?region_id=16&date_reg_from=2023-01-01&date_reg_to=2023-12-31&districts=275,277'
#![allow(unused_assignments)]

use actix_web::{web, HttpResponse};
use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, MySqlPool};

use crate::{
    helpers::{
        all_districts_filter, district_filter_query, get_district_name_by_id, get_region_districts,
        get_region_guid,
    },
    structs::QueryData,
};

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct SqlResponse {
    district_code: String,
    all_apiary: i32,
    all_enterprises: i32,
    all_juridical_apiary: i32,
    all_individual_apiary: i32,
    all_juridical_enterprise: i32,
    all_individual_enterprise: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseItem {
    id: i64,
    name: String,
    all_apiary: i32,
    all_enterprises: i32,
    all_juridical_apiary: i32,
    all_individual_apiary: i32,
    all_juridical_enterprise: i32,
    all_individual_enterprise: i32,
}

pub async fn get_apiary_count_by_district(
    data: web::Query<QueryData>,
    _pool: web::Data<MySqlPool>,
) -> HttpResponse {
    //* Income data parse
    let region_id: u32 = {
        match data.region_id {
            Some(data) => data,
            None => 16,
        }
    };

    let region_guid = get_region_guid(region_id); // получаем GUID региона

    // Грузим данные по районам в регионе
    let region_districts = get_region_districts(region_id).await;

    let date_from: String = {
        match &data.date_reg_from {
            Some(date_from) => date_from.to_string(),
            None => "2023-01-01".to_string(),
        }
    };

    let date_to: String = {
        match &data.date_reg_to {
            Some(date_to) => date_to.to_string(),
            None => Local::now().format("%Y-%m-%d").to_string(),
        }
    };

    // Получаем список GUID районов, для дальнейшего запроса в БД
    let districts: String = {
        match &data.districts {
            Some(data) => district_filter_query(&data.as_str(), &region_districts),
            None => all_districts_filter(&region_districts),
        }
    };

    // Database request section
    let sql_request = format!(
        r#"
        SELECT ea.district_code,
            CAST(SUM(a.count) AS INTEGER) as all_apiary,
            COUNT(DISTINCT(e.id)) as all_enterprises,
            CAST(SUM(CASE WHEN o.legal_form_id IN (1, 3, 4) THEN a.count ELSE 0 END) AS INTEGER) as all_juridical_apiary,
            CAST(SUM(CASE WHEN o.legal_form_id = 2 THEN a.count ELSE 0 END) AS INTEGER) as all_individual_apiary,
            CAST(SUM(CASE WHEN o.legal_form_id IN (1, 3, 4) THEN 1 ELSE 0 END) AS INTEGER) as all_juridical_enterprise,
            CAST(SUM(CASE WHEN o.legal_form_id = 2 THEN 1 ELSE 0 END) AS INTEGER) as all_individual_enterprise
            
            FROM regagro_3_0.enterprises as e
            LEFT JOIN regagro_3_0.enterprise_addresses as ea ON ea.enterprise_id = e.id
            LEFT JOIN regagro_3_0.owners as o ON e.owner_id = o.id
            LEFT JOIN (
                SELECT enterprise_id, SUM(count) as count
                FROM regagro_3_0.animals
                WHERE kind_id = 4
                    AND created_at >= '{}'
                    AND created_at <= '{}'
                    AND is_super_group = 0          
                GROUP BY enterprise_id
            ) as a ON a.enterprise_id = e.id
        WHERE ea.region_code = '{}'
            AND ea.district_code IN ({})
            AND a.count > 0
        GROUP BY ea.district_code;
        "#,
        date_from, date_to, region_guid, districts
    );

    let mut sql_response: Vec<SqlResponse> = vec![];
    let connection =
        MySqlPool::connect("mysql://mp_analytic:8Nlr7fDQNwmniu6h@vo.regagro.ru:33633/regagro_3_0")
            .await;

    match connection {
        Err(err) => {
            println!("Cannot connect to database [{}]", err.to_string());
        }
        Ok(pool) => {
            println!("Connected to database successfully.");
            let result_all: Result<Vec<SqlResponse>, _> =
                sqlx::query_as(&sql_request).fetch_all(&pool).await;
            dbg!(&result_all);
            sql_response = result_all.unwrap();
        }
    }

    let mut response: Vec<ResponseItem> = vec![];

    for sql_item in sql_response {
        let (id, name) =
            get_district_name_by_id(&region_districts, sql_item.district_code.as_str());
        let response_item = ResponseItem {
            id,
            name: name.as_str().to_string(),
            all_apiary: sql_item.all_apiary,
            all_enterprises: sql_item.all_enterprises,
            all_juridical_apiary: sql_item.all_juridical_apiary,
            all_individual_apiary: sql_item.all_individual_apiary,
            all_juridical_enterprise: sql_item.all_juridical_enterprise,
            all_individual_enterprise: sql_item.all_individual_enterprise,
        };

        response.push(response_item);
    }

    let json_response = json!(response);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json_response)
}
