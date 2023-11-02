//! src/routes/health_check.rs

use actix_web::HttpResponse;
use sqlx::{Error, MySql, MySqlPool, Pool};

pub async fn health_check() -> HttpResponse {
    let connection = MySqlPool::connect("mysql://mp_analytic:8Nlr7fDQNwmniu6h@vo.regagro.ru:33633/regagro_3_0").await;
    
    let district_code = "bb9c86eb-30de-4b8f-9ea8-9edc71fc0488".to_string();
    let query = format!(
        "SELECT a.id FROM animals AS a LEFT JOIN enterprises AS e ON a.enterprise_id=e.id LEFT JOIN enterprise_addresses AS ea ON ea.enterprise_id=e.id WHERE ea.district_code='{}';", district_code
    );

    dbg!(&connection);

    match connection {
        Err(err) => {
            println!("Cannot connect to database [{}]", err.to_string());
        }        
        Ok(pool) => {
            println!("Connected to database successfully.");
            let query_result = sqlx::query(
                query.as_str())
                .fetch_all(&pool).await.unwrap();

            println!("Number of Animals selected: {:?}", query_result.len());
        }
    }

    HttpResponse::Ok().finish()
}
