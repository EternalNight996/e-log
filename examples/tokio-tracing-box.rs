#[cfg(all(
  feature = "tracing",
  feature = "panic",
  feature = "dialog",
  feature = "tracing-subscriber",
))]
// e_utils = {version="0.3", features=["tracing","panic","dialog","tracing-subscriber"]}
#[tokio::main]
async fn main() -> e_utils::Result<()> {
  use e_log::init_subscriber;
  use e_log::subscriber::layer::SubscriberExt as _;
  use e_log::subscriber::{self, Registry};
  use e_log::{preload::*, Level, __private::subscriber::Subscriber};
  use e_utils::parse::AutoPath as _;
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
  fn init(fpath: impl AsRef<Path>, sub: impl Subscriber + Send + Sync) -> e_utils::Result<()> {
    e_log::panic::reattach_windows_terminal();
    e_log::panic::set_panic_hook(fpath)?;
    init_subscriber(sub, false);
    Ok(())
  }
  let folder = current_dir().unwrap().join("test");
  folder.auto_create_dir()?;
  let subscriber = get_subscriber(Level::Debug);
  init(folder.join("bug.test.log"), subscriber)?;
  a_info_box("测试", "INFO").await;
  a_error_box("测试", "ERROR").await;
  a_warn_box("测试", "WARN").await;
  a_ok_zh_box("测试", "确认").await;
  a_ok_box("测试", "OK").await;
  assert!(a_yesno_zh_box("测试", "同意或取消").await);
  // BUG弹窗
  assert_eq!("BUG", "???");
  Ok(())
}
#[cfg(not(feature = "tracing"))]
fn main() {}
