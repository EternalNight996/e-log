#[cfg(all(
  feature = "tracing",
  feature = "panic",
  feature = "dialog",
  feature = "tracing-appender",
  feature = "tracing-subscriber"
))]
// e_utils = {version="0.3", features=["tracing","panic","dialog","tracing-appender","tracing-subscriber"]}
fn main() -> e_utils::AnyResult<()> {
  use e_log::appender::non_blocking::WorkerGuard;
  use e_log::subscriber::layer::SubscriberExt as _;
  use e_log::subscriber::{self, Registry};
  use e_log::{appender, init_subscriber};
  use e_log::{preload::*, Level, __private::subscriber::Subscriber};
  use e_utils::fs::AutoPath as _;
  use std::env::current_dir;
  use std::path::Path;
  use tracing_subscriber::fmt::time::ChronoUtc;
  /// Compose multiple layers into a `tracing`'s subscriber.
  pub fn get_subscriber(
    folder: impl AsRef<Path>,
    fname: impl AsRef<Path>,
    level: Level,
  ) -> (impl Subscriber + Send + Sync, Vec<WorkerGuard>) {
    let roll = appender::rolling::daily(folder, fname, e_log::FileShare::Read);
    let (f, guard) = appender::non_blocking(roll);
    let (f2, guard2) = appender::non_blocking(std::io::stdout());
    let file_layer = subscriber::fmt::layer()
      .with_ansi(false)
      .with_target(true)
      .with_writer(f);
    let timer = ChronoUtc::new("[%F %H:%M:%S]".to_owned());
    let base_layer = subscriber::fmt::layer()
      .with_timer(timer)
      .with_ansi(false)
      .with_target(true)
      .with_writer(f2);
    let def = Registry::default()
      .with(level.to_level_filter())
      .with(base_layer)
      .with(file_layer);
    (def, vec![guard, guard2])
  }
  fn init(
    folder: impl AsRef<Path>,
    fname: &str,
    sub: impl Subscriber + Send + Sync,
  ) -> e_utils::AnyResult<()> {
    e_log::panic::reattach_windows_terminal();
    e_log::panic::set_panic_hook(folder, fname)?;
    init_subscriber(sub, false);
    Ok(())
  }
  let folder = current_dir().unwrap().join("test");
  folder.auto_create_dir()?;
  let (subscriber, _guards) = get_subscriber(&folder, "test.log", Level::Info);
  init(folder, "bug.test.log", subscriber)?;
  info!("info");
  debug!("debug");
  error!("error");
  trace!("trace");
  warn!("warn");
  error2!(target = "test", "测试error2");
  error2!(target: "test", "测试error2");
  info_box("测试", "INFO");
  // BUG弹窗
  assert_eq!("BUG", "???");
  Ok(())
}
#[cfg(not(all(
  feature = "tracing",
  feature = "panic",
  feature = "dialog",
  feature = "tracing-appender",
  feature = "tracing-subscriber"
)))]
fn main() {}
