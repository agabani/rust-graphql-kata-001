use rust_actix_web_template_web::{startup, tracing as trace};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    trace::init(trace::configure("info"));

    let (server, port, configuration) = startup::run(&[]);

    tracing::info!(
        http_server.host = %configuration.http_server.host,
        http_server.port = %port,
        "Starting server"
    );

    server.await
}
