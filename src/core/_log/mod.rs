cfg_log_tauri! {
  /// Tuari
  #[path="./tauri.rs"]
  pub mod logger;
  pub use logger::*;
}
cfg_log_actix! {
  /// Tuari
  #[path="./actix.rs"]
  pub mod logger;
  pub use logger::*;
}
/// Private
pub mod __private;
