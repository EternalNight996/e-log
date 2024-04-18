macro_rules! cfg_std {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "std")]
            $item
        )*
    }
}
macro_rules! cfg_trace {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "trace")]
            $item
        )*
    }
}
#[allow(unused)]
macro_rules! cfg_tauri {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "_tauri")]
            $item
        )*
    }
}

#[cfg(feature = "std")]
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
#[cfg(feature = "std")]
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
#[cfg(feature = "std")]
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
#[cfg(feature = "std")]
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
#[cfg(feature = "std")]
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

#[cfg(feature = "trace")]
#[doc(hidden)]
#[macro_export]
macro_rules! info {
  (tag: $tag:expr, $msg:expr) => ({
    let data = $crate::LevelData {
      msg: $msg,
      tag: $tag,
    }.to_string();
    $crate::core::trace::__private::info!("{}",data)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::trace::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::trace::__private::info!("{}",data)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::trace::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::trace::__private::info!("{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::trace::__private::info!(target:$target,$($arg)+));
  ($($arg:tt)+) => ($crate::core::trace::__private::info!($($arg)+))
}
#[cfg(feature = "trace")]
#[doc(hidden)]
#[macro_export]
macro_rules! warn {
  (tag: $tag:expr, $msg:expr) => ({
    let data = $crate::LevelData {
      msg: $msg,
      tag: $tag,
    }.to_string();
    $crate::core::trace::__private::warn!("{}",data)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::trace::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::trace::__private::warn!("{}",data)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::trace::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::trace::__private::warn!("{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::trace::__private::warn!(target:$target,$($arg)+));
  ($($arg:tt)+) => ($crate::core::trace::__private::warn!($($arg)+))
}
#[cfg(feature = "trace")]
#[doc(hidden)]
#[macro_export]
macro_rules! error {
  (tag: $tag:expr, $msg:expr) => ({
    let data = $crate::LevelData {
      msg: $msg,
      tag: $tag,
    }.to_string();
    $crate::core::trace::__private::error!("{}",data)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::trace::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::trace::__private::error!("{}",data)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::trace::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::trace::__private::error!("{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::trace::__private::error!(target:$target,$($arg)+));
  ($($arg:tt)+) => ($crate::core::trace::__private::error!($($arg)+))
}
#[cfg(feature = "trace")]
#[doc(hidden)]
#[macro_export]
macro_rules! debug {
  (tag: $tag:expr, $msg:expr) => ({
    let data = $crate::LevelData {
      msg: $msg,
      tag: $tag,
    }.to_string();
    $crate::core::trace::__private::debug!("{}",data)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::trace::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::trace::__private::debug!("{}",data)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::trace::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::trace::__private::debug!("{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::trace::__private::debug!(target:$target,$($arg)+));
  ($($arg:tt)+) => ($crate::core::trace::__private::debug!($($arg)+))
}
#[cfg(feature = "trace")]
#[doc(hidden)]
#[macro_export]
macro_rules! trace {
  (tag: $tag:expr, $msg:expr) => ({
    let data = $crate::LevelData {
      msg: $msg,
      tag: $tag,
    }.to_string();
    $crate::core::trace::__private::trace!("{}",data)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::trace::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::trace::__private::trace!("{}",data)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::core::trace::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::core::trace::__private::trace!("{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::trace::__private::trace!(target:$target,$($arg)+));
  ($($arg:tt)+) => ($crate::core::trace::__private::trace!($($arg)+))
}