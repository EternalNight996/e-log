cfg_tracing! {
  /// trace
  pub mod _tracing;
  #[allow(unused_imports)]
  pub use _tracing::*;
  pub use tracing::level_filters::LevelFilter;
  pub use tracing::{error,debug,warn,info,trace};
}
cfg_log! {
  /// Std
  pub mod _log;
  #[allow(unused_imports)]
  pub use _log::*;
  pub use log::LevelFilter;
  pub use log::{error,debug,warn,info,trace};
}
pub use e_utils::fs::FileShare;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum::*;

/// 日志等级
#[derive(Clone, Copy, Debug, Default, FromRepr, Display)]
#[repr(u8)]
pub enum Level {
  /// A level lower than all log levels.
  Off = 1,
  /// Corresponds to the `Error` log level.
  Error,
  /// Corresponds to the `Warn` log level.
  Warn,
  /// Corresponds to the `Info` log level.
  #[default]
  Info,
  /// Corresponds to the `Debug` log level.
  Debug,
  /// Corresponds to the `Trace` log level.
  Trace,
}

impl Level {
  /// 写入日志
  pub fn add<T, S>(&self, msg: T, target: S) -> bool
  where
    T: AsRef<str>,
    S: AsRef<str>,
  {
    let target = target.as_ref();
    let msg = msg.as_ref();
    match self {
      Level::Off => return false,
      Level::Error => error2!(target:target,msg),
      Level::Warn => warn2!(target:target,msg),
      Level::Info => info2!(target:target,msg),
      Level::Debug => debug2!(target:target,msg),
      Level::Trace => trace2!(target:target,msg),
    }
    true
  }
}

#[cfg(feature = "log")]
impl From<LevelFilter> for Level {
  fn from(value: LevelFilter) -> Self {
    match value {
      LevelFilter::Off => Level::Off,
      LevelFilter::Debug => Level::Debug,
      LevelFilter::Info => Level::Info,
      LevelFilter::Trace => Level::Trace,
      LevelFilter::Error => Level::Error,
      LevelFilter::Warn => Level::Warn,
    }
  }
}
#[cfg(feature = "tracing")]
impl From<LevelFilter> for Level {
  fn from(value: LevelFilter) -> Self {
    match value {
      LevelFilter::OFF => Level::Off,
      LevelFilter::DEBUG => Level::Debug,
      LevelFilter::INFO => Level::Info,
      LevelFilter::TRACE => Level::Trace,
      LevelFilter::ERROR => Level::Error,
      LevelFilter::WARN => Level::Warn,
    }
  }
}
impl Level {
  /// 日志兼容Std
  #[cfg(feature = "log")]
  pub fn to_level_filter(&self) -> LevelFilter {
    match self {
      Level::Off => LevelFilter::Off,
      Level::Trace => LevelFilter::Trace,
      Level::Debug => LevelFilter::Debug,
      Level::Info => LevelFilter::Info,
      Level::Warn => LevelFilter::Warn,
      Level::Error => LevelFilter::Error,
    }
  }
  /// 日志兼容trace
  #[cfg(feature = "tracing")]
  pub fn to_level_filter(&self) -> LevelFilter {
    match self {
      Level::Off => LevelFilter::OFF,
      Level::Trace => LevelFilter::TRACE,
      Level::Debug => LevelFilter::DEBUG,
      Level::Info => LevelFilter::INFO,
      Level::Warn => LevelFilter::WARN,
      Level::Error => LevelFilter::ERROR,
    }
  }
  /// 输出成字符串
  pub fn to_string(&self) -> String {
    self.to_level_filter().to_string()
  }
}

/// An enum representing the available targets of the logger.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum LogTarget {
  /// Print logs to stdout.
  Stdout,
  /// Print logs to stderr.
  Stderr,
  /// Write logs to the given directory.
  ///
  /// The plugin will ensure the directory exists before writing logs.
  Folder(std::path::PathBuf),
  /// Write logs to the OS specific logs directory.
  ///
  /// ### Platform-specific
  ///
  /// |Platform | Value                                         | Example                                        |
  /// | ------- | --------------------------------------------- | ---------------------------------------------- |
  /// | Linux   | `{configDir}/{bundleIdentifier}`              | `/home/alice/.config/com.tauri.dev`            |
  /// | macOS   | `{homeDir}/Library/Logs/{bundleIdentifier}`   | `/Users/Alice/Library/Logs/com.tauri.dev`      |
  /// | Windows | `{configDir}/{bundleIdentifier}`              | `C:\Users\Alice\AppData\Roaming\com.tauri.dev` |
  OutDir(LogOutDirType),
  /// Forward logs to the webview (via the `log://log` event).
  ///
  /// This requires the webview to subscribe to log events, via this plugins `attachConsole` function.
  Web,
}
impl ToString for LogTarget {
  fn to_string(&self) -> String {
    match self {
      LogTarget::Stdout => "Stdout",
      LogTarget::Stderr => "Stderr",
      LogTarget::Folder(_) => "Folder",
      LogTarget::OutDir(x) => match x {
        LogOutDirType::Json => "OutDirJson",
        LogOutDirType::Text => "OutDirText",
      },
      LogTarget::Web => "Web",
    }
    .to_string()
  }
}
impl LogTarget {
  /// 解析
  pub fn from_str(s: &str) -> e_utils::Result<Self> {
    Ok(match s {
      "Stdout" => Self::Stdout,
      "Stderr" => Self::Stderr,
      "Folder" => Self::Folder(PathBuf::new()),
      "OutDirJson" => Self::OutDir(LogOutDirType::Json),
      "OutDirText" => Self::OutDir(LogOutDirType::Text),
      "Web" => Self::Web,
      _ => return Err(e_utils::Error::String(s.to_string())),
    })
  }
  /// 设置Folder路径
  pub fn set_folder<P>(&mut self, path: P) -> &mut Self
  where
    P: AsRef<std::path::Path>,
  {
    if let Self::Folder(_) = self {
      *self = Self::Folder(path.as_ref().to_path_buf());
    }
    self
  }
}
/// 文本格式
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LogOutDirType {
  /// 文本
  Text,
  /// Json
  Json,
}

/// ser to str
pub fn ser_to_str<T: Serialize + ?Sized>(s: &T) -> String {
  serde_json::to_string(s).unwrap_or("Unknow".to_string())
}
