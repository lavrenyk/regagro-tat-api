use std::fs;

use serde_json::Value;

pub fn animals_filter_query(animals: &str) -> String {
    let codes: Vec<&str> = animals.split(",").collect();
    dbg!(&codes);
    let mut query_filter = "".to_string();
    let file_path = "src/data/animals.json".to_owned();

    // Грузим данные из файла в переменную
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    let contents = contents.as_str();
    // переводим в JSON
    let object: Value = serde_json::from_str(contents).unwrap();

    for (i, code) in codes.iter().enumerate() {
        dbg!(&code);
        let mut regagro_code_v3: String = "".to_string();
        for i in 0..12 {
            if &object[i]["regagro_code"].as_str().unwrap() == code {
                regagro_code_v3 = object[i]["regagro_code_v3"].as_str().unwrap().to_string();
            }
        }

        dbg!(&regagro_code_v3);

        if &regagro_code_v3 == "" {
            continue;
        }

        if i == 0 {
            query_filter = format!("a.kind_id='{}'", regagro_code_v3);
        } else {
            // dbg!(regagro_code_v3);

            query_filter = format!("{} OR a.kind_id='{}'", query_filter, regagro_code_v3);
        }
    }

    query_filter
}

pub fn district_filter_query(districts: &str) -> String {
    dbg!(&districts);
    let codes: Vec<&str> = districts.split(",").collect();
    let mut query_filter = "".to_string();
    let file_path = "src/data/districts.json".to_owned();

    // Грузим данные из файла в переменную
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    let contents = contents.as_str();

    // переводим в JSON
    let object: Value = serde_json::from_str(contents).unwrap();

    for (i, code) in codes.iter().enumerate() {
        // получаем guid дистрикта
        let mut guid = "".to_string();
        for i in 0..44 {
            if object[i]["id"] == code.parse::<i64>().unwrap() {
                guid = object[i]["guid"].to_string();
            }
        }

        // Обрезаем кавычки
        let guid: &str = &guid.as_str()[1..guid.len() - 1];

        if i == 0 {
            query_filter = format!("ea.district_code='{}'", guid);
        } else {
            query_filter = format!("{} OR ea.district_code='{}'", query_filter, guid);
        }
    }

    query_filter
}

pub fn all_districts_filter() -> String {
    let mut all_districts = "264".to_string();
    for i in 265..305 {
        all_districts = format!("{},{}", all_districts, i);
    }
    district_filter_query(all_districts.as_str())
}
