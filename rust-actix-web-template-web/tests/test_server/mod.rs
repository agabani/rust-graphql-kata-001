use rust_actix_web_template_web::startup;

pub struct TestServer {
    pub address: String,
}

impl TestServer {
    pub async fn spawn(overrides: &[(&str, &str)]) -> Self {
        let defaults = &[("http_server.host", "127.0.0.1"), ("http_server.port", "0")];

        let (server, port, _configuration) = startup::run(&[defaults, overrides].concat());

        // configure the http server and dependencies here.

        let _ = tokio::spawn(server);

        Self {
            address: format!("http://127.0.0.1:{}", port),
        }
    }
}
