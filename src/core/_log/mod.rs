cfg_log_tauri! {
  /// Tuari
  pub mod tauri;
  pub use tauri::*;
}
cfg_log_actix! {
  /// Tuari
  pub mod actix;
  pub use actix::*;
}
/// Private
pub mod __private;
