macro_rules! cfg_std {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "std")]
            #[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
            $item
        )*
    }
}
macro_rules! cfg_trace {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "trace")]
            #[cfg_attr(doc_cfg, doc(cfg(feature = "trace")))]
            $item
        )*
    }
}
macro_rules! cfg_tauri {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "_tauri")]
            #[cfg_attr(doc_cfg, doc(cfg(feature = "_tauri")))]
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
    $crate::__private::slog!($crate::__private::StdLevel::Info, "{}",data)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::__private::slog!($crate::__private::StdLevel::Info, "{}",data)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::__private::slog!($crate::__private::StdLevel::Info, "{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::__private::slog!(target: $target, $crate::__private::StdLevel::Info, $($arg)+));
  ($($arg:tt)+) => ($crate::__private::slog!($crate::__private::StdLevel::Info, $($arg)+))
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
    $crate::__private::slog!($crate::__private::StdLevel::Warn, "{}",data)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::__private::slog!($crate::__private::StdLevel::Warn, "{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::__private::format_args!($($arg)+),
      tag: $crate::LevelTag::Unknow,
    }.to_string();
    $crate::__private::slog!(target: $target, $crate::__private::StdLevel::Warn, "{}", data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::__private::slog!(target: $target, $crate::__private::StdLevel::Warn, $($arg)+));
  ($($arg:tt)+) => ($crate::__private::slog!($crate::__private::StdLevel::Warn, $($arg)+))
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
    $crate::__private::slog!($crate::__private::StdLevel::Error, "{}",data)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::__private::slog!($crate::__private::StdLevel::Error, "{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::__private::format_args!($($arg)+),
      tag: $crate::LevelTag::Unknow,
    }.to_string();
    $crate::__private::slog!(target: $target, $crate::__private::StdLevel::Error, "{}", data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::__private::slog!(target: $target, $crate::__private::StdLevel::Error, $($arg)+));
  ($($arg:tt)+) => ($crate::__private::slog!($crate::__private::StdLevel::Error, $($arg)+))
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
    $crate::__private::slog!($crate::__private::StdLevel::Debug, "{}",data)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::__private::slog!($crate::__private::StdLevel::Debug, "{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::__private::format_args!($($arg)+),
      tag: $crate::LevelTag::Unknow,
    }.to_string();
    $crate::__private::slog!(target: $target, $crate::__private::StdLevel::Debug, "{}", data)
  });
  (target: $target:expr, $($arg:tt)+) => ($crate::__private::slog!(target: $target, $crate::__private::StdLevel::Debug, $($arg)+));
  ($($arg:tt)+) => ($crate::__private::slog!($crate::__private::StdLevel::Debug, $($arg)+))
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
    $crate::__private::slog!($crate::__private::StdLevel::Trace, "{}",data)
  });
  (tag: $tag:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::__private::format_args!($($arg)+),
      tag: $tag,
    }.to_string();
    $crate::__private::slog!($crate::__private::StdLevel::Trace, "{}",data)
  });
  (target: $target:expr, $($arg:tt)+) => ({
    let data = $crate::LevelData {
      msg: $crate::__private::format_args!($($arg)+),
      tag: $crate::LevelTag::Unknow,
    }.to_string();
    $crate::__private::slog!(target: $target, $crate::__private::StdLevel::Trace, "{}", data)
  });
  (target: $target:expr, $($arg:tt)+) => ({$crate::__private::slog!(target: $target, $crate::__private::StdLevel::Trace, $($arg)+)});
  ($($arg:tt)+) => ({$crate::__private::slog!($crate::__private::StdLevel::Trace, $($arg)+)})
}
