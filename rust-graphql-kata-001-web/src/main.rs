use rust_graphql_kata_001_web::{startup, tracing as trace};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    trace::init(trace::configure("info"));

    let (server, port, configuration) = startup::run(&[]);

    configuration.postgres.migrate().await;

    tracing::info!(
        http_server.host = %configuration.http_server.host,
        http_server.port = %port,
        "Starting server"
    );

    server.await
}
