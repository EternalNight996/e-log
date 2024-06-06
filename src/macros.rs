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
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:&$crate::core::ser_to_str($target),$crate::core::_log::__private::StdLevel::Info, "{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target:&$crate::core::ser_to_str($target), $crate::core::_log::__private::StdLevel::Info, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Info, $($arg)+))
}
#[cfg(feature = "log")]
#[doc(hidden)]
#[macro_export]
macro_rules! warn {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:&$crate::core::ser_to_str($target),$crate::core::_log::__private::StdLevel::Warn, "{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target:&$crate::core::ser_to_str($target), $crate::core::_log::__private::StdLevel::Warn, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Warn, $($arg)+))
}
#[cfg(feature = "log")]
#[doc(hidden)]
#[macro_export]
macro_rules! error {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:&$crate::core::ser_to_str($target),$crate::core::_log::__private::StdLevel::Error, "{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target:&$crate::core::ser_to_str($target), $crate::core::_log::__private::StdLevel::Error, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Error, $($arg)+))
}
#[cfg(feature = "log")]
#[doc(hidden)]
#[macro_export]
macro_rules! debug {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:&$crate::core::ser_to_str($target),$crate::core::_log::__private::StdLevel::Debug, "{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target: &$crate::core::ser_to_str($target), $crate::core::_log::__private::StdLevel::Debug, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Debug, $($arg)+))
}
#[cfg(feature = "log")]
#[doc(hidden)]
#[macro_export]
macro_rules! trace {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:&$crate::core::ser_to_str($target),$crate::core::_log::__private::StdLevel::Trace, "{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ({$crate::core::_log::__private::slog!(target:&$crate::core::ser_to_str($target), $crate::core::_log::__private::StdLevel::Trace, $($arg)+)});
  ($($arg:tt)+) => ({$crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Trace, $($arg)+)})
}

#[cfg(feature = "tracing")]
#[doc(hidden)]
#[macro_export]
macro_rules! info {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_tracing::__private::info!(target:&$crate::core::ser_to_str($target),"{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_tracing::__private::info!(target:&$crate::core::ser_to_str($target),$($arg)+));
  ($($arg:tt)+) => ($crate::core::_tracing::__private::info!($($arg)+))
}
#[cfg(feature = "tracing")]
#[doc(hidden)]
#[macro_export]
macro_rules! warn {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_tracing::__private::warn!(target:&$crate::core::ser_to_str($target),"{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_tracing::__private::warn!(target:&$crate::core::ser_to_str($target),$($arg)+));
  ($($arg:tt)+) => ($crate::core::_tracing::__private::warn!($($arg)+))
}
#[cfg(feature = "tracing")]
#[doc(hidden)]
#[macro_export]
macro_rules! error {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_tracing::__private::error!(target:&$crate::core::ser_to_str($target),"{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_tracing::__private::error!(target:&$crate::core::ser_to_str($target),$($arg)+));
  ($($arg:tt)+) => ($crate::core::_tracing::__private::error!($($arg)+))
}
#[cfg(feature = "tracing")]
#[doc(hidden)]
#[macro_export]
macro_rules! debug {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_tracing::__private::debug!(target:&$crate::core::ser_to_str($target),"{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_tracing::__private::debug!(target:&$crate::core::ser_to_str($target),$($arg)+));
  ($($arg:tt)+) => ($crate::core::_tracing::__private::debug!($($arg)+))
}
#[cfg(feature = "tracing")]
#[doc(hidden)]
#[macro_export]
macro_rules! trace {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_tracing::__private::trace!(target:&$crate::core::ser_to_str($target),"{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_tracing::__private::trace!(target:&$crate::core::ser_to_str($target),$($arg)+));
  ($($arg:tt)+) => ($crate::core::_tracing::__private::trace!($($arg)+))
}
