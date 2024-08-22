use crate::preload::*;
use e_utils::dialog;

/// error
pub fn error_box(title: &str, msg: impl Into<String>) {
  let msg = msg.into();
  error!(msg);
  dialog::sync::error(title, msg);
}
/// error
pub async fn a_error_box(title: &str, msg: impl Into<String>) {
  let msg = msg.into();
  error!(msg);
  dialog::a_sync::error(title, msg).await
}

/// info
pub fn info_box(title: &str, msg: impl Into<String>) {
  let msg = msg.into();
  info!(msg);
  dialog::sync::info(title, msg);
}
/// info
pub async fn a_info_box(title: &str, msg: impl Into<String>) {
  let msg = msg.into();
  info!(msg);
  dialog::a_sync::info(title, msg).await
}

/// warn
pub fn warn_box(title: &str, msg: impl Into<String>) {
  let msg = msg.into();
  warn!(msg);
  dialog::sync::warn(title, msg);
}
/// warn
pub async fn a_warn_box(title: &str, msg: impl Into<String>) {
  let msg = msg.into();
  warn!(msg);
  dialog::a_sync::warn(title, msg).await
}

/// OK 警告确认 [INFO]
pub fn ok_zh_box(title: &str, msg: impl Into<String>) {
  let msg = msg.into();
  info!(msg);
  dialog::sync::ok_zh(title, msg)
}
/// OK 警告确认 [INFO]
pub async fn a_ok_zh_box(title: &str, msg: impl Into<String>) {
  let msg = msg.into();
  info!(msg);
  dialog::a_sync::ok_zh(title, msg).await
}

/// OK [INFO]
pub fn ok_box(title: &str, msg: impl Into<String>) {
  let msg = msg.into();
  info!(msg);
  dialog::sync::ok(title, msg)
}
/// OK [INFO]
pub async fn a_ok_box(title: &str, msg: impl Into<String>) {
  let msg = msg.into();
  info!(msg);
  dialog::a_sync::ok(title, msg).await
}

/// 同意取消 [INFO]
pub fn yesno_zh_box(title: &str, msg: impl Into<String>) -> bool {
  let msg = msg.into();
  info!(msg);
  dialog::sync::yesno_zh(title, msg)
}
/// 同意取消 [INFO]
pub async fn a_yesno_zh_box(title: &str, msg: impl Into<String>) -> bool {
  let msg = msg.into();
  info!(msg);
  dialog::a_sync::yesno_zh(title, msg).await
}
/// YES NO [INFO]
pub fn yesno_box(title: &str, msg: impl Into<String>) -> bool {
  let msg = msg.into();
  info!(msg);
  dialog::sync::yesno(title, msg)
}
/// YES NO [INFO]
pub async fn a_yesno_box(title: &str, msg: impl Into<String>) -> bool {
  let msg = msg.into();
  info!(msg);
  dialog::a_sync::yesno(title, msg).await
}
