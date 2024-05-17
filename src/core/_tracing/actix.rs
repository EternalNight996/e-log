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
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///   use std::path::Path;
///   use actix_web::{web, App, HttpServer};
///   use e_log::appender;
///   use e_log::appender::non_blocking::WorkerGuard;
///   use e_log::logger::{
///     init_subscriber,
///     subscriber::{self, layer::SubscriberExt as _, Registry},
///     web::TracingLogger,
///   };
///   use e_log::{preload::*, Level, __private::subscriber::Subscriber};
///   use e_utils::parse::AutoPath as _;
///   async fn hello() -> &'static str {
///     info!("hello");
///     warn!("hello");
///     debug!("hello");
///     error!("hello");
///     trace!("hello");
///     "Hello world!"
///   }
///   /// Compose multiple layers into a `tracing`'s subscriber.
///   fn get_subscriber(level: Level) -> (impl Subscriber + Send + Sync, Vec<WorkerGuard>) {
///     let fname = format!("test.log");
///     let folder = Path::new("logs");
///     folder.auto_create_dir().unwrap();
///     let roll = appender::rolling::daily(folder, fname, e_log::FileShare::Read);
///     let (f, guard) = appender::non_blocking(roll);
///     let (f2, guard2) = appender::non_blocking(std::io::stdout());
///     let file_layer = subscriber::fmt::layer().with_ansi(false).with_writer(f);
///     let base_layer = subscriber::fmt::layer().with_ansi(false).with_writer(f2);
///     let def = Registry::default()
///       .with(level.to_level_filter())
///       .with(base_layer)
///       .with(file_layer);
///     (def, vec![guard, guard2])
///   }
///   let (subscriber, _guards) = get_subscriber(Level::Info);
///   init_subscriber(subscriber);
///   HttpServer::new(move || {
///     App::new()
///       .wrap(TracingLogger::default())
///       .service(web::resource("/hello").to(hello))
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
