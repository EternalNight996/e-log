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
#[allow(unused)]
macro_rules! cfg_panic {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "panic")]
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

#[cfg(all(feature="log", not(feature="tracing")))]
#[doc(hidden)]
#[macro_export]
macro_rules! info2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:$target,$crate::core::_log::__private::StdLevel::Info, "{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target:$target, $crate::core::_log::__private::StdLevel::Info, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Info, $($arg)+))
}
#[cfg(all(feature="log", not(feature="tracing")))]
#[doc(hidden)]
#[macro_export]
macro_rules! warn2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:$target,$crate::core::_log::__private::StdLevel::Warn, "{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target:$target, $crate::core::_log::__private::StdLevel::Warn, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Warn, $($arg)+))
}
#[cfg(all(feature="log", not(feature="tracing")))]
#[doc(hidden)]
#[macro_export]
macro_rules! error2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:$target,$crate::core::_log::__private::StdLevel::Error, "{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target:$target, $crate::core::_log::__private::StdLevel::Error, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Error, $($arg)+))
}
#[cfg(all(feature="log", not(feature="tracing")))]
#[doc(hidden)]
#[macro_export]
macro_rules! debug2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:$target,$crate::core::_log::__private::StdLevel::Debug, "{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target: &$target, $crate::core::_log::__private::StdLevel::Debug, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Debug, $($arg)+))
}
#[cfg(all(feature="log", not(feature="tracing")))]
#[doc(hidden)]
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
#[doc(hidden)]
macro_rules! info2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_tracing::__private::info!(target=$target,"{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_tracing::__private::info!(target=$target,$($arg)+));
  ($($arg:tt)+) => ($crate::core::_tracing::__private::info!($($arg)+))
}
#[cfg(all(feature="tracing", not(feature="log")))]
#[doc(hidden)]
#[macro_export]
macro_rules! warn2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_tracing::__private::warn!(target=$target,"{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_tracing::__private::warn!(target=$target,$($arg)+));
  ($($arg:tt)+) => ($crate::core::_tracing::__private::warn!($($arg)+))
}
#[cfg(all(feature="tracing", not(feature="log")))]
#[doc(hidden)]
#[macro_export]
macro_rules! error2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_tracing::__private::error!(target=$target,"{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_tracing::__private::error!(target=$target,$($arg)+));
  ($($arg:tt)+) => ($crate::core::_tracing::__private::error!($($arg)+))
}
#[cfg(all(feature="tracing", not(feature="log")))]
#[doc(hidden)]
#[macro_export]
macro_rules! debug2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_tracing::__private::debug!(target=$target,"{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_tracing::__private::debug!(target=$target,$($arg)+));
  ($($arg:tt)+) => ($crate::core::_tracing::__private::debug!($($arg)+))
}
#[cfg(all(feature="tracing", not(feature="log")))]
#[doc(hidden)]
#[macro_export]
macro_rules! trace2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_tracing::__private::trace!(target=$target,"{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_tracing::__private::trace!(target=$target,$($arg)+));
  ($($arg:tt)+) => ($crate::core::_tracing::__private::trace!($($arg)+))
}
