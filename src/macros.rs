#[allow(unused)]
macro_rules! cfg_log {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "log")]
            $item
        )*
    }
}
#[allow(unused)]
macro_rules! cfg_tracing {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "tracing")]
            $item
        )*
    }
}
#[allow(unused)]
macro_rules! cfg_log_tauri {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "log-tauri")]
            $item
        )*
    }
}
#[allow(unused)]
macro_rules! cfg_log_actix {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "log-actix")]
            $item
        )*
    }
}
#[allow(unused)]
macro_rules! cfg_tracing_actix {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "tracing-actix")]
            $item
        )*
    }
}
#[allow(unused)]
macro_rules! cfg_tracing_appender {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "tracing-appender")]
            $item
        )*
    }
}

macro_rules! cfg_must {
  ($($item:item)*) => {
      $(
        #[cfg(any(feature="log", feature="tracing"))]
          $item
      )*
  }
}
#[cfg(feature = "log")]
#[doc(hidden)]
#[macro_export]
macro_rules! info {
  (tag: $tag:expr, $msg:expr) => ({
    let data = $crate::LevelData {
      msg: $msg,
      tag: $tag,
    }.to_string();
    $crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Info, "{}",data)
  });
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:$target,$crate::core::_log::__private::StdLevel::Info, "{}",$msg)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_log::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Info, "{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target: $target, $crate::core::_log::__private::StdLevel::Info, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Info, $($arg)+))
}
#[cfg(feature = "log")]
#[doc(hidden)]
#[macro_export]
macro_rules! warn {
  (tag: $tag:expr, $msg:expr) => ({
    let data = $crate::LevelData {
      msg: $msg,
      tag: $tag,
    }.to_string();
    $crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Warn, "{}",data)
  });
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:$target,$crate::core::_log::__private::StdLevel::Warn, "{}",$msg)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_log::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Warn, "{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target: $target, $crate::core::_log::__private::StdLevel::Warn, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Warn, $($arg)+))
}
#[cfg(feature = "log")]
#[doc(hidden)]
#[macro_export]
macro_rules! error {
  (tag: $tag:expr, $msg:expr) => ({
    let data = $crate::LevelData {
      msg: $msg,
      tag: $tag,
    }.to_string();
    $crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Error, "{}",data)
  });
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:$target,$crate::core::_log::__private::StdLevel::Error, "{}",$msg)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_log::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Error, "{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target: $target, $crate::core::_log::__private::StdLevel::Error, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Error, $($arg)+))
}
#[cfg(feature = "log")]
#[doc(hidden)]
#[macro_export]
macro_rules! debug {
  (tag: $tag:expr, $msg:expr) => ({
    let data = $crate::LevelData {
      msg: $msg,
      tag: $tag,
    }.to_string();
    $crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Debug, "{}",data)
  });
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:$target,$crate::core::_log::__private::StdLevel::Debug, "{}",$msg)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_log::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Debug, "{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target: $target, $crate::core::_log::__private::StdLevel::Debug, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Debug, $($arg)+))
}
#[cfg(feature = "log")]
#[doc(hidden)]
#[macro_export]
macro_rules! trace {
  (tag: $tag:expr, $msg:expr) => ({
    let data = $crate::LevelData {
      msg: $msg,
      tag: $tag,
    }.to_string();
    $crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Trace, "{}",data)
  });
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:$target,$crate::core::_log::__private::StdLevel::Trace, "{}",$msg)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_log::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Trace, "{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ({$crate::core::_log::__private::slog!(target: $target, $crate::core::_log::__private::StdLevel::Trace, $($arg)+)});
  ($($arg:tt)+) => ({$crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Trace, $($arg)+)})
}

#[cfg(feature = "tracing")]
#[doc(hidden)]
#[macro_export]
macro_rules! info {
  (tag: $tag:expr, $msg:expr) => ({
    let data = $crate::LevelData {
      msg: $msg,
      tag: $tag,
    }.to_string();
    $crate::core::_tracing::__private::info!("{}",data)
  });
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_tracing::__private::info!(target:$target,"{}",data)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_tracing::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::_tracing::__private::info!("{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_tracing::__private::info!(target:$target,$($arg)+));
  ($($arg:tt)+) => ($crate::core::_tracing::__private::info!($($arg)+))
}
#[cfg(feature = "tracing")]
#[doc(hidden)]
#[macro_export]
macro_rules! warn {
  (tag: $tag:expr, $msg:expr) => ({
    let data = $crate::LevelData {
      msg: $msg,
      tag: $tag,
    }.to_string();
    $crate::core::_tracing::__private::warn!("{}",data)
  });
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_tracing::__private::warn!(target:$target,"{}",data)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_tracing::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::_tracing::__private::warn!("{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_tracing::__private::warn!(target:$target,$($arg)+));
  ($($arg:tt)+) => ($crate::core::_tracing::__private::warn!($($arg)+))
}
#[cfg(feature = "tracing")]
#[doc(hidden)]
#[macro_export]
macro_rules! error {
  (tag: $tag:expr, $msg:expr) => ({
    let data = $crate::LevelData {
      msg: $msg,
      tag: $tag,
    }.to_string();
    $crate::core::_tracing::__private::error!("{}",data)
  });
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_tracing::__private::error!(target:$target,"{}",data)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_tracing::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::_tracing::__private::error!("{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_tracing::__private::error!(target:$target,$($arg)+));
  ($($arg:tt)+) => ($crate::core::_tracing::__private::error!($($arg)+))
}
#[cfg(feature = "tracing")]
#[doc(hidden)]
#[macro_export]
macro_rules! debug {
  (tag: $tag:expr, $msg:expr) => ({
    let data = $crate::LevelData {
      msg: $msg,
      tag: $tag,
    }.to_string();
    $crate::core::_tracing::__private::debug!("{}",data)
  });
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_tracing::__private::debug!(target:$target,"{}",data)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_tracing::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::_tracing::__private::debug!("{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_tracing::__private::debug!(target:$target,$($arg)+));
  ($($arg:tt)+) => ($crate::core::_tracing::__private::debug!($($arg)+))
}
#[cfg(feature = "tracing")]
#[doc(hidden)]
#[macro_export]
macro_rules! trace {
  (tag: $tag:expr, $msg:expr) => ({
    let data = $crate::LevelData {
      msg: $msg,
      tag: $tag,
    }.to_string();
    $crate::core::_tracing::__private::trace!("{}",data)
  });
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_tracing::__private::trace!(target:$target,"{}",data)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_tracing::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::_tracing::__private::trace!("{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_tracing::__private::trace!(target:$target,$($arg)+));
  ($($arg:tt)+) => ($crate::core::_tracing::__private::trace!($($arg)+))
}
