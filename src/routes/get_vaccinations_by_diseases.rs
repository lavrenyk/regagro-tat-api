//! src/routes/get_vaccinations_by_diseases.rs
//! 127.0.0.1:8000/api/v2/analytics/vaccinations/getVaccinationsByDiseases?region_id=16&date_from='2023-01-01'&date_to='2023-12-31'&kind_ids=1,2,3&enterprise_districts=275,277
#![allow(unused_assignments)]

use actix_web::{web, HttpResponse};
use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, MySqlPool};

use crate::{helpers::*, structs::QueryData};

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct ResponseItem {
    id: u64,
    count: u32,
    public_sector_count: u32,
    private_sector_count: u32,
    name: String,
    view: String,
}

pub async fn get_vaccinations_by_diseases(
    data: web::Query<QueryData>,
    _pool: web::Data<MySqlPool>,
) -> HttpResponse {
    //* Income data parse
    let mut kind_ids = "1,2,3,4,5,6,7,8,9,10,11,12,13".to_string();
    let mut districts = "".to_string();

    //TODO: Перенести проверку в функцию
    // Обработка данных региона `id` и `guid`
    let region_id: u32 = {
        match data.region_id {
            Some(data) => data,
            None => 0,
        }
    };

    let region_guid = get_region_guid(region_id); // получаем GUID региона

    let date_from: String = {
        match &data.date_from {
            Some(date_from) => date_from.to_string(),
            None => "2020-01-01".to_string(),
        }
    };

    let date_to: String = {
        match &data.date_to {
            Some(date_to) => date_to.to_string(),
            None => Local::now().format("%Y-%m-%d").to_string(),
        }
    };

    match &data.kind_ids {
        Some(data_kind_ids) => {
            kind_ids = data_kind_ids.to_string();
        }
        None => (),
    }

    //TODO: Перенести проверку в функцию
    // Обработка данных региона `id` и `guid`
    let region_id: u32 = {
        match data.region_id {
            Some(data) => data,
            None => 0,
        }
    };

    // Грузим данные по районам в регионе
    let region_districts = get_region_districts(region_id).await;

    match &data.enterprise_districts {
        Some(data_districts) => {
            districts = district_filter_query(data_districts, &region_districts);
        }
        None => {
            districts = all_districts_filter(&region_districts);
        }
    }

    // Database request section
    let sql_request = format!(
        r#" SELECT 
CAST(SUM(`a`.`count`) AS UNSIGNED) as `count`, `d`.`id` as `id`, `d`.`name` as `name`, `d`.`name` as `view`,
CAST( SUM(CASE WHEN `o`.`legal_form_id` IN (1,3,4) THEN 1 ELSE 0 END) AS UNSIGNED) as `public_sector_count`,
CAST( SUM(CASE WHEN `o`.`legal_form_id` = 2 THEN 1 ELSE 0 END) AS UNSIGNED) as `private_sector_count`
FROM `regagro_3_0`.`vaccinations` as `v`
LEFT JOIN `regagro_3_0`.`vaccination_diseases` as `vd` ON `vd`.`vaccination_id` = `v`.`id`
LEFT JOIN `regagro_3_0`.`animals` as `a` ON `a`.`id` = `v`.`animal_id`
LEFT JOIN `regagro_3_0`.`owners` as `o` ON `v`.`owner_id` = `o`.`id`
LEFT JOIN `regagro_3_0`.`enterprises` as `e` ON `v`.`enterprise_id` = `e`.`id`
LEFT JOIN `regagro_3_0`.`enterprise_addresses` as `ea` ON `ea`.`enterprise_id` = `e`.`id`
LEFT JOIN `regagro_3_0_handbooks`.`diseases` as `d` ON `d`.`id` = `vd`.`disease_id`
WHERE `v`.`is_super_group` = 0 
AND `v`.`date` >= '{}' AND `v`.`date` <= '{}'
AND `ea`.`region_code` = '{}'
AND `ea`.`district_code` IN ({})
AND `a`.`kind_id` IN ({})
GROUP BY `d`.`id`;"#,
        date_from, date_to, region_guid, districts, kind_ids
    );

    let mut response: Vec<ResponseItem> = vec![];
    let connection =
        MySqlPool::connect("mysql://mp_analytic:8Nlr7fDQNwmniu6h@vo.regagro.ru:33636/regagro_3_0")
            .await;

    match connection {
        Err(err) => {
            println!("Cannot connect to database [{}]", err.to_string());
        }
        Ok(pool) => {
            println!("Connected to database successfully.");
            let result_all: Result<Vec<ResponseItem>, _> =
                sqlx::query_as(&sql_request).fetch_all(&pool).await;
            dbg!(&result_all);
            response = result_all.unwrap();
        }
    }

    // JSON preparation
    let json_response = json!(response);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json_response)
}
