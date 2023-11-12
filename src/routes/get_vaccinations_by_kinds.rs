//! src/routes/get_vaccinations_by_kinds.rs

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, MySqlPool};

use crate::structs::QueryData;

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
    _data: web::Query<QueryData>,
    _pool: web::Data<MySqlPool>,
) -> HttpResponse {
    // let json_response = json!("[]");

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
AND `v`.`date` >= "2023/01/01" AND `v`.`date` <= "2023/12/12"
/*Код региона*/
AND `ea`.`region_code` = "6f2cbfd8-692a-4ee4-9b16-067210bde3fc"
/*Коды районов*/
AND `ea`.`district_code` IN ("c7a81174-8d01-4ae6-83e6-386ae23ee629","993701c9-2100-48ef-8b83-1167e82df13f", "53056d1c-5c65-43e8-be8e-8edf32480797")
/*Виды животных*/
AND `a`.`kind_id` IN (1,2,3,4,5,6,7,8,9,10)
GROUP BY `k`.`id`;
    "#
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
