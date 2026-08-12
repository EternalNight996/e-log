# 债务登记

| 日期 | 阶段 | 债务 | 影响 | 偿还计划 | 状态 |
|---|---|---|---|---|---|
| 2026-08-12 | 8 QA | `cargo test` 在 GNU 工具链缺 dlltool 无法运行 | 需 msvc toolchain 或安装 dlltool | 用 `cargo +stable-x86_64-pc-windows-msvc test`，或 CI 固定 msvc | open |
| 2026-08-12 | 5 架构 | 轮转时间固定 UTC | 国内场景每日文件在北京时间 8 点切换 | 需要时加本地时区配置项 | open |
| 2026-08-12 | 5 架构 | 旧日志裁剪按 `metadata.created()` 排序 | Linux 上创建时间不可靠，可能删错 | 改按文件名日期排序 | open |
| 2026-08-12 | 5 架构 | `time::format_description::parse` 弃用告警（4 处） | 构建噪音 | 升级为 `parse_borrowed` 并调整字段类型 | open |
| 2026-08-12 | 2 需求 | `Level::Display`(Off) 与 `to_string`(OFF) 大小写不一致 | API 输出不一致 | 统一为一种（破坏性变更，随 minor 版本） | open |
| 2026-08-12 | 2 需求 | `LogTarget::Web`/`ser_to_str`/`built` 为空壳 | 误导使用者 | 删除或补实现（破坏性变更，随 minor 版本） | open |
| 2026-08-12 | 7 开发 | worker 写失败静默丢日志（上游 TODO） | 磁盘满时日志静默丢失 | 暴露错误计数或输出到 stderr | open |
