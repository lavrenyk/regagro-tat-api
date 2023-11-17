//! tests/health_checks.rs
use sqlx::{Connection, Executor, MySqlConnection, MySqlPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::run;

// `tokio::test` is the testing equivalent of `tokio::main`
// it also spares you from having to specify the `#[test]` attribute
//
// You can inspect what code gets generated using
// `cargo-expand --test health_check` (<- name of the test file)

pub struct TestApp {
    pub address: String,
    pub db_pool: MySqlPool,
}

/// Spin up an instance of our application
async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port(); // Retrieving the port assigned by th OS
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address.");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

/// Configuring our test db
pub async fn configure_database(config: &DatabaseSettings) -> MySqlPool {
    // Create database
    // let mut connection = MySqlConnection::connect(&config.connection_string_without_db())
    //     .await
    //     .expect("Failed to connect to MySql");
    let mut connection = MySqlConnection::connect("mysql://root:kakeepoo@127.0.0.1:3306/")
        .await
        .expect("Failed to connect to MySql");

    let create_db_query = format!(r#"CREATE DATABASE `{}`;"#, &config.database_name);

    dbg!(&create_db_query);

    connection
        .execute(create_db_query.as_str())
        .await
        .expect("Failed to create database");

    dbg!(&config.connection_string());

    // Migrate database
    let connection_pool = MySqlPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to MySql");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn get_animal_count_by_district_works_without_parameters() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
}

// Упрощенный тест запроса, необходимо изменить его таким образом,
// чтобы он обрабатывал входящий запрос API
// #[tokio::test]
// async fn get_animals_by_period() {
//     // Arrange
//     let app = spawn_app().await;
//     let client = reqwest::Client::new();

//     // Act
//     // let reqwest = "region_id=13&date_reg_from=2023-01-01&date_reg_to=2023-12-01&kinds=1,2,3,4,5,6,7,8,9,10&districts=170,171,172,173,174,175,176";
//     let kind_id = "1".to_string();
//     let date_reg_from = "2023-01-01".to_string();
//     let date_reg_to = "2023-12-01".to_string();
//     let district_code = "3b67dc8e-79b1-43f4-8f9e-2b4990a1a922".to_string();

//     let get_db_data = sqlx::query!(
//         r#"
//         SELECT * FROM animals;
//         "#,
//     );

//     dbg!(get_db_data);
// }

// #[tokio::test]
// async fn subscribe_returns_a_200_for_valid_form_data() {
//     // Arrange
//     let app = spawn_app().await;
//     let client = reqwest::Client::new();

//     // Act
//     let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
//     let response = client
//         .post(&format!("{}/subscriptions", &app.address))
//         .header("Content-Type", "application/x-www-form-urlencoded")
//         .body(body)
//         .send()
//         .await
//         .expect("Failed to execute request.");

//     // Assert
//     assert_eq!(200, response.status().as_u16());

//     let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
//         .fetch_one(&app.db_pool)
//         .await
//         .expect("Failed to fetch saved subscription");

//     assert_eq!(saved.email, "ursula_le_guin@gmail.com");
//     assert_eq!(saved.name, "le guin");
// }

// #[tokio::test]
// async fn subscribe_returns_a_400_when_data_is_missing() {
//     // Arrange
//     let app = spawn_app().await;
//     let client = reqwest::Client::new();
//     let test_cases = vec![
//         ("name=le%20guin", "missing the email"),
//         ("email=ursula_le_guin%40gmail.com", "missing the name"),
//         ("", "missing both name and email"),
//     ];

//     for (invalid_body, error_message) in test_cases {
//         // Act
//         let response = client
//             .post(&format!("{}/subscriptions", &app.address))
//             .header("Content-Type", "application/x-www-form-urlencoded")
//             .body(invalid_body)
//             .send()
//             .await
//             .expect("Failed to execute request.");

//         // Assert
//         assert_eq!(
//             400,
//             response.status().as_u16(),
//             // Additional customized error message on test failure
//             "The API did not fail with 400 Bad Request when the payload was {}",
//             error_message
//         )
//     }
// }
