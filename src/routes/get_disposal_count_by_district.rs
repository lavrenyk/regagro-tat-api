//! src/routes/get_disposal_count_by_district.rs

//TODO: Добавить возможность выгружать данные из городов районного назначения

use actix_web::{web, HttpResponse};
use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, MySqlPool};

use crate::{helpers::*, structs::QueryData};

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct SqlItemResponse {
    district_code: Option<String>,
    count: i64,
    public_sector_count: i64,
    private_sector_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseItem {
    id: u64,
    name: String,
    count: i64,
    public_sector_count: i64,
    private_sector_count: i64,
}

pub async fn get_disposal_count_by_district(
    data: web::Query<QueryData>,
    _pool: web::Data<MySqlPool>,
) -> HttpResponse {
    //TODO: Перенести проверку в функцию
    // Обработка данных региона `id` и `guid`
    let region_id: u32 = {
        match data.region_id {
            Some(data) => data,
            None => 0,
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

    // Подготавливаем соединение с базой данных
    let connection =
        MySqlPool::connect("mysql://mp_analytic:8Nlr7fDQNwmniu6h@vo.regagro.ru:33636").await;

    let query = format!(
        r#"
        SELECT `ea`.`district_code` as `district_code`, 
        
        CAST( SUM(`da`.`count`) AS INTEGER) as `count`,
        CAST( SUM(CASE WHEN `o`.`legal_form_id` IN (1,3,4) THEN `da`.`count` ELSE 0 END) AS INTEGER) as `public_sector_count`,
        CAST( SUM(CASE WHEN `o`.`legal_form_id` = 2 THEN 1 ELSE `da`.`count` END) AS INTEGER) as `private_sector_count`
        
        FROM `regagro_3_0`.`disposal_list_animals` as `da`
        LEFT JOIN `regagro_3_0`.`animals` as `a` ON `da`.`animal_id` = `a`.`id`
        LEFT JOIN `regagro_3_0_handbooks`.`kinds` as `k` ON `a`.`kind_id` = `k`.`id`
        LEFT JOIN `regagro_3_0`.`enterprises` as `e` ON `a`.`enterprise_id` = `e`.`id`
        LEFT JOIN `regagro_3_0`.`enterprise_addresses` as `ea` ON `ea`.`enterprise_id` = `e`.`id`
        LEFT JOIN `regagro_3_0`.`disposal_lists` as `dl` on `dl`.`id` = `da`.`disposal_list_id`
        LEFT JOIN `regagro_3_0`.`owners` as `o` ON `e`.`owner_id` = `o`.`id`
        
        WHERE `a`.`is_super_group` = 0 AND `dl`.`disposal_status_id` = 2
        
        AND `dl`.`activated_at` >= '{}' AND `dl`.`activated_at` <= '{}'
        AND `ea`.`region_code` = '{}'
        AND `ea`.`district_code` IN ({})
        
        GROUP BY `ea`.`district_code` 
        "#,
        date_from, date_to, region_guid, districts_filter
    );

    let mut sql_response: Vec<SqlItemResponse> = vec![];

    match connection {
        Err(err) => {
            println!("Cannot connect to database [{}]", err.to_string());
        }
        Ok(pool) => {
            println!("Connected to database successfully.");
            let result_all: Result<Vec<SqlItemResponse>, _> =
                sqlx::query_as(&query).fetch_all(&pool).await;
            sql_response = result_all.unwrap_or(vec![]);
        }
    }

    let mut response: Vec<ResponseItem> = vec![];

    for sql_item in sql_response {
        dbg!(&sql_item);
        let (id, name) =
            get_district_name_by_id(&region_districts, &sql_item.district_code.unwrap().as_str());

        let response_item = ResponseItem {
            id: id.try_into().unwrap(),
            name,
            count: sql_item.count,
            public_sector_count: sql_item.public_sector_count,
            private_sector_count: sql_item.private_sector_count,
        };

        response.push(response_item);
    }

    let json_response = json!(response);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json_response)
}
