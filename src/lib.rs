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

#[cfg(all(feature = "log", feature = "tracing"))]
#[cfg(panic = "abort")]
compile_error!("Feature 'log' and 'tracing' cannot exists tow to be enabled.");

cfg_must! {
  /// core
  pub mod core;
  pub use core::*;
  /// preload
  #[path = "./preload.rs"]
  pub mod preload;
  /// panic
  pub mod panic;
}
