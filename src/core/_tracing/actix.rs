use base::LogTracer;
use std::sync::Once;
use tracing::{subscriber::set_global_default, Subscriber};
pub use tracing_actix_web as web;
pub use tracing_bunyan_formatter as formatter;
pub use tracing_log as base;
#[cfg(feature = "tracing-subscriber")]
pub use tracing_subscriber as subscriber;
static INIT: Once = Once::new();

/// # Examples
///```rust
/// use std::fs;
/// use std::path::Path;
/// use actix_web::body::MessageBody;
/// use actix_web::dev::{ServiceRequest, ServiceResponse};
/// use actix_web::{web, App, Error, HttpServer};
/// use e_log::actix::{
///   formatter::{BunyanFormattingLayer, JsonStorageLayer},
///   init_subscriber,
///   subscriber::{layer::SubscriberExt as _, Registry},
///   web::{root_span, DefaultRootSpanBuilder, RootSpan, RootSpanBuilder, TracingLogger},
/// };
/// use e_log::{Level, preload::*, __private::{subscriber::Subscriber,Span}};
/// use e_utils::parse::AutoPath as _;
/// use e_utils::fs::FileOptionsExt;
/// use e_utils::system::time::{TimezoneStrategy, LOG_FILE_DATE_FORMAT};
/// /// We will define a custom root span builder to capture additional fields, specific
/// /// to our application, on top of the ones provided by `DefaultRootSpanBuilder` out of the box.
/// struct CustomRootSpanBuilder;
/// impl RootSpanBuilder for CustomRootSpanBuilder {
///   fn on_request_start(request: &ServiceRequest) -> Span {
///     // Not sure why you'd be keen to capture this, but it's an example and we try to keep it simple
///     let n_headers = request.headers().len();
///     // We set `cloud_provider` to a constant value.
///     //
///     // `name` is not known at this point - we delegate the responsibility to populate it
///     // to the `personal_hello` handler. We MUST declare the field though, otherwise
///     // `span.record("caller_name", XXX)` will just be silently ignored by `tracing`.
///     root_span!(
///       request,
///       n_headers,
///       cloud_provider = "localhost",
///       caller_name = tracing::field::Empty
///     )
///   }
///   fn on_request_end<B: MessageBody>(span: Span, outcome: &Result<ServiceResponse<B>, Error>) {
///     // Capture the standard fields when the request finishes.
///     DefaultRootSpanBuilder::on_request_end(span, outcome);
///   }
/// }
/// async fn hello() -> &'static str {
///   info!("hello");
///   warn!("hello");
///   debug!("hello");
///   error!("hello");
///   trace!("hello");
///   "Hello world!"
/// }
/// async fn personal_hello(root_span: RootSpan, name: web::Path<String>) -> String {
///   // Add more context to the root span of the request.
///   root_span.record("caller_name", &name.as_str());
///   info!("hello");
///   warn!("hello");
///   debug!("hello");
///   error!("hello");
///   trace!("hello");
///   format!("Hello {}!", name)
/// }
/// /// Compose multiple layers into a `tracing`'s subscriber.
/// fn get_subscriber(name: impl AsRef<str>, level: Level) -> impl Subscriber + Send + Sync {
///   let formatting_layer = BunyanFormattingLayer::new(name.as_ref().into(), std::io::stdout);
///   let now = TimezoneStrategy::UseUtc
///     .get_now()
///     .format(LOG_FILE_DATE_FORMAT);
///   let fpath = Path::new("logs").join(format!("test.{}.log", now));
///   fpath.auto_create_dir().unwrap();
///   let f = fs::OpenOptions::new()
///     .create(true)
///     .write(true)
///     .read(true)
///     .lock_share(e_utils::fs::FileShare::Read)
///     .open(fpath)
///     .unwrap();
///   let file_layer = BunyanFormattingLayer::new(name.as_ref().into(), f);
///   Registry::default()
///     .with(level.to_level_filter())
///     .with(JsonStorageLayer)
///     .with(formatting_layer)
///     .with(file_layer)
/// }
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///   let subscriber = get_subscriber("tracing-actix-web-demo", Level::Info);
///   init_subscriber(subscriber);
///   HttpServer::new(move || {
///     App::new()
///       .wrap(TracingLogger::default())
///       .service(web::resource("/hello").to(hello))
///       .service(web::resource("/hello/{name}").to(personal_hello))
///   })
///   .bind("127.0.0.1:12345")?
///   .run()
///   .await?;
///   Ok(())
/// }
///```
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
  INIT.call_once(|| {
    LogTracer::init().unwrap();
    set_global_default(subscriber).unwrap();
  });
}
