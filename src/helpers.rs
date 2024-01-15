use reqwest::{self, Client};
use serde_json::Value;
use std::fs;

use crate::structs::ErrDistrictData;

/// Загрузка списка объектов с данными районов принадлежащих региона
pub async fn get_region_districts(region_id: u32) -> Vec<ErrDistrictData> {
    let mut region_districts: Vec<ErrDistrictData> = vec![];
    let url = format!(
        "https://err.regagro.net/api/regions/{}/districts",
        region_id
    );

    //TODO: Сделать проверку токена
    const ACCESS_TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzIiwianRpIjoiNTU3MjdjODFmZDY1NDkzYTQ5YzgwMjEwZTIzYTRjMzdkZjcxMWU2NDhiN2IyY2M5ZmMyOTkyMDNmYzk4MjZhNGJjNDgwOTI1Y2ZkMWY1YzUiLCJpYXQiOjE3MDE3MTM4ODYuODQwNzc1LCJuYmYiOjE3MDE3MTM4ODYuODQwNzgsImV4cCI6MTczMzMzNjI4Ni44Mjk1MTEsInN1YiI6IiIsInNjb3BlcyI6W119.YtH-GSYPuVouXNFJac4K4lQtUH8Emi5Gm8lCsuIuGLc4IpTSgsH9azLV9BOLKipMztrkmtdDVXQ_LqG1LUVs93b3HAyl_juUboX6zmUUw5EYy5cFIWiAjkREqS5eX_DPMpTmlriaQn30gHFmf6VZ6hIzqO1J66cPcOYe6833nebp0wm-1L18puC7H0aFhb1D-vSL3mmhelgvQ6mvQbhSNqvErkrwNpO6N1LM5a0Ox5qDgtAQWQeT-ZQdIw8OktFjop_n7ba9N1EFWuGFLfW5y0OEK4UiS-KxTnjf_oqPaG795XrPW2T-7rqfr6eiK3zrQrRch_4J1xSNLy1UU86i2Fgfi8ijpD29AVHqnIteFx7g42cqTH4xltkmG08G5IAZYkpTzJWHxmTvZCNXXfBTkLzhVjegW3H4mY23V8HPU6sDVRU3xdIW3rp7d8K8cjTrw68f6vC2HV9ahfl721kCYUHVn79x6W52DXQbVEQBGPh5e1nbWBAb1ocYKrgTzCh62TKN_2eiFqMXWdC0C-uy77H7lKsR-wL5GEIFCmdWgQmg-Y0Zvg9E2FHLH2L9G4WcQSF0DkJk9JNyYZyjVaX330O0jISNBry6LdyRBczNqhxAnY-S62YfYoBK13DYqI_hXwuCb_pvZiJpWlMNZTcbpBx28EAQ1i_liyOJXDCj2Mo";
    // Создаем клиента для запроса данные о районах региона
    let result = Client::new()
        .get(url)
        .bearer_auth(ACCESS_TOKEN)
        .send()
        .await
        .unwrap()
        .json::<Vec<ErrDistrictData>>()
        .await;

    match result {
        Ok(json_data) => {
            region_districts = json_data;
        }
        Err(err) => {
            dbg!(err);
        }
    }

    region_districts
}

pub fn animals_filter_query(animals: &str) -> String {
    let codes: Vec<&str> = animals.split(",").collect();
    // dbg!(&codes);
    let mut query_filter = "".to_string();
    let file_path = "src/data/animals.json".to_owned();

    // Грузим данные из файла в переменную
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    let contents = contents.as_str();
    // переводим в JSON
    let object: Value = serde_json::from_str(contents).unwrap();

    for (i, code) in codes.iter().enumerate() {
        let mut regagro_code_v3: String = "".to_string();
        for i in 0..12 {
            if &object[i]["id"].as_str().unwrap() == code {
                regagro_code_v3 = object[i]["regagro_code_v3"].as_str().unwrap().to_string();
            }
        }

        if &regagro_code_v3 == "" {
            continue;
        }

        if i == 0 {
            query_filter = format!("a.kind_id='{}'", regagro_code_v3);
        } else {
            query_filter = format!("{} OR a.kind_id='{}'", query_filter, regagro_code_v3);
        }
    }

    query_filter
}

pub fn district_filter_query(
    districts_ids: &str,
    region_districts: &Vec<ErrDistrictData>,
) -> String {
    let codes: Vec<&str> = districts_ids.split(",").collect();
    let mut query_filter = String::new();

    for (i, code) in codes.iter().enumerate() {
        // получаем guid дистрикта
        let mut district_guid = String::new();
        // Try to fing district in regions data by `id` and return `district_guid`
        for district in region_districts {
            if district.id == code.parse().unwrap_or(0) {
                district_guid = district.guid.clone();
            }
        }

        if i == 0 {
            query_filter = format!("'{}'", district_guid);
        } else {
            query_filter = format!("{}, '{}'", query_filter, district_guid);
        }
    }

    query_filter
}

pub fn all_districts_filter(region_districts: &Vec<ErrDistrictData>) -> String {
    let mut districts_guid = String::new();

    for district in region_districts {
        if districts_guid.len() > 0 {
            districts_guid = format!("{}, '{}'", districts_guid, district.guid);
        } else {
            districts_guid = format!("'{}'", district.guid);
        }
    }

    districts_guid
}

pub fn get_district_names(districts: &str) -> String {
    let codes: Vec<&str> = districts.split(",").collect();
    let mut district_names = "".to_string();
    let file_path = "src/data/districts.json".to_owned();

    // Грузим данные из файла в переменную
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    let contents = contents.as_str();

    // переводим в JSON
    let object: Value = serde_json::from_str(contents).unwrap();

    for (i, code) in codes.iter().enumerate() {
        // получаем guid дистрикта
        let mut name = "".to_string();
        for i in 0..44 {
            if object[i]["id"] == code.parse::<i64>().unwrap() {
                name = object[i]["name"].to_string();
            }
        }

        // Обрезаем кавычки
        let name: &str = &name.as_str()[1..name.len() - 1];

        if i == 0 {
            district_names = format!("'{}'", name);
        } else {
            district_names = format!("{}, '{}'", district_names, name);
        }
    }

    district_names
}

pub fn load_json_file(name: &str) -> Value {
    //! СДЕЛАТЬ ПРОВЕРКУ ИМЕНИ ФАЙЛА!!!
    let file_path = format!("src/data/{}.json", name).to_owned();
    // Грузим данные из файла в переменную
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    let contents = contents.as_str();

    // переводим данные из файла в JSON
    let json_data: Value = serde_json::from_str(contents).unwrap();

    json_data
}

pub fn get_district_name_by_id(
    region_districts: &Vec<ErrDistrictData>,
    district_guid: &str,
) -> (i64, String) {
    // let districts_data = load_districts_data();

    let mut district_id = 0;
    let mut district_name = String::new();

    for district in region_districts {
        if district.guid == district_guid {
            district_id = district.id;
            district_name = district.name.to_owned();
            break;
        }
    }

    (district_id, district_name)
}

pub fn get_all_kind_ids() -> String {
    let mut kind_ids = "1".to_string();
    // Определяем количество типов животных
    for i in 2..16 {
        kind_ids = format!("{},{}", kind_ids, i);
    }

    kind_ids.to_string()
}

pub fn get_kind_name_by_id(kind_id: &u64) -> (String, String) {
    let animals_json_data = load_json_file("animals");

    let mut kind_name = String::new();
    let mut kind_view = String::new();

    for animal in animals_json_data.as_array().unwrap() {
        if &(animal["id"]).as_u64().unwrap() == kind_id {
            kind_name = animal["name"].as_str().unwrap().to_string();
            kind_view = animal["view"].as_str().unwrap().to_string();
        }
    }

    (kind_name, kind_view)
}

/// Return `region_guid` by `id`
pub fn get_region_guid(id: u32) -> String {
    // set empty `region_guid`
    let mut region_guid = String::new();

    // Load regions data
    let regions = load_json_file("regions");

    // Try to fing region in regions data by `id` and return `region_guid`
    for region in regions.as_array().unwrap() {
        if (&region["id"]) == id {
            region_guid = region["guid"].as_str().unwrap().to_string();
        }
    }

    region_guid
}
