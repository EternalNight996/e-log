# ADR-001：log/tracing 特性共存（log 优先）

- **背景**：原设计 `log` 与 `tracing` 互斥，`cfg_must!` 仅在"恰好启用一个"时导出 core/preload/panic；拦截 compile_error 挂在 `panic="abort"` 上不生效。自引用 dev-dependency 默认开启 `log`，导致 tracing 组合下 examples 全部编译失败。
- **方案**：A 严格互斥；B log 优先共存。
- **选型**：B。共存是 tracing-log 桥接的生态常态；`to_level_filter`/`From<LevelFilter>`/宏再导出改为 log 开启时使用 log 版本，tracing 版本仅 tracing-only 生效。
- **理由**：所有 feature 组合（含 log+tracing、无特性）均可编译；最小 diff，无新依赖。
- **否决项**：tracing 宏在共存时优先于 log 宏（需宏选择层，未来需要时另立 ADR）。
- **复用搜索**：门控模式沿用现有 `cfg_log!`/`cfg_tracing!` 宏，未引入新机制。
