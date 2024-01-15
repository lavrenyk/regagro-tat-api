//! src/routes/get_enterprise_count_by_district.rs

use actix_web::{web, HttpResponse};
use serde::Serialize;
use sqlx::{FromRow, MySqlPool};

use crate::{
    helpers::{
        all_districts_filter, get_district_name_by_id, get_region_districts, get_region_guid,
    },
    structs::QueryData,
};
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct SqlItemResponse {
    id: u64,
    district_code: Option<String>,
    locality_code: Option<String>,
    name: String,
    all_count: i64,
    farm_count: i64,
    slaughterhouse_count: i64,
    meat_count: i64,
    disposal_count: i64,
    milk_count: i64,
    dairy_count: i64,
    apiary_count: i64,
    shelter_count: i64,
    krs_farm_count: i64,
    milk_farm_count: i64,
    pig_farm_count: i64,
    sheep_farm_count: i64,
    goat_farm_count: i64,
    horse_farm_count: i64,
    horse_club_count: i64,
    vivarium_count: i64,
    herd_horse_count: i64,
    deer_count: i64,
    individual_count: i64,
    lph_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseItem {
    id: u64,
    name: String,
    all_count: i64,
    farm_count: i64,
    slaughterhouse_count: i64,
    meat_count: i64,
    disposal_count: i64,
    milk_count: i64,
    dairy_count: i64,
    apiary_count: i64,
    shelter_count: i64,
    krs_farm_count: i64,
    milk_farm_count: i64,
    pig_farm_count: i64,
    sheep_farm_count: i64,
    goat_farm_count: i64,
    horse_farm_count: i64,
    horse_club_count: i64,
    vivarium_count: i64,
    herd_horse_count: i64,
    deer_count: i64,
    individual_count: i64,
    lph_count: i64,
}

pub async fn get_enterprise_count_by_district(
    data: web::Query<QueryData>,
    _pool: web::Data<MySqlPool>,
) -> HttpResponse {
    // Parse the region ID
    let region_id: u32 = {
        match data.region_id {
            Some(data) => data,
            None => 0,
        }
    };

    let region_guid = get_region_guid(region_id); // получаем GUID региона

    // Грузим данные по районам в регионе
    let region_districts = get_region_districts(region_id).await;

    let districts_guids: String = {
        match &data.districts {
            Some(data) => data.to_string(),
            None => all_districts_filter(&region_districts),
        }
    };

    let sql_query = format!(
        r#"
        SELECT ea.id AS id, ea.district_code, ea.locality_code,
        (CASE WHEN district_code IS NULL THEN locality_code ELSE district_code END) as name, 
        
        COUNT(`e`.`id`) as `all_count`,
        CAST( SUM(CASE WHEN `e`.`enterprise_type_id` = 1 THEN 1 ELSE 0 END) AS INTEGER) as `farm_count`,
        CAST( SUM(CASE WHEN `e`.`enterprise_type_id` = 2 THEN 1 ELSE 0 END) AS INTEGER) as `slaughterhouse_count`,
        CAST( SUM(CASE WHEN `e`.`enterprise_type_id` = 3 THEN 1 ELSE 0 END) AS INTEGER) as `meat_count`,
        CAST( SUM(CASE WHEN `e`.`enterprise_type_id` = 4 THEN 1 ELSE 0 END) AS INTEGER) as `disposal_count`,
        CAST( SUM(CASE WHEN `e`.`enterprise_type_id` = 5 THEN 1 ELSE 0 END) AS INTEGER) as `milk_count`,
        CAST( SUM(CASE WHEN `e`.`enterprise_type_id` = 6 THEN 1 ELSE 0 END) AS INTEGER) as `dairy_count`,
        CAST( SUM(CASE WHEN `e`.`enterprise_type_id` = 7 THEN 1 ELSE 0 END) AS INTEGER) as `apiary_count`,
        CAST( SUM(CASE WHEN `e`.`enterprise_type_id` = 8 THEN 1 ELSE 0 END) AS INTEGER) as `shelter_count`,
        CAST( SUM(CASE WHEN `e`.`enterprise_type_id` = 9 THEN 1 ELSE 0 END) AS INTEGER) as `krs_farm_count`,
        CAST( SUM(CASE WHEN `e`.`enterprise_type_id` = 10 THEN 1 ELSE 0 END) AS INTEGER) as `milk_farm_count`,
        CAST( SUM(CASE WHEN `e`.`enterprise_type_id` = 11 THEN 1 ELSE 0 END) AS INTEGER) as `pig_farm_count`,
        CAST( SUM(CASE WHEN `e`.`enterprise_type_id` = 12 THEN 1 ELSE 0 END) AS INTEGER) as `sheep_farm_count`,
        CAST( SUM(CASE WHEN `e`.`enterprise_type_id` = 13 THEN 1 ELSE 0 END) AS INTEGER) as `goat_farm_count`,
        CAST( SUM(CASE WHEN `e`.`enterprise_type_id` = 14 THEN 1 ELSE 0 END) AS INTEGER) as `horse_farm_count`,
        CAST( SUM(CASE WHEN `e`.`enterprise_type_id` = 15 THEN 1 ELSE 0 END) AS INTEGER) as `horse_club_count`,
        CAST( SUM(CASE WHEN `e`.`enterprise_type_id` = 16 THEN 1 ELSE 0 END) AS INTEGER) as `vivarium_count`,
        CAST( SUM(CASE WHEN `e`.`enterprise_type_id` = 17 THEN 1 ELSE 0 END) AS INTEGER) as `herd_horse_count`,
        CAST( SUM(CASE WHEN `e`.`enterprise_type_id` = 18 THEN 1 ELSE 0 END) AS INTEGER) as `deer_count`,
        CAST( SUM(CASE WHEN `e`.`enterprise_type_id` = 20 THEN 1 ELSE 0 END) AS INTEGER) as `individual_count`,
        CAST( SUM(CASE WHEN `e`.`enterprise_type_id` = 20 THEN 1 ELSE 0 END) AS INTEGER) as `lph_count`

        FROM `regagro_3_0`.`enterprises` as `e`
        LEFT JOIN `regagro_3_0`.`enterprise_addresses` as `ea` ON `ea`.`enterprise_id` = `e`.`id`
        
        WHERE `ea`.`region_code` = "{}"
        AND `ea`.`district_code` IN ({})

        GROUP BY `ea`.`district_code`;
        "#,
        &region_guid, &districts_guids
    );

    let mut sql_response: Vec<SqlItemResponse> = vec![];

    // Подготавливаем соединение с базой данных
    let connection =
        MySqlPool::connect("mysql://mp_analytic:8Nlr7fDQNwmniu6h@vo.regagro.ru:33636").await;

    match connection {
        Err(err) => {
            println!("Cannot connect to database [{}]", err.to_string());
        }
        Ok(pool) => {
            println!("Connected to database successfully.");
            let result_all: Result<Vec<SqlItemResponse>, _> =
                sqlx::query_as(&sql_query).fetch_all(&pool).await;
            sql_response = result_all.unwrap_or(vec![]);
        }
    }

    let mut response: Vec<ResponseItem> = vec![];

    for sql_item in &sql_response {
        let (id, name) = get_district_name_by_id(&region_districts, &sql_item.name);

        let response_item = ResponseItem {
            id: id.try_into().unwrap(),
            name,
            all_count: sql_item.all_count,
            farm_count: sql_item.farm_count,
            slaughterhouse_count: sql_item.slaughterhouse_count,
            meat_count: sql_item.meat_count,
            disposal_count: sql_item.disposal_count,
            milk_count: sql_item.milk_count,
            dairy_count: sql_item.dairy_count,
            apiary_count: sql_item.apiary_count,
            shelter_count: sql_item.shelter_count,
            krs_farm_count: sql_item.krs_farm_count,
            milk_farm_count: sql_item.milk_farm_count,
            pig_farm_count: sql_item.pig_farm_count,
            sheep_farm_count: sql_item.sheep_farm_count,
            goat_farm_count: sql_item.goat_farm_count,
            horse_farm_count: sql_item.horse_farm_count,
            horse_club_count: sql_item.horse_club_count,
            vivarium_count: sql_item.vivarium_count,
            herd_horse_count: sql_item.herd_horse_count,
            deer_count: sql_item.deer_count,
            individual_count: sql_item.individual_count,
            lph_count: sql_item.lph_count,
        };

        response.push(response_item);
    }

    HttpResponse::Ok()
        .content_type("application/json")
        .json(response)
}
