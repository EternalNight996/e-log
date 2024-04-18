// cfg_trace! {
//   /// trace
//   pub mod trace;
//   #[allow(unused_imports)]
//   pub use trace::*;
// }
cfg_std! {
  /// Std
  pub mod _log;
  #[allow(unused_imports)]
  pub use _log::*;
}

use e_utils::{CResult, Result};
#[cfg(feature = "std")]
pub use log::LevelFilter;
// #[cfg(feature = "trace")]
// pub use tracing::level_filters::LevelFilter;

use serde::{Deserialize, Serialize};
use std::{borrow::Cow, str::FromStr};

/// 针对结果的解析打印
pub trait AutoLevelRes<T, S: Serialize> {
  /// 自动
  fn print(self, add: &str, tag: S) -> Self;
  /// 带错误
  fn eprint(self, add: &str, tag: S) -> Self;
}

/// 便捷打印
pub trait AutoLevel<A: Serialize> {
  /// 自动判断类型， 如LevelData 和AsRef<str> 默认标签LevelTag::Unknow
  fn print(&self, tag: A, level: Level) -> bool;
}

impl<S: AsRef<str>, A: Serialize> AutoLevel<A> for S {
  fn print(&self, tag: A, level: Level) -> bool {
    let data = self.as_ref();
    match level {
      Level::Trace => trace!(tag:tag, data),
      Level::Debug => debug!(tag:tag, data),
      Level::Info => info!(tag:tag, data),
      Level::Warn => warn!(tag:tag, data),
      Level::Error => error!(tag:tag, data),
      Level::Off => return false,
    };
    true
  }
}
impl<T, S: Serialize> AutoLevelRes<T, S> for CResult<T> {
  fn print(self, add: &str, tag: S) -> Self {
    match &self {
      CResult::Ok(_x) => info!(tag:tag, add),
      CResult::Err(x) => warn!(tag:tag, "{}: {}", add, x),
    };
    self
  }

  fn eprint(self, add: &str, tag: S) -> Self {
    match &self {
      CResult::Ok(_x) => info!(tag:tag, add),
      CResult::Err(x) => error!(tag:tag, "{}: {}", add, x,),
    };
    self
  }
}

impl<T, S: Serialize> AutoLevelRes<T, S> for Result<T> {
  fn print(self, add: &str, tag: S) -> Self {
    match &self {
      Ok(_x) => info!(tag:tag, add),
      Err(x) => warn!(tag:tag, "{}: {}", add, x,),
    };
    self
  }

  fn eprint(self, add: &str, tag: S) -> Self {
    match &self {
      Ok(_x) => info!(tag:tag, add),
      Err(x) => error!(tag:tag, "{}: {}", add, x,),
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
  pub tag: S,
}
impl<T: Serialize, S: Serialize> LevelData<T, S> {
  /// 序列化
  pub fn to_string(&self) -> String {
    serde_json::to_string(self).unwrap_or_default()
  }
}
impl Level {
  /// 写入日志
  pub fn add<T, S>(&self, msg: T, tag: S) -> bool
  where
    T: Serialize,
    S: Serialize,
  {
    match self {
      Level::Off => return false,
      Level::Error => error!(tag:tag,msg),
      Level::Warn => warn!(tag:tag,msg),
      Level::Info => info!(tag:tag,msg),
      Level::Debug => debug!(tag:tag,msg),
      Level::Trace => trace!(tag:tag,msg),
    }
    true
  }
}

#[cfg(feature = "std")]
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
// #[cfg(feature = "trace")]
// impl From<LevelFilter> for Level {
//   fn from(value: LevelFilter) -> Self {
//     match value {
//       LevelFilter::OFF => Level::Off,
//       LevelFilter::DEBUG => Level::Debug,
//       LevelFilter::INFO => Level::Info,
//       LevelFilter::TRACE => Level::Trace,
//       LevelFilter::ERROR => Level::Error,
//       LevelFilter::WARN => Level::Warn,
//     }
//   }
// }
impl Level {
  /// 日志兼容Std
  #[cfg(feature = "std")]
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
  // /// 日志兼容trace
  // #[cfg(feature = "trace")]
  // pub fn to_level_filter(&self) -> LevelFilter {
  //   match self {
  //     Level::Off => LevelFilter::OFF,
  //     Level::Trace => LevelFilter::TRACE,
  //     Level::Debug => LevelFilter::DEBUG,
  //     Level::Info => LevelFilter::INFO,
  //     Level::Warn => LevelFilter::WARN,
  //     Level::Error => LevelFilter::ERROR,
  //   }
  // }
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

#[cfg(feature = "std")]
impl From<&str> for Level {
  fn from(s: &str) -> Self {
    let level: Level = LevelFilter::from_str(s).unwrap_or(LevelFilter::Off).into();
    level
  }
}
// #[cfg(feature = "trace")]
// impl From<&str> for Level {
//   fn from(s: &str) -> Self {
//     let level: Level = LevelFilter::from_str(s).unwrap_or(LevelFilter::OFF).into();
//     level
//   }
// }

impl From<String> for Level {
  fn from(s: String) -> Self {
    s.as_str().into()
  }
}
