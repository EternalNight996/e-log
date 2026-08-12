# 需求清单（MoSCoW）

| ID | 优先级 | 需求 | 验收标准 | 状态 |
|---|---|---|---|---|
| M1 | Must | feature 组合不静默断裂 | `log`+`tracing` 及 tracing 全组合 lib+examples 编译通过 | done |
| M2 | Must | tracing 组合下示例可编译 | dev-dependency 不强制 default features；examples 兜底门控正确 | done |
| M3 | Must | 中文注释/文档可读 | 确认为读取显示假象，文件本身为 UTF-8；README 事实修正 | done |
| S1 | Should | `max_log_files(0)` 不崩溃 | 有回归测试覆盖 | done |
| S2 | Should | panic 路径无死代码、无双 panic | `PANIC_SIGNAL` 移除；join 不 unwrap | done |
| S3 | Should | 初始化失败不 panic | `set_global_default` 失败降级 | done |
| S4 | Should | `ep!` 输出到 stderr | 代码审查 + 示例运行 | done |
| C1 | Could | 轮转时间本地化 | 当前为 UTC，文档声明即可 | debt |
| C2 | Could | 旧日志裁剪排序依据 | `metadata.created()` 在 Linux 不可靠，按文件名日期排序 | debt |
| C3 | Could | `Level` 显示大小写统一 | `Display`(Off) 与 `to_string`(OFF) 不一致 | debt |
| C4 | Could | 删除空壳 API | `LogTarget::Web`/`ser_to_str`/`built` 未实现 | debt |
