cfg_log_tauri! {
  /// Tuari
  #[path="./tauri.rs"]
  pub mod logger;
}
cfg_log_actix! {
  /// Tuari
  #[path="./actix.rs"]
  pub mod logger;
}
/// Private
pub mod __private;
