use config::Config;
use std::net::TcpListener;

#[derive(serde::Deserialize)]
pub struct Configuration {
    pub http_server: HttpServer,
}

impl Configuration {
    pub fn load(overrides: &[(&str, &str)]) -> Result<Configuration, config::ConfigError> {
        let mut config = Config::default();

        config.merge(config::Environment::with_prefix("APP_").separator("__"))?;

        for &(key, value) in overrides {
            config.set(key, value)?;
        }

        config.try_into()
    }
}

#[derive(serde::Deserialize)]
pub struct HttpServer {
    pub host: String,
    pub port: u16,
}

impl HttpServer {
    pub fn tcp_listener(&self) -> std::io::Result<TcpListener> {
        TcpListener::bind(format!("{}:{}", self.host, self.port))
    }
}
