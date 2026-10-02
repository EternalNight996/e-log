//! e-log

#![allow(
  clippy::cognitive_complexity,
  clippy::large_enum_variant,
  clippy::module_inception,
  clippy::needless_doctest_main
)]
#![warn(
  missing_debug_implementations,
  missing_docs,
  rust_2018_idioms,
//   unreachable_pub
)]
#![deny(unused_must_use)]
#![doc(test(
  no_crate_inject,
  attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
// #![cfg_attr(loom, allow(dead_code, unreachable_pub))]

/// Macros
#[path = "./macros.rs"]
#[macro_use]
mod macros;

cfg_must! {
  /// core
  pub mod core;
  pub use core::*;
  /// preload
  #[path = "./preload.rs"]
  pub mod preload;
  /// 带 tag 的函数式日志入口（宏写法在 log 路径支持不了 `tag = …` 时的替代）
  #[path = "./tag.rs"]
  pub mod tag;
  /// panic
  pub mod panic;
  /// dialog
  #[cfg(feature="dialog")]
  pub mod dialog;
}
