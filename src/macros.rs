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
          #[cfg(feature = "log")]
          $item
          #[cfg(all(feature="tracing", not(feature="log")))]
          $item
      )*
  };
}

///
#[cfg(feature = "log")]
#[macro_export]
macro_rules! info2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:$target,$crate::core::_log::__private::StdLevel::Info, "{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target:$target, $crate::core::_log::__private::StdLevel::Info, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Info, $($arg)+))
}
#[cfg(feature = "log")]
#[macro_export]
/// 姝ゅ畯鐢ㄤ簬璁板綍璀﹀憡绾у埆鐨勬棩蹇椼€傚綋 `log` 鐗规€у惎鐢ㄤ笖 `tracing` 鐗规€ф湭鍚敤鏃讹紝浣跨敤 `slog` 杩涜鏃ュ織璁板綍锛涘綋 `tracing` 鐗规€у惎鐢ㄤ笖 `log` 鐗规€ф湭鍚敤鏃讹紝浣跨敤 `tracing` 杩涜鏃ュ織璁板綍銆?
macro_rules! warn2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:$target,$crate::core::_log::__private::StdLevel::Warn, "{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target:$target, $crate::core::_log::__private::StdLevel::Warn, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Warn, $($arg)+))
}
#[cfg(feature = "log")]
#[macro_export]
/// 姝ゅ畯鐢ㄤ簬璁板綍閿欒绾у埆鐨勬棩蹇椼€傚綋 `log` 鐗规€у惎鐢ㄤ笖 `tracing` 鐗规€ф湭鍚敤鏃讹紝浣跨敤 `slog` 杩涜鏃ュ織璁板綍锛涘綋 `tracing` 鐗规€у惎鐢ㄤ笖 `log` 鐗规€ф湭鍚敤鏃讹紝浣跨敤 `tracing` 杩涜鏃ュ織璁板綍銆?
macro_rules! error2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:$target,$crate::core::_log::__private::StdLevel::Error, "{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target:$target, $crate::core::_log::__private::StdLevel::Error, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Error, $($arg)+))
}
///
#[cfg(feature = "log")]
#[macro_export]
macro_rules! debug2 {
  (target: $target:expr, $msg:expr) => ({
    $crate::core::_log::__private::slog!(target:$target,$crate::core::_log::__private::StdLevel::Debug, "{}",$msg)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::core::_log::__private::slog!(target: &$target, $crate::core::_log::__private::StdLevel::Debug, $($arg)+));
  ($($arg:tt)+) => ($crate::core::_log::__private::slog!($crate::core::_log::__private::StdLevel::Debug, $($arg)+))
}
///
#[cfg(feature = "log")]
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
/// 鎵撳嵃鏃ュ織info锛堝畯鐗堟湰锛?
macro_rules! p {
  ($($arg:tt)+) => {{
    #[cfg(debug_assertions)]
    println!($($arg)+);
    #[cfg(any(feature = "log", feature = "tracing"))]
    $crate::preload::info!($($arg)+);
  }};
}

/// 鎵撳嵃鏃ュ織error锛堝畯鐗堟湰锛?
#[macro_export]
macro_rules! ep {
  ($($arg:tt)+) => {{
    #[cfg(debug_assertions)]
    eprintln!($($arg)+);
    #[cfg(any(feature = "log", feature = "tracing"))]
    $crate::preload::error!($($arg)+);
  }};
}

/// 鎵撳嵃鏃ュ織warn锛堝畯鐗堟湰锛?
#[macro_export]
macro_rules! wp {
  ($($arg:tt)+) => {{
    #[cfg(debug_assertions)]
    println!($($arg)+);
    #[cfg(any(feature = "log", feature = "tracing"))]
    $crate::preload::warn!($($arg)+);
  }};
}

/// 鎵撳嵃鏃ュ織debug锛堝畯鐗堟湰锛?
#[macro_export]
macro_rules! dp {
  ($($arg:tt)+) => {{
    #[cfg(debug_assertions)]
    println!($($arg)+);
    #[cfg(any(feature = "log", feature = "tracing"))]
    $crate::preload::debug!($($arg)+);
  }};
}
/// 鎵撳嵃鏃ュ織trace锛堝畯鐗堟湰锛?
#[macro_export]
macro_rules! tp {
  ($($arg:tt)+) => {{
    #[cfg(debug_assertions)]
    println!($($arg)+);
    #[cfg(any(feature = "log", feature = "tracing"))]
    $crate::preload::trace!($($arg)+);
  }};
}
