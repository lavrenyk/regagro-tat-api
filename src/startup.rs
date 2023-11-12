//! src/startup.rs
use crate::routes::{
    get_animal_count_by_district, get_animal_count_by_kind_for_period,
    get_apiary_count_by_district, get_disposal_count_by_district, get_disposal_count_by_reason,
    get_enterprise_count_by_district, get_livestock_age_group_count_by_districts,
    get_livestock_age_group_count_by_pos_councils, get_livestock_count_by_companies,
    get_livestock_count_by_districts, get_livestock_count_by_pos_councils,
    get_researches_by_diseases, get_researches_by_districts, get_researches_by_kinds,
    get_researches_by_pos_councils, get_vaccinations_by_diseases, get_vaccinations_by_districts,
    get_vaccinations_by_kinds, get_vaccinations_by_pos_councils, health_check,
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
            .route(
                "/api/analytics/animals/getApiaryCountByDistrict",
                web::get().to(get_apiary_count_by_district),
            )
            .route(
                "/api/analytics/disposals/getDisposalCountByDistrict",
                web::get().to(get_disposal_count_by_district),
            )
            .route(
                "/api/analytics/disposals/getDisposalCountByReason",
                web::get().to(get_disposal_count_by_reason),
            )
            .route(
                "/api/analytics/enterprises/getEnterpriseCountByDistrict",
                web::get().to(get_enterprise_count_by_district),
            )
            .route(
                "/api/reports/getLivestockAgeGroupCountByDistricts",
                web::get().to(get_livestock_age_group_count_by_districts),
            )
            .route(
                "/api/reports/getLivestockAgeGroupCountByPosCouncils",
                web::get().to(get_livestock_age_group_count_by_pos_councils),
            )
            .route(
                "/api/reports/getLivestockCountByCompanies",
                web::get().to(get_livestock_count_by_companies),
            )
            .route(
                "/api/reports/getLivestockCountByDistricts",
                web::get().to(get_livestock_count_by_districts),
            )
            .route(
                "/api/reports/getLivestockCountByPosCouncils",
                web::get().to(get_livestock_count_by_pos_councils),
            )
            .route(
                "/api/analytics/researches/getResearchesByDiseases",
                web::get().to(get_researches_by_diseases),
            )
            .route(
                "/api/analytics/researches/getResearchesByDistricts",
                web::get().to(get_researches_by_districts),
            )
            .route(
                "/api/analytics/researches/getResearchesByKinds",
                web::get().to(get_researches_by_kinds),
            )
            .route(
                "/api/analytics/researches/getResearchesByPosCouncils",
                web::get().to(get_researches_by_pos_councils),
            )
            .route(
                "/api/v2/analytics/vaccinations/getVaccinationsByDiseases",
                web::get().to(get_vaccinations_by_diseases),
            )
            .route(
                "/api/v2/analytics/vaccinations/getVaccinationsByDistricts",
                web::get().to(get_vaccinations_by_districts),
            )
            .route(
                "/api/v2/analytics/vaccinations/getVaccinationsByKinds",
                web::get().to(get_vaccinations_by_kinds),
            )
            .route(
                "/api/v2/analytics/vaccinations/getVaccinationsByPosCouncils",
                web::get().to(get_vaccinations_by_pos_councils),
            )
            // Register the connection as part of the application
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
