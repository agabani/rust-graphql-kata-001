use tracing::{subscriber, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

/// Initialize tracing using a subscriber.
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rust_actix_web_template_web::tracing;
///
/// let subscriber = tracing::configure("info");
///
/// tracing::init(subscriber);
/// ```
pub fn init(subscriber: impl Subscriber + Send + Sync) {
    subscriber::set_global_default(subscriber).expect("setting tracing default failed.");
}

/// Returns a preconfigured tracing subscriber.
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rust_actix_web_template_web::tracing;
///
/// let subscriber = tracing::configure("info");
/// ```
///
/// # Panics
///
/// Will panic if environment filter cannot construct.
pub fn configure(level: &str) -> impl Subscriber + Send + Sync {
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(level))
        .unwrap();

    let fmt_layer =
        BunyanFormattingLayer::new("rust-actix-web-template.web".to_string(), std::io::stdout);

    Registry::default()
        .with(filter_layer)
        .with(JsonStorageLayer)
        .with(fmt_layer)
}

/// Transparent passthrough tracing for Result types.
pub trait TraceErrorExt<T, E: std::fmt::Display> {
    /// # Errors
    ///
    /// Will return `Err` transparently when the input is also an `Err`.
    fn trace_err(self) -> Result<T, E>;
}

impl<T, E: std::fmt::Display> TraceErrorExt<T, E> for Result<T, E> {
    /// Traces value of Err. Does nothing on Ok.
    ///
    /// # Examples
    ///
    /// Basic `Ok` usage:
    /// ```
    /// use rust_actix_web_template_web::tracing::TraceErrorExt;
    ///
    /// let result: Result<i32, String> = Ok(42).trace_err();
    /// ```
    ///
    /// Basic `Err` usage:
    /// ```
    /// use rust_actix_web_template_web::tracing::TraceErrorExt;
    ///
    /// let result: Result<i32, String> = Err("Something went wrong".to_string()).trace_err();
    /// ```
    ///
    fn trace_err(self) -> Result<T, E> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                tracing::error!(error = %e);
                Err(e)
            }
        }
    }
}
