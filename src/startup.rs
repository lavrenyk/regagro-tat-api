//! src/startup.rs
use crate::routes::{
    get_animal_count_by_district, get_animal_count_by_kind_for_period, health_check,
};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sqlx::MySqlPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: MySqlPool) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let db_pool = web::Data::new(db_pool);
    // Capture `connection` from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            // .route("/subscriptions", web::post().to(subscribe))
            .route(
                "/api/analytics/animals/getAnimalCountByDistrict",
                web::get().to(get_animal_count_by_district),
            )
            .route(
                "/api/analytics/animals/getAnimalCountByKindForPeriod",
                web::get().to(get_animal_count_by_kind_for_period),
            )
            // Register the connection as part of the application
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
