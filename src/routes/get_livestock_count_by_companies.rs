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

    let sql_request = r#"
    SELECT o.id as `id`, o.name as `name`, o.uuid as `regagro_code`, 
    CAST( SUM(a.count) AS UNSIGNED) as `count`,
    CAST( COUNT(DISTINCT(e.id) AS UNSIGNED) as `enterprise_count`,
    CAST( SUM(CASE WHEN a.kind_id = 1 THEN a.count ELSE 0 END) AS UNSIGNED) as `krs_count`,
    CAST( SUM(CASE WHEN a.kind_id = 1 AND a.gender = 2 AND TIMESTAMPDIFF(MONTH, a.birth_date, NOW()) >= 24 THEN a.count ELSE 0 END) AS UNSIGNED) as `cow_count`,
    CAST( SUM(CASE WHEN a.kind_id = 2 THEN a.count ELSE 0 END) AS UNSIGNED) as `pig_count`,
    CAST( SUM(CASE WHEN a.kind_id = 12 THEN a.count ELSE 0 END) AS UNSIGNED) as `goat_count`,
    CAST( SUM(CASE WHEN a.kind_id = 13 THEN a.count ELSE 0 END) AS UNSIGNED) as `sheep_count`,
    CAST( SUM(CASE WHEN a.kind_id = 3 THEN a.count ELSE 0 END) AS UNSIGNED) as `horse_count`,
    CAST( SUM(CASE WHEN a.kind_id = 4 THEN a.count ELSE 0 END) AS UNSIGNED) as `bee_count`,
    CAST( SUM(CASE WHEN a.kind_id = 5 THEN a.count ELSE 0 END) AS UNSIGNED) as `dog_count`,
    CAST( SUM(CASE WHEN a.kind_id = 6 THEN a.count ELSE 0 END) AS UNSIGNED) as `cat_count`,
    CAST( SUM(CASE WHEN a.kind_id = 7 THEN a.count ELSE 0 END) AS UNSIGNED) as `deer_count`,
    CAST( SUM(CASE WHEN a.kind_id = 8 THEN a.count ELSE 0 END) AS UNSIGNED) as `maral_count`,
    CAST( SUM(CASE WHEN a.kind_id = 9 THEN a.count ELSE 0 END) AS UNSIGNED) as `camel_count`,
    CAST( SUM(CASE WHEN a.kind_id = 10 THEN a.count ELSE 0 END) AS UNSIGNED) as `donkey_count`,
    CAST( SUM(CASE WHEN a.kind_id = 11 THEN a.count ELSE 0 END) AS UNSIGNED) as `bison_count`
    
    FROM `regagro_3_0`.`animals` as `a`
    
    LEFT JOIN `regagro_3_0`.`enterprises` as `e` ON `a`.`enterprise_id` = `e`.`id`
    LEFT JOIN `regagro_3_0`.`enterprise_addresses` as `ea` ON `ea`.`enterprise_id` = `e`.`id`
    LEFT JOIN `regagro_3_0`.`owners` as `o` ON `e`.`owner_id` = `o`.`id`
    WHERE `a`.`is_super_group` = 0
    AND `a`.`created_at` >= "2023/01/01" AND `a`.`created_at` <= "2023/12/12"
    AND `ea`.`region_code` = "6f2cbfd8-692a-4ee4-9b16-067210bde3fc"
    AND `ea`.`district_code` IN ("c7a81174-8d01-4ae6-83e6-386ae23ee629","993701c9-2100-48ef-8b83-1167e82df13f", "53056d1c-5c65-43e8-be8e-8edf32480797")
    AND `a`.`kind_id` IN (1,2,3,4,5,6,7,8,9,10)
    AND `o`.`legal_form_id` IN (1,2,3)
    GROUP BY `e`.`owner_id`
    ORDER BY `enterprise_count` ASC
    "#;

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json_response)
}
