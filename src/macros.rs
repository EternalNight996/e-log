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
macro_rules! cfg_tracing_appender {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "tracing-appender")]
            $item
        )*
    }
}
#[allow(unused)]
macro_rules! cfg_panic {
    ($($item:item)*) => {
        $(
            #[cfg(all(feature = "panic", any(feature = "log", feature = "tracing")))]
            $item
        )*
    }
}

macro_rules! cfg_must {
  ($($item:item)*) => {
      $(
          #[cfg(all(feature="log", not(feature="tracing")))]
          $item
          #[cfg(all(feature="tracing", not(feature="log")))]
          $item
      )*
  };
}

///
#[cfg(all(feature="log", not(feature="tracing")))]
#[macro_export]
macro_rules! info2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:$target,$crate::core::_log::__private::StdLevel::Info, "{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target:$target, $crate::core::_log::__private::StdLevel::Info, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Info, $($arg)+))
}
#[cfg(all(feature="log", not(feature="tracing")))]
#[macro_export]
/// 此宏用于记录警告级别的日志。当 `log` 特性启用且 `tracing` 特性未启用时，使用 `slog` 进行日志记录；当 `tracing` 特性启用且 `log` 特性未启用时，使用 `tracing` 进行日志记录。
macro_rules! warn2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:$target,$crate::core::_log::__private::StdLevel::Warn, "{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target:$target, $crate::core::_log::__private::StdLevel::Warn, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Warn, $($arg)+))
}
#[cfg(all(feature="log", not(feature="tracing")))]
#[macro_export]
/// 此宏用于记录错误级别的日志。当 `log` 特性启用且 `tracing` 特性未启用时，使用 `slog` 进行日志记录；当 `tracing` 特性启用且 `log` 特性未启用时，使用 `tracing` 进行日志记录。
macro_rules! error2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:$target,$crate::core::_log::__private::StdLevel::Error, "{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target:$target, $crate::core::_log::__private::StdLevel::Error, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Error, $($arg)+))
}
///
#[cfg(all(feature="log", not(feature="tracing")))]
#[macro_export]
macro_rules! debug2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:$target,$crate::core::_log::__private::StdLevel::Debug, "{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target: &$target, $crate::core::_log::__private::StdLevel::Debug, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Debug, $($arg)+))
}
///
#[cfg(all(feature="log", not(feature="tracing")))]
#[macro_export]
macro_rules! trace2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:$target,$crate::core::_log::__private::StdLevel::Trace, "{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ({$crate::core::_log::__private::slog!(target:$target, $crate::core::_log::__private::StdLevel::Trace, $($arg)+)});
  ($($arg:tt)+) => ({$crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Trace, $($arg)+)})
}

#[cfg(all(feature="tracing", not(feature="log")))]
#[macro_export]
///
macro_rules! info2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_tracing::__private::info!(target=$target,"{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_tracing::__private::info!(target=$target,$($arg)+));
  ($($arg:tt)+) => ($crate::core::_tracing::__private::info!($($arg)+))
}
#[cfg(all(feature="tracing", not(feature="log")))]
#[macro_export]
///
macro_rules! warn2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_tracing::__private::warn!(target=$target,"{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_tracing::__private::warn!(target=$target,$($arg)+));
  ($($arg:tt)+) => ($crate::core::_tracing::__private::warn!($($arg)+))
}
#[cfg(all(feature="tracing", not(feature="log")))]
#[macro_export]
///
macro_rules! error2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_tracing::__private::error!(target=$target,"{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_tracing::__private::error!(target=$target,$($arg)+));
  ($($arg:tt)+) => ($crate::core::_tracing::__private::error!($($arg)+))
}
#[cfg(all(feature="tracing", not(feature="log")))]
#[macro_export]
///
macro_rules! debug2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_tracing::__private::debug!(target=$target,"{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_tracing::__private::debug!(target=$target,$($arg)+));
  ($($arg:tt)+) => ($crate::core::_tracing::__private::debug!($($arg)+))
}
#[cfg(all(feature="tracing", not(feature="log")))]
#[macro_export]
///
macro_rules! trace2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_tracing::__private::trace!(target=$target,"{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_tracing::__private::trace!(target=$target,$($arg)+));
  ($($arg:tt)+) => ($crate::core::_tracing::__private::trace!($($arg)+))
}


#[macro_export]
/// 打印日志info（宏版本）
macro_rules! p {
  ($v:expr) => {{
    #[cfg(debug_assertions)]
    println!("{}", $v);
    #[cfg(any(feature = "log", feature = "tracing"))]
    $crate::preload::info!("{}", $v);
  }};
}

/// 打印日志error（宏版本）
#[macro_export]
macro_rules! ep {
  ($v:expr) => {{
    #[cfg(debug_assertions)]
    println!("{}", $v);
    #[cfg(any(feature = "log", feature = "tracing"))]
    $crate::preload::error!("{}", $v);
  }};
}

/// 打印日志warn（宏版本）
#[macro_export]
macro_rules! wp {
  ($v:expr) => {{
    #[cfg(debug_assertions)]
    println!("{}", $v);
    #[cfg(any(feature = "log", feature = "tracing"))]
    $crate::preload::warn!("{}", $v);
  }};
}

/// 打印日志debug（宏版本）
#[macro_export]
macro_rules! dp {
  ($v:expr) => {{
    #[cfg(debug_assertions)]
    println!("{}", $v);
    #[cfg(any(feature = "log", feature = "tracing"))]
    $crate::preload::debug!("{}", $v);
  }};
}
/// 打印日志trace（宏版本）
#[macro_export]
macro_rules! tp {
  ($v:expr) => {{
    #[cfg(debug_assertions)]
    println!("{}", $v);
    #[cfg(any(feature = "log", feature = "tracing"))]
    $crate::preload::trace!("{}", $v);
  }};
}
