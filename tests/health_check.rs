use std::net::TcpListener;

use news::{configuration::{get_configuration, DatabaseSettings}, startup::run};
use sqlx::{Connection, PgConnection, PgPool, Executor,};
use uuid::Uuid;

#[tokio::test]
async fn health_check_works(){
    let  addr = spawn_app().await;
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    let response = client
    .get(&format!("{}/health_check", &addr.address))
    .send()
    .await
    .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

}
pub struct TestApp{
    pub address:String,
    pub db_pool:PgPool
}
// Launch our application in the background ~somehow~
 async fn spawn_app() -> TestApp{
    let listener = TcpListener::bind("127.0.0.1:0")
    .expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);


    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    // random database name
    let connection_pool =  configure_database(&configuration.database)
    .await;

    let server = run(listener,connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    // format!("http://127.0.0.1:{}", port)
    TestApp{
        address,
        db_pool:connection_pool

    }
    // return testapp that includes the address and the connection pool
}
// build a PgPool without db_name
pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
     // Create database
     let mut connection = PgConnection::connect(&config.connection_string_without_db())
     .await
     .expect("Failed to connect to Postgres");
 connection
     .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
     .await
     .expect("Failed to create database.");

 // Migrate database
 let connection_pool = PgPool::connect(&config.connection_string())
     .await
     .expect("Failed to connect to Postgres.");
 sqlx::migrate!("./migrations")
     .run(&connection_pool)
     .await
     .expect("Failed to migrate the database");

 connection_pool
}


    #[tokio::test]
    async fn subscribe_returns_a_200_for_valid_form_data(){
        // Arrange
        let app = spawn_app().await;
        let configuration = get_configuration().expect("Failed to read configuration.");
        let connection_string =configuration.database.connection_string();
        
        let client = reqwest::Client::new();
        let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
        let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");


        // Assert
        assert_eq!(200, response.status().as_u16());

        // test sql 
        let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");
        assert_eq!(saved.email, "ursula_le_guin@gmail.com");
        assert_eq!(saved.name, "le guin");
    }
    #[tokio::test]
    async fn subscribe_returns_a_400_when_data_is_missing(){
        // Arrange
        let app = spawn_app().await;
        let client = reqwest::Client::new();
        let test_cases = vec![
            ("name=le%20guin", "missing the email"),
            ("email=ursula_le_guin%40gmail.com", "missing the name"),
            ("", "missing both name and email"),
        ];
        for (invalid_body, error_message) in test_cases {
            let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
            // Assert
            assert_eq!(400, response.status().as_u16(), "The API did not fail with 400 Bad Request when the payload was {}.", error_message);
        }
    }

