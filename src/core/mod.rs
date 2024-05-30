cfg_tracing! {
  /// trace
  pub mod _tracing;
  #[allow(unused_imports)]
  pub use _tracing::*;
}
cfg_log! {
  /// Std
  pub mod _log;
  #[allow(unused_imports)]
  pub use _log::*;
}
pub use e_utils::fs::FileShare;
use e_utils::{CResult, Result};
#[cfg(feature = "log")]
pub use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, path::PathBuf, str::FromStr};
#[cfg(feature = "tracing")]
pub use tracing::level_filters::LevelFilter;

/// 针对结果的解析打印
pub trait AutoLevelRes<T, S: AsRef<str>> {
  /// 自动
  fn print(self, add: &str, target: S) -> Self;
  /// 带错误
  fn eprint(self, add: &str, target: S) -> Self;
}

/// 便捷打印
pub trait AutoLevel<A: AsRef<str>> {
  /// 自动判断类型， 如LevelData 和AsRef<str> 默认标签LevelTag::Unknow
  fn print(&self, target: A, level: Level) -> bool;
}

impl<S: AsRef<str>, A: AsRef<str>> AutoLevel<A> for S {
  fn print(&self, target: A, level: Level) -> bool {
    let data = self.as_ref();
    let target = target.as_ref();
    match level {
      Level::Trace => trace!(target:target, data),
      Level::Debug => debug!(target:target, data),
      Level::Info => info!(target:target, data),
      Level::Warn => warn!(target:target, data),
      Level::Error => error!(target:target, data),
      Level::Off => return false,
    };
    true
  }
}
impl<T, S: AsRef<str>> AutoLevelRes<T, S> for CResult<T> {
  fn print(self, add: &str, target: S) -> Self {
    let target = target.as_ref();
    match &self {
      CResult::Ok(_x) => info!(target:target, add),
      CResult::Err(x) => warn!(target:target, "{}: {}", add, x),
    };
    self
  }

  fn eprint(self, add: &str, target: S) -> Self {
    let target = target.as_ref();
    match &self {
      CResult::Ok(_x) => info!(target:target, add),
      CResult::Err(x) => error!(target:target, "{}: {}", add, x,),
    };
    self
  }
}

impl<T, S: AsRef<str>> AutoLevelRes<T, S> for Result<T> {
  fn print(self, add: &str, target: S) -> Self {
    let target = target.as_ref();
    match &self {
      Ok(_x) => info!(target:target, add),
      Err(x) => warn!(target:target, "{}: {}", add, x,),
    };
    self
  }

  fn eprint(self, add: &str, target: S) -> Self {
    let target = target.as_ref();
    match &self {
      Ok(_x) => info!(target:target, add),
      Err(x) => error!(target:target, "{}: {}", add, x,),
    };
    self
  }
}
/// 基础标签
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum LevelTag {
  /// 未知
  Unknow,
  /// 文件
  File,
  /// 日志
  Log,
  /// 系统
  System,
  /// 权限
  Permissions,
  /// 前端
  Ui,
  /// 数据库
  Database,
  /// 自定义
  Other(Cow<'static, str>),
}
impl Default for LevelTag {
  fn default() -> Self {
    LevelTag::Unknow
  }
}
/// 日志等级
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Level {
  /// A level lower than all log levels.
  Off,
  /// Corresponds to the `Error` log level.
  Error,
  /// Corresponds to the `Warn` log level.
  Warn,
  /// Corresponds to the `Info` log level.
  Info,
  /// Corresponds to the `Debug` log level.
  Debug,
  /// Corresponds to the `Trace` log level.
  Trace,
}
impl Default for Level {
  fn default() -> Self {
    Self::Info
  }
}
/// Level数据结构
#[derive(Deserialize, Serialize, Debug)]
pub struct LevelData<T: Serialize, S: Serialize> {
  /// 主体
  pub msg: T,
  /// 标签
  pub target: S,
}
impl<T: Serialize, S: Serialize> LevelData<T, S> {
  /// 序列化
  pub fn to_string(&self) -> String {
    serde_json::to_string(self).unwrap_or_default()
  }
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
      Level::Error => error!(target:target,msg),
      Level::Warn => warn!(target:target,msg),
      Level::Info => info!(target:target,msg),
      Level::Debug => debug!(target:target,msg),
      Level::Trace => trace!(target:target,msg),
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

impl core::fmt::Display for Level {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", self.to_string())
  }
}

#[cfg(feature = "log")]
impl From<&str> for Level {
  fn from(s: &str) -> Self {
    let level: Level = LevelFilter::from_str(s).unwrap_or(LevelFilter::Off).into();
    level
  }
}
#[cfg(feature = "tracing")]
impl From<&str> for Level {
  fn from(s: &str) -> Self {
    let level: Level = LevelFilter::from_str(s).unwrap_or(LevelFilter::OFF).into();
    level
  }
}

impl From<String> for Level {
  fn from(s: String) -> Self {
    s.as_str().into()
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
