use std::sync::Once;
use tracing::{subscriber::set_global_default, Subscriber};
#[cfg(feature = "tracing-log")]
pub use tracing_log;
static INIT: Once = Once::new();
///
pub mod __private;

cfg_tracing_appender! {
    /// tracing-appender = { version = "0.2" }
    pub mod appender;
}

#[cfg(feature = "tracing-subscriber")]
pub use tracing_subscriber as subscriber;



/// # Examples
///```rust
/// // e_log = {version="0.2", features=["tracing","panic","dialog","tracing-appender","tracing-subscriber"]}
/// // e_utils = "0.3"
/// fn main() -> e_utils::Result<()> {
///   use e_log::appender::non_blocking::WorkerGuard;
///   use e_log::subscriber::layer::SubscriberExt as _;
///   use e_log::subscriber::{self, Registry};
///   use e_log::{appender, init_subscriber};
///   use e_log::{preload::*, Level, __private::subscriber::Subscriber};
///   use e_utils::parse::AutoPath as _;
///   use std::env::current_dir;
///   use std::path::Path;
///   use tracing_subscriber::fmt::time::ChronoUtc;
///   /// Compose multiple layers into a `tracing`'s subscriber.
///   pub fn get_subscriber(
///     folder: impl AsRef<Path>,
///     fname: impl AsRef<Path>,
///     level: Level,
///   ) -> (impl Subscriber + Send + Sync, Vec<WorkerGuard>) {
///     let roll = appender::rolling::daily(folder, fname, e_log::FileShare::Read);
///     let (f, guard) = appender::non_blocking(roll);
///     let (f2, guard2) = appender::non_blocking(std::io::stdout());
///     let file_layer = subscriber::fmt::layer()
///       .with_ansi(false)
///       .with_target(true)
///       .with_writer(f);
///     let timer = ChronoUtc::new("[%F %H:%M:%S]".to_owned());
///     let base_layer = subscriber::fmt::layer()
///       .with_timer(timer)
///       .with_ansi(false)
///       .with_target(true)
///       .with_writer(f2);
///     let def = Registry::default()
///       .with(level.to_level_filter())
///       .with(base_layer)
///       .with(file_layer);
///     (def, vec![guard, guard2])
///   }
///   fn init(fpath: impl AsRef<Path>, sub: impl Subscriber + Send + Sync) -> e_utils::Result<()> {
///     e_log::panic::reattach_windows_terminal();
///     e_log::panic::set_panic_hook(fpath)?;
///     init_subscriber(sub, false);
///     Ok(())
///   }
///   let folder = current_dir().unwrap().join("test");
///   folder.auto_create_dir()?;
///   let (subscriber, _guards) = get_subscriber(&folder, "test.log", Level::Info);
///   init(folder.join("bug.test.log"), subscriber)?;
///   info!("info");
///   debug!("debug");
///   error!("error");
///   trace!("trace");
///   warn!("warn");
///   info_box("测试", "INFO");
///   // BUG弹窗
///   assert_eq!("BUG", "???");
///   Ok(())
/// }
///```
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync, is_trace: bool) {
  INIT.call_once(|| {
    if is_trace {
      #[cfg(feature = "tracing-log")]
      tracing_log::LogTracer::init().unwrap();
    }
    if let Err(e) = set_global_default(subscriber) {
      eprintln!("e-log: failed to set global tracing subscriber: {e}");
    }
  });
}
