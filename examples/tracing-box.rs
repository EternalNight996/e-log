#[cfg(all(
  feature = "tracing",
  feature = "panic",
  feature = "dialog",
  feature = "tracing-subscriber"
))]
// e_utils = {version="0.3", features=["tracing","panic","dialog","tracing-subscriber"]}
fn main() -> e_utils::AnyResult<()> {
  use e_log::init_subscriber;
  use e_log::subscriber::layer::SubscriberExt as _;
  use e_log::subscriber::{self, Registry};
  use e_log::{preload::*, Level, __private::subscriber::Subscriber};
  use e_utils::fs::AutoPath as _;
  use std::env::current_dir;
  use std::path::Path;
  use tracing_subscriber::fmt::time::ChronoUtc;
  /// Compose multiple layers into a `tracing`'s subscriber.
  pub fn get_subscriber(level: Level) -> impl Subscriber + Send + Sync {
    let timer = ChronoUtc::new("[%F %H:%M:%S]".to_owned());
    let base_layer = subscriber::fmt::layer()
      .with_timer(timer)
      .with_ansi(false)
      .with_target(true);
    let def = Registry::default()
      .with(level.to_level_filter())
      .with(base_layer);
    def
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
  let subscriber = get_subscriber(Level::Debug);
  init(folder, "bug.test.log", subscriber)?;
  info_box("测试", "INFO");
  error_box("测试", "ERROR");
  warn_box("测试", "WARN");
  ok_zh_box("测试", "确认");
  ok_box("测试", "OK");
  assert!(yesno_zh_box("测试", "同意或取消"));
  // BUG弹窗
  assert_eq!("BUG", "???");
  Ok(())
}
#[cfg(not(feature = "tracing"))]
fn main() {}
