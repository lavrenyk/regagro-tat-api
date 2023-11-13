//! src/routes/get_vaccinations_by_districts.rs
//! test URL: '127.0.0.1:8000/api/v2/analytics/vaccinations/getVaccinationsByDistricts?region_id=16&date_from=2023-01-01&date_to=2023-12-31&kind_ids=1,2,3,4,5&enterprise_districts=275,277'

use std::vec;

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, MySqlPool};

use crate::{
    helpers::{all_districts_filter, district_filter_query, get_district_name_by_id},
    structs::QueryData,
};

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct SqlResponse {
    id: String,
    count: u32,
    public_sector_count: u32,
    private_sector_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseItem {
    id: i64,
    count: u32,
    name: String,
    public_sector_count: u32,
    private_sector_count: u32,
}

pub async fn get_vaccinations_by_districts(
    data: web::Query<QueryData>,
    _pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let json_response = json!("[]");

    //* Income data parse
    let mut date_from = "2023-01-01".to_string();
    let mut date_to = "2023-12-31".to_string();
    let mut kind_ids = "1,2,3,4,5,6,7,8,9,10,11,12,13".to_string();
    let mut districts = "".to_string();

    match &data.date_from {
        // check date
        Some(data_date_from) => {
            date_from = data_date_from.to_string();
        }
        None => (),
    }

    match &data.date_to {
        Some(data_date_to) => {
            date_to = data_date_to.to_string();
        }
        None => (),
    }

    match &data.kind_ids {
        Some(data_kind_ids) => {
            kind_ids = data_kind_ids.to_string();
        }
        None => (),
    }

    match &data.enterprise_districts {
        Some(data_districts) => {
            districts = district_filter_query(data_districts);
        }
        None => {
            districts = all_districts_filter();
        }
    }

    // Database request section
    let sql_request = format!(
        r#"
SELECT 
CAST(SUM(`a`.`count`) AS UNSIGNED) as `count`, `ea`.`district_code` as `id`, 
CAST( SUM(CASE WHEN `o`.`legal_form_id` IN (1,3,4) THEN 1 ELSE 0 END) AS UNSIGNED) as `public_sector_count`,
CAST( SUM(CASE WHEN `o`.`legal_form_id` = 2 THEN 1 ELSE 0 END) AS UNSIGNED) as `private_sector_count`
FROM `regagro_3_0`.`vaccinations` as `v`
LEFT JOIN `regagro_3_0`.`animals` as `a` ON `a`.`id` = `v`.`animal_id`
LEFT JOIN `regagro_3_0`.`owners` as `o` ON `v`.`owner_id` = `o`.`id`
LEFT JOIN `regagro_3_0`.`enterprises` as `e` ON `v`.`enterprise_id` = `e`.`id`
LEFT JOIN `regagro_3_0`.`enterprise_addresses` as `ea` ON `ea`.`enterprise_id` = `e`.`id`
WHERE `v`.`is_super_group` = 0 
AND `v`.`date` >= "{}" AND `v`.`date` <= "{}"
AND `ea`.`region_code` = "0c089b04-099e-4e0e-955a-6bf1ce525f1a"
AND `ea`.`district_code` IN ({})
AND `a`.`kind_id` IN ({})
GROUP BY `ea`.`district_code`;"#,
        date_from, date_to, districts, kind_ids
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
        let (id, name) = get_district_name_by_id(sql_item.id.as_str());
        let response_item = ResponseItem {
            id,
            name: name.as_str().to_string(),
            count: sql_item.count,
            public_sector_count: sql_item.public_sector_count,
            private_sector_count: sql_item.private_sector_count,
        };

        response.push(response_item);
    }

    // JSON preparation
    let json_response = json!(response);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json_response)
}
