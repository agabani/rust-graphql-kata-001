use rust_graphql_kata_001_web::startup;
use uuid::Uuid;

pub struct TestServer {
    pub address: String,
}

impl TestServer {
    pub async fn spawn(overrides: &[(&str, &str)]) -> Self {
        let defaults = &[
            ("http_server.host", "127.0.0.1"),
            ("http_server.port", "0"),
            (
                "postgres.database_name",
                &format!("test-{}", Uuid::new_v4()),
            ),
            ("postgres.host", "127.0.0.1"),
            ("postgres.password", "password"),
            ("postgres.port", "5432"),
            ("postgres.require_ssl", "false"),
            ("postgres.username", "postgres"),
        ];

        let (server, port, configuration) = startup::run(&[defaults, overrides].concat());

        let postgres_server_pool = configuration.postgres.server_pool();

        sqlx::query(&format!(
            r#"CREATE DATABASE "{}""#,
            configuration.postgres.database_name
        ))
        .execute(&postgres_server_pool)
        .await
        .expect("Failed to create database");

        let postgres_database_pool = configuration.postgres.database_pool();

        sqlx::migrate!("../migrations")
            .run(&postgres_database_pool)
            .await
            .expect("Failed to migrate database");

        let _ = tokio::spawn(server);

        Self {
            address: format!("http://127.0.0.1:{}", port),
        }
    }
}
