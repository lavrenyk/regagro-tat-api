//! src/routes/get_vaccinations_by_kinds.rs
//! test url: '127.0.0.1:8000/api/v2/analytics/vaccinations/getVaccinationsByKinds?region_id=16&date_from=2023-01-01&date_to=2023-12-31&kind_ids=1,2,3,4,5&enterprise_districts=275,277&enterprise_pos_councils=1,2,3'
#![allow(unused_assignments)]

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, MySqlPool};

use crate::{
    helpers::{all_districts_filter, district_filter_query, get_region_districts},
    structs::QueryData,
};

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct ResponseItem {
    id: u64,
    count: u32,
    public_sector_count: u32,
    private_sector_count: u32,
    name: String,
    view: String,
}

pub async fn get_vaccinations_by_kinds(
    data: web::Query<QueryData>,
    _pool: web::Data<MySqlPool>,
) -> HttpResponse {
    //* Income data parse
    dbg!(&data);

    let mut date_from = "2023-01-01".to_string();
    let mut date_to = "2023-12-31".to_string();
    let mut kind_ids = "1,2,3,4,5,6,7,8,9,10,11,12,13".to_string();
    let mut districts = String::new();

    match &data.date_from {
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
        r#"
SELECT 
CAST(SUM(`a`.`count`) AS UNSIGNED) as `count`, `k`.`id` as `id`, `k`.`name` as `name`, `k`.`name` as `view`,
CAST( SUM(CASE WHEN `o`.`legal_form_id` IN (1,3,4) THEN 1 ELSE 0 END) AS UNSIGNED) as `public_sector_count`,
CAST( SUM(CASE WHEN `o`.`legal_form_id` = 2 THEN 1 ELSE 0 END) AS UNSIGNED) as `private_sector_count`
FROM `regagro_3_0`.`vaccinations` as `v` 
LEFT JOIN `regagro_3_0`.`animals` as `a` ON `a`.`id` = `v`.`animal_id`
LEFT JOIN `regagro_3_0`.`owners` as `o` ON `v`.`owner_id` = `o`.`id`
LEFT JOIN `regagro_3_0`.`enterprises` as `e` ON `v`.`enterprise_id` = `e`.`id`
LEFT JOIN `regagro_3_0`.`enterprise_addresses` as `ea` ON `ea`.`enterprise_id` = `e`.`id`
LEFT JOIN `regagro_3_0_handbooks`.`kinds` as `k` ON `k`.`id` = `a`.`kind_id`
WHERE `v`.`is_super_group` = 0
/*Даты от и до */
AND `v`.`date` >= "{}" AND `v`.`date` <= "{}"
/*Код региона*/
AND `ea`.`region_code` = "0c089b04-099e-4e0e-955a-6bf1ce525f1a"
/*Коды районов*/
AND `ea`.`district_code` IN ({})
/*Виды животных*/
AND `a`.`kind_id` IN ({})
GROUP BY `k`.`id`;
    "#,
        date_from, date_to, districts, kind_ids
    );

    let mut response: Vec<ResponseItem> = vec![];
    let connection =
        MySqlPool::connect("mysql://mp_analytic:8Nlr7fDQNwmniu6h@vo.regagro.ru:33633/regagro_3_0")
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

    let json_response = json!(response);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json_response)
}
