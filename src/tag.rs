//! 带标签的日志入口（**函数式**，不是宏）。
//!
//! 为什么不用宏：`log` 路径下 `preload` 的短名宏（`info!`/`warn!`/…）按 ADR-001 是
//! `log` crate 的再导出，只认 `format_args!`，不接受 `tag = …`；而在同一 crate 里
//! 既不能用本地定义遮蔽它（E0659 歧义），`#[macro_export]` 的名字又不在模块命名空间里
//! 无法 `pub use` 覆盖（E0433）。所以短名这一层没法在宏上兼容 `tag = …`。
//!
//! 换法：把"带 tag 发一条日志"做成**普通函数**，由 e-log 内部按已启用特性分流：
//! - `tracing` 开 → 发 tracing 事件，tag 作为结构化字段（与 tracing 路径行为一致）；
//! - `tracing` 未开、`log` 开 → 走 `log`，tag 拼进消息文本（log 本身没有字段概念）。
//!
//! 调用方（业务 core 的 logger.rs）只需：
//! ```ignore
//! e_log::emit_tag(e_log::Level::Info, &tag, msg);
//! ```

/// 把 tag 转成 `log` 的目标名 / `tracing` 的事件目标。
pub trait IntoLogTarget {
  /// 取目标字符串
  fn into_target(&self) -> String;
}

impl IntoLogTarget for str {
  fn into_target(&self) -> String {
    self.to_string()
  }
}

impl IntoLogTarget for String {
  fn into_target(&self) -> String {
    self.clone()
  }
}

impl IntoLogTarget for &str {
  fn into_target(&self) -> String {
    (*self).to_string()
  }
}

impl IntoLogTarget for &String {
  fn into_target(&self) -> String {
    (*self).clone()
  }
}

#[cfg(feature = "tracing")]
#[inline]
fn emit_impl(level: crate::Level, tag: &str, msg: &str) {
  // 注:`target:` 只接受常量字符串(tracing 宏限制),运行时 tag 用结构体字段 `tag = %…` 记录,
  // 语义一致(可过滤、可在 subscriber 里按字段取值),且没有常量限制。
  match level {
    crate::Level::Error => tracing::error!(tag = %tag, "{}", msg),
    crate::Level::Warn => tracing::warn!(tag = %tag, "{}", msg),
    crate::Level::Info => tracing::info!(tag = %tag, "{}", msg),
    crate::Level::Debug => tracing::debug!(tag = %tag, "{}", msg),
    crate::Level::Trace => tracing::trace!(tag = %tag, "{}", msg),
    _ => {}
  }
}

#[cfg(all(feature = "log", not(feature = "tracing")))]
#[inline]
fn emit_impl(level: crate::Level, tag: &str, msg: &str) {
  // `log` 没有结构化字段:tag 拼进消息文本
  match level {
    crate::Level::Error => log::error!(target: tag, "{}", msg),
    crate::Level::Warn => log::warn!(target: tag, "{}", msg),
    crate::Level::Info => log::info!(target: tag, "{}", msg),
    crate::Level::Debug => log::debug!(target: tag, "{}", msg),
    crate::Level::Trace => log::trace!(target: tag, "{}", msg),
    _ => {}
  }
}

/// 发一条带 tag 的日志（函数式入口，宏支持不了 `tag = …` 时用它）。
pub fn emit_tag(level: crate::Level, tag: impl IntoLogTarget, msg: &str) {
  let target = tag.into_target();
  emit_impl(level, &target, msg);
}
