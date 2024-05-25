// Copyright 2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
use crate::core::Level;
use crate::LogTarget;
use e_utils::fs::{FileOptionsExt, FileShare};
use e_utils::parse::MyParseFormat;
use e_utils::system::time::{TimezoneStrategy, LOG_DATE_FORMAT, LOG_FILE_DATE_FORMAT};
use e_utils::Result;

pub use fern;
use fern::FormatCallback;
use log::{logger, RecordBuilder};
use log::{LevelFilter, Record};
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io;
use std::{
  fmt::Arguments,
  fs::{self, File},
  iter::FromIterator,
  path::{Path, PathBuf},
};
use tauri::{
  plugin::{self, TauriPlugin},
  Manager, Runtime,
};

const DEFAULT_MAX_FILE_SIZE: u128 = 40000;
const DEFAULT_ROTATION_STRATEGY: RotationStrategy = RotationStrategy::KeepOne;
const DEFAULT_TIMEZONE_STRATEGY: TimezoneStrategy = TimezoneStrategy::UseUtc;
const DEFAULT_LOG_TARGETS: [LogTarget; 2] = [
  LogTarget::Stdout,
  LogTarget::OutDir(crate::LogOutDirType::Text),
];
#[cfg(target_os = "windows")]
const LINEFEED: &'static str = "\r";
#[cfg(not(target_os = "windows"))]
const LINEFEED: &'static str = "";

/// # Example
/// ```rust
/// fn init_layer_log(&self) -> Result<TauriBuilder> {
///   let folder = self.get_log_folder()?;
///   let fname = self.get_log_fname()?;
///   folder.auto_create_dir()?;
///   let log = init_layer_log(
///     Some(fname),
///     folder,
///     self.log.level,
///     self.log.format.clone(),
///     self.log.output_list.clone(),
///     vec!["tao::platform_impl::platform::event_loop::runner".to_string()],
///   )
///   .map_err(|e| e.to_string())?;
///   Ok(log)
/// }
/// ```
pub fn init_layer_log(
  fname: Option<String>,
  level: Level,
  _format: String,
  output_list: impl IntoIterator<Item = LogTarget>,
  filter_target: Vec<String>,
) -> Result<Builder> {
  let mut log = Builder::default()
    .rotation_strategy(RotationStrategy::KeepAll)
    .targets(output_list)
    .format(move |callback, message, record| {
      let target = record.target();
      if filter_target.iter().find(|x| &***x == target).is_none() {
        let time = "{time}".parse_format().unwrap();
        let date = "{date}".parse_format().unwrap();
        callback.finish(format_args!(
          "[{}][{}][{}][{}] {}{LINEFEED}",
          date,
          time,
          target,
          record.level(),
          message
        ))
      }
    })
    .level(level.to_level_filter());

  if let Some(x) = fname {
    log = log.log_name(x);
  }
  Ok(log)
}

/// An enum representing the available verbosity levels of the logger.
///
/// It is very similar to the [`log::Level`], but serializes to unsigned ints instead of strings.
#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u16)]
pub enum LogLevel {
  /// The "trace" level.
  ///
  /// Designates very low priority, often extremely verbose, information.
  Trace = 1,
  /// The "debug" level.
  ///
  /// Designates lower priority information.
  Debug,
  /// The "info" level.
  ///
  /// Designates useful information.
  Info,
  /// The "warn" level.
  ///
  /// Designates hazardous situations.
  Warn,
  /// The "error" level.
  ///
  /// Designates very serious errors.
  Error,
}

impl From<LogLevel> for log::Level {
  fn from(log_level: LogLevel) -> Self {
    match log_level {
      LogLevel::Trace => log::Level::Trace,
      LogLevel::Debug => log::Level::Debug,
      LogLevel::Info => log::Level::Info,
      LogLevel::Warn => log::Level::Warn,
      LogLevel::Error => log::Level::Error,
    }
  }
}

impl From<log::Level> for LogLevel {
  fn from(log_level: log::Level) -> Self {
    match log_level {
      log::Level::Trace => LogLevel::Trace,
      log::Level::Debug => LogLevel::Debug,
      log::Level::Info => LogLevel::Info,
      log::Level::Warn => LogLevel::Warn,
      log::Level::Error => LogLevel::Error,
    }
  }
}
///
#[derive(Debug)]
pub enum RotationStrategy {
  ///
  KeepAll,
  ///
  KeepOne,
}

#[derive(Debug, Serialize, Clone)]
struct RecordPayload {
  message: String,
  level: LogLevel,
}

#[tauri::command]
fn log(
  level: LogLevel,
  message: String,
  location: Option<&str>,
  file: Option<&str>,
  line: Option<u32>,
  key_values: Option<HashMap<String, String>>,
) {
  let location = location.unwrap_or("webview");
  let mut builder = RecordBuilder::new();
  builder
    .level(level.into())
    .target(location)
    .file(file)
    .line(line);

  let key_values = key_values.unwrap_or_default();
  let mut kv = HashMap::new();
  for (k, v) in key_values.iter() {
    kv.insert(k.as_str(), v.as_str());
  }
  builder.key_values(&kv);

  logger().log(&builder.args(format_args!("{message}")).build());
}
///
pub type TauriBuilder = Builder;
///
#[derive(Debug)]
pub struct Builder {
  ///
  dispatch: fern::Dispatch,
  ///
  rotation_strategy: RotationStrategy,
  ///
  timezone_strategy: TimezoneStrategy,
  ///
  max_file_size: u128,
  ///
  targets: Vec<LogTarget>,
  ///
  log_name: Option<String>,
}

impl Default for Builder {
  fn default() -> Self {
    let dispatch = fern::Dispatch::new().format(move |out, message, record| {
      out.finish(format_args!(
        "{}[{}][{}] {}",
        DEFAULT_TIMEZONE_STRATEGY.get_now().format(LOG_DATE_FORMAT),
        record.level(),
        record.target(),
        message
      ))
    });
    Self {
      dispatch,
      rotation_strategy: DEFAULT_ROTATION_STRATEGY,
      timezone_strategy: DEFAULT_TIMEZONE_STRATEGY,
      max_file_size: DEFAULT_MAX_FILE_SIZE,
      targets: DEFAULT_LOG_TARGETS.into(),
      log_name: None,
    }
  }
}

impl Builder {
  ///
  pub fn new() -> Self {
    Default::default()
  }
  ///
  pub fn rotation_strategy(mut self, rotation_strategy: RotationStrategy) -> Self {
    self.rotation_strategy = rotation_strategy;
    self
  }
  ///
  pub fn timezone_strategy(mut self, timezone_strategy: TimezoneStrategy) -> Self {
    self.timezone_strategy = timezone_strategy.clone();

    self.dispatch = fern::Dispatch::new().format(move |out, message, record| {
      out.finish(format_args!(
        "{}[{}][{}] {}",
        timezone_strategy.get_now().format(LOG_DATE_FORMAT),
        record.level(),
        record.target(),
        message
      ))
    });
    self
  }
  ///
  pub fn max_file_size(mut self, max_file_size: u128) -> Self {
    self.max_file_size = max_file_size;
    self
  }
  ///
  pub fn format<F>(mut self, formatter: F) -> Self
  where
    F: Fn(FormatCallback<'_>, &Arguments<'_>, &Record<'_>) + Sync + Send + 'static,
  {
    self.dispatch = self.dispatch.format(formatter);
    self
  }
  ///
  pub fn level(mut self, level_filter: impl Into<LevelFilter>) -> Self {
    self.dispatch = self.dispatch.level(level_filter.into());
    self
  }
  ///
  pub fn level_for(mut self, module: impl Into<Cow<'static, str>>, level: LevelFilter) -> Self {
    self.dispatch = self.dispatch.level_for(module, level);
    self
  }
  ///
  pub fn filter<F>(mut self, filter: F) -> Self
  where
    F: Fn(&log::Metadata<'_>) -> bool + Send + Sync + 'static,
  {
    self.dispatch = self.dispatch.filter(filter);
    self
  }
  ///
  pub fn target(mut self, target: LogTarget) -> Self {
    self.targets.push(target);
    self
  }
  ///
  pub fn targets(mut self, targets: impl IntoIterator<Item = LogTarget>) -> Self {
    self.targets = Vec::from_iter(targets);
    self
  }

  /// Writes logs to the given file. Default: <app_name>.log)
  ///
  /// Note: This does not modify the directory logs go into. For that refer to `LogTarget::Folder`.
  ///
  /// # Examples
  /// ```rust
  /// fn main() {
  ///   use e_log::init_layer_log;
  ///   use e_log::Level;
  ///   use e_log::LogTarget;
  ///   use e_log::TauriBuilder;
  ///   use e_utils::parse::AutoPath as _;
  ///   use e_utils::parse::MyParseFormat;
  ///   #[inline]
  ///   fn init_layer(
  ///     folder: &str,
  ///     fname: Option<String>,
  ///     level: Level,
  ///     out_str: Vec<&str>,
  ///   ) -> e_utils::Result<TauriBuilder> {
  ///     folder.auto_create_dir()?;
  ///     let out_list: e_utils::Result<Vec<LogTarget>> = out_str
  ///       .iter()
  ///       .map(|x| {
  ///         LogTarget::from_str(*x).and_then(|x2| {
  ///           Ok(if let LogTarget::Folder(_) = &x2 {
  ///             LogTarget::Folder(std::path::Path::new(folder).to_path_buf())
  ///           } else {
  ///             x2
  ///           })
  ///         })
  ///       })
  ///       .collect();
  ///     let log = init_layer_log(
  ///       fname,
  ///       level,
  ///       "".to_string(),
  ///       out_list?,
  ///       vec!["tao::platform_impl::platform::event_loop::runner".to_string()],
  ///     )?;
  ///     Ok(log)
  ///   }
  ///   let logger = init_layer(
  ///     &"{cwd}\\logs".parse_path(),
  ///     Some("test.log".into()),
  ///     Level::Info,
  ///     vec!["Stdout", "Web", "OutDirJson", "Folder"],
  ///   )
  ///   .unwrap()
  ///   .build();
  ///   tauri::Builder::default()
  ///     .plugin(logger)
  ///     .run(tauri::generate_context!())
  ///     .expect("error while running tauri application");
  /// }
  /// ```
  pub fn log_name<S: Into<String>>(mut self, log_name: S) -> Self {
    self.log_name = Some(log_name.into());
    self
  }
  ///
  #[cfg(feature = "tauri-colored")]
  pub fn with_colors(self, colors: fern::colors::ColoredLevelConfig) -> Self {
    let format =
      time::format_description::parse("[[[year]-[month]-[day]][[[hour]:[minute]:[second]]")
        .unwrap();

    let timezone_strategy = self.timezone_strategy.clone();
    self.format(move |out, message, record| {
      out.finish(format_args!(
        "{}[{}][{}] {}",
        timezone_strategy.get_now().format(&format).unwrap(),
        colors.color(record.level()),
        record.target(),
        message
      ))
    })
  }
  ///
  pub fn build<R: Runtime>(mut self) -> TauriPlugin<R> {
    plugin::Builder::new("log")
      .invoke_handler(tauri::generate_handler![log])
      .setup(move |app_handle| {
        let log_name = self
          .log_name
          .as_deref()
          .unwrap_or_else(|| &app_handle.package_info().name);

        // setup targets
        for target in &self.targets {
          self.dispatch = self.dispatch.chain(match target {
            LogTarget::Stdout => fern::Output::from(std::io::stdout()),
            LogTarget::Stderr => fern::Output::from(std::io::stderr()),
            LogTarget::Folder(path) => {
              if !path.exists() {
                fs::create_dir_all(path).unwrap();
              }

              log_file(get_log_file_path(
                &path,
                log_name,
                &self.rotation_strategy,
                &self.timezone_strategy,
                self.max_file_size,
              )?)?
              .into()
            }
            LogTarget::OutDir(otype) => {
              let path = app_handle.path_resolver().app_log_dir().unwrap();
              if !path.exists() {
                fs::create_dir_all(&path).unwrap();
              }
              match otype {
                crate::LogOutDirType::Text => log_file(get_log_file_path(
                  &path,
                  log_name,
                  &self.rotation_strategy,
                  &self.timezone_strategy,
                  self.max_file_size,
                )?)?
                .into(),
                crate::LogOutDirType::Json => log_file(get_log_file_path(
                  &path,
                  log_name,
                  &self.rotation_strategy,
                  &self.timezone_strategy,
                  self.max_file_size,
                )?)?
                .into(),
              }
            }
            LogTarget::Web => {
              let app_handle = app_handle.clone();

              fern::Output::call(move |record| {
                let payload = RecordPayload {
                  message: record.args().to_string(),
                  level: record.level().into(),
                };
                let app_handle = app_handle.clone();
                tauri::async_runtime::spawn(async move {
                  app_handle.emit_all("log://log", payload).unwrap();
                });
              })
            }
          });
        }

        self.dispatch.apply()?;

        Ok(())
      })
      .build()
  }
}
///
fn get_log_file_path(
  dir: &impl AsRef<Path>,
  log_name: &str,
  rotation_strategy: &RotationStrategy,
  timezone_strategy: &TimezoneStrategy,
  max_file_size: u128,
) -> plugin::Result<PathBuf> {
  let path = dir.as_ref().join(log_name);

  if path.exists() {
    let log_size = File::open(&path)?.metadata()?.len() as u128;
    if log_size > max_file_size {
      match rotation_strategy {
        RotationStrategy::KeepAll => {
          let to = dir.as_ref().join(format!(
            "{}_{}",
            log_name,
            timezone_strategy.get_now().format(LOG_FILE_DATE_FORMAT),
          ));
          if to.is_file() {
            // designated rotated log file name already exists
            // highly unlikely but defensively handle anyway by adding .bak to filename
            let mut to_bak = to.clone();
            to_bak.set_file_name(format!(
              "{}.bak",
              to_bak.file_name().unwrap().to_string_lossy()
            ));
            fs::rename(&to, to_bak)?;
          }
          fs::rename(&path, to)?;
        }
        RotationStrategy::KeepOne => {
          fs::remove_file(&path)?;
        }
      }
    }
  }

  Ok(path)
}

#[inline]
fn log_file<P: AsRef<Path>>(path: P) -> io::Result<File> {
  OpenOptions::new()
    .write(true)
    .create(true)
    .append(true)
    .lock_share(FileShare::Read)
    .open(path)
}
