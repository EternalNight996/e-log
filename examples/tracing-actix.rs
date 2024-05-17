#[cfg(feature = "tracing-actix")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  use std::path::Path;

  use actix_web::{web, App, HttpServer};
  use e_log::{appender, LogTarget};
  use e_log::appender::non_blocking::WorkerGuard;
  use e_log::logger::{
    init_subscriber,
    subscriber::{self, layer::SubscriberExt as _, Registry},
    web::TracingLogger,
  };
  use e_log::{preload::*, Level, __private::subscriber::Subscriber};
  use e_utils::parse::AutoPath as _;
  use e_utils::system::time::{TimezoneStrategy, LOG_FILE_DATE_FORMAT};

  async fn hello() -> &'static str {
    info!("hello");
    warn!("hello");
    debug!("hello");
    error!("hello");
    trace!("hello");
    "Hello world!"
  }

  async fn personal_hello(name: web::Path<String>) -> String {
    info!("hello");
    warn!("hello");
    debug!("hello");
    error!("hello");
    trace!("hello");
    format!("Hello {}!", name)
  }

  /// Compose multiple layers into a `tracing`'s subscriber.
  fn get_subscriber(level: Level, enabled: Vec<LogTarget>) -> (impl Subscriber + Send + Sync, WorkerGuard) {
    let now = TimezoneStrategy::UseUtc
      .get_now()
      .format(LOG_FILE_DATE_FORMAT);
    let fname = format!("test.{}.log", now);
    let folder = Path::new("logs");
    folder.auto_create_dir().unwrap();

    let roll = appender::rolling::never(folder, fname);
    let (f, guard) = appender::non_blocking(roll);
    let file_layer = subscriber::fmt::layer()
      .with_ansi(false)
      .with_writer(f);
    let base_layer = subscriber::fmt::layer()
      .with_writer(std::io::stdout);
    (
      Registry::default()
        .with(level.to_level_filter())
        .with(file_layer)
        .with(base_layer),
      guard,
    )
  }

  let (subscriber, _guard) = get_subscriber(Level::Info);
  init_subscriber(subscriber);

  HttpServer::new(move || {
    App::new()
      .wrap(TracingLogger::default())
      .service(web::resource("/hello").to(hello))
      .service(web::resource("/hello/{name}").to(personal_hello))
  })
  .bind("127.0.0.1:12345")?
  .run()
  .await?;

  Ok(())
}

#[cfg(not(feature = "tracing-actix"))]
fn main() {}
