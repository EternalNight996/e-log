#[cfg(feature = "dialog")]
pub use super::dialog::*;
pub use super::{debug, debug2, error, error2, info, info2, trace, trace2, warn, warn2};
pub use super::{Level, LevelFilter, LogTarget};

// 说明：`tag = …` 支持落在 `*2` 宏上（见 src/macros.rs 的 `[_tag]` 分支）。
// 短名 `info!`/`warn!`/`error!`/`debug!`/`trace!` 在 log 路径按 ADR-001 保持
// `log` crate 的再导出 —— 它们无法同时兼容 `tag=`：
// `log::info!` 只认 format_args，而给短名加本地定义又会与 crate 根的同名宏
// 撞成 E0659/E0433（`#[macro_export]` 的名字不在模块命名空间里，无法用 `pub use` 遮蔽）。
// 需要带 tag 的业务代码请用 `info2!`/`error2!` 等（与 logger.rs 里已有的 `*2` 命名一致）。
