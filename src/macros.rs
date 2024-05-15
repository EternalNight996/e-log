macro_rules! cfg_log {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "_log")]
            $item
        )*
    }
}
macro_rules! cfg_tracing {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "_tracing")]
            $item
        )*
    }
}
macro_rules! cfg_log_tauri {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "log-tauri")]
            $item
        )*
    }
}
macro_rules! cfg_log_actix {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "log-actix")]
            $item
        )*
    }
}
macro_rules! cfg_tracing_actix {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "tracing-actix")]
            $item
        )*
    }
}

macro_rules! cfg_must {
  ($($item:item)*) => {
      $(
        #[cfg(any(feature="_log", feature="_tracing"))]
          $item
      )*
  }
}
#[cfg(feature = "_log")]
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
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_log::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Info, "{}",data)
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
#[cfg(feature = "_log")]
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
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_log::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Warn, "{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_log::__private::format_args!($($arg)+),
      tag: $crate::LevelTag::Unknow,
    }.to_string();
    $crate::core::_log::__private::slog!(target: $target, $crate::core::_log::__private::StdLevel::Warn, "{}", data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target: $target, $crate::core::_log::__private::StdLevel::Warn, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Warn, $($arg)+))
}
#[cfg(feature = "_log")]
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
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_log::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Error, "{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_log::__private::format_args!($($arg)+),
      tag: $crate::LevelTag::Unknow,
    }.to_string();
    $crate::core::_log::__private::slog!(target: $target, $crate::core::_log::__private::StdLevel::Error, "{}", data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target: $target, $crate::core::_log::__private::StdLevel::Error, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Error, $($arg)+))
}
#[cfg(feature = "_log")]
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
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_log::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Debug, "{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_log::__private::format_args!($($arg)+),
      tag: $crate::LevelTag::Unknow,
    }.to_string();
    $crate::core::_log::__private::slog!(target: $target, $crate::core::_log::__private::StdLevel::Debug, "{}", data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target: $target, $crate::core::_log::__private::StdLevel::Debug, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Debug, $($arg)+))
}
#[cfg(feature = "_log")]
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
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_log::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Trace, "{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_log::__private::format_args!($($arg)+),
      tag: $crate::LevelTag::Unknow,
    }.to_string();
    $crate::core::_log::__private::slog!(target: $target, $crate::core::_log::__private::StdLevel::Trace, "{}", data)
  });
  (target: $target:expr, $($arg:tt)+) => ({$crate::core::_log::__private::slog!(target: $target, $crate::core::_log::__private::StdLevel::Trace, $($arg)+)});
  ($($arg:tt)+) => ({$crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Trace, $($arg)+)})
}

#[cfg(feature = "_tracing")]
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
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_tracing::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::_tracing::__private::info!("{}",data)
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
#[cfg(feature = "_tracing")]
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
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_tracing::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::_tracing::__private::warn!("{}",data)
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
#[cfg(feature = "_tracing")]
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
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_tracing::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::_tracing::__private::error!("{}",data)
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
#[cfg(feature = "_tracing")]
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
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_tracing::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::_tracing::__private::debug!("{}",data)
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
#[cfg(feature = "_tracing")]
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
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::_tracing::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::_tracing::__private::trace!("{}",data)
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