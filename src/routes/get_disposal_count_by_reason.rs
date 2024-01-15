//! src/routes/get_disposal_count_by_reason.rs
//! GET {baseUrlErr}/api/analytics/disposals/getDisposalCountByReason?region_id={regionId}&date_reg_from={dateRegFrom}&date_reg_to={dateRegTo}&districts={districts}

use actix_web::{web, HttpResponse};
use chrono::Local;
use serde_json::json;
use sqlx::MySqlPool;

use crate::{
    helpers::{all_districts_filter, district_filter_query, get_region_districts, get_region_guid},
    structs::QueryData,
};

pub async fn get_disposal_count_by_reason(
    data: web::Query<QueryData>,
    _pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let json_response = json!([]);

    // Parse the region ID
    let region_id: u32 = {
        match data.region_id {
            Some(data) => data,
            None => 0,
        }
    };

    let date_from: String = {
        match &data.date_reg_from {
            Some(date_from) => date_from.to_string(),
            None => "2020-01-01".to_string(),
        }
    };

    let date_to: String = {
        match &data.date_reg_to {
            Some(date_to) => date_to.to_string(),
            None => Local::now().format("%Y-%m-%d").to_string(),
        }
    };

    let region_guid = get_region_guid(region_id); // получаем GUID региона

    // Грузим данные по районам в регионе
    let region_districts = get_region_districts(region_id).await;

    // Получаем список GUID районов, для дальнейшего запроса в БД
    let districts_filter: String = {
        match &data.districts {
            Some(data) => district_filter_query(&data.as_str(), &region_districts),
            None => all_districts_filter(&region_districts),
        }
    };

    // Подготавливаем соединение с базой данных
    let connection =
        MySqlPool::connect("mysql://mp_analytic:8Nlr7fDQNwmniu6h@vo.regagro.ru:33636/regagro_3_0")
            .await;

    let sql_query = format!(
        r#"
        SELECT dr.id as id, dr.name as name, 
        SUM(da.count) as count,
        CAST( SUM(CASE WHEN o.legal_form_id IN (1,3,4) THEN da.count ELSE 0 END) AS UNSIGNED) as public_sector_count,
        CAST( SUM(CASE WHEN o.legal_form_id = 2 THEN 1 ELSE da.count END) AS UNSIGNED) as private_sector_count

        FROM regagro_3_0.disposal_list_animals as da
        LEFT JOIN regagro_3_0.animals as a ON da.animal_id = a.id
        LEFT JOIN regagro_3_0_handbooks.kinds as k ON a.kind_id = k.id
        LEFT JOIN regagro_3_0.enterprises as e ON a.enterprise_id = e.id
        LEFT JOIN regagro_3_0.enterprise_addresses as ea ON ea.enterprise_id = e.id
        LEFT JOIN regagro_3_0.disposal_lists as dl on dl.id = da.disposal_list_id
        LEFT JOIN regagro_3_0_handbooks.disposal_reasons as dr on dl.disposal_reason_id = dr.id
LEFT JOIN `regagro_3_0`.`owners` as `o` ON `e`.`owner_id` = `o`.`id`
WHERE `a`.`is_super_group` = 0 AND `dl`.`disposal_status_id` = 2
AND `dl`.`activated_at` >= "{}" AND `dl`.`activated_at` <= "{}"
AND `ea`.`region_code` = "{}"
AND `ea`.`district_code` IN ({})
GROUP BY `dr`.`id`;
    "#,
        &date_from, &date_to, &region_guid, &districts_filter
    );

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json_response)
}
