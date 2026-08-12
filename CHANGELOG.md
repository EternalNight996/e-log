# Changelog

## 0.4.3 (2026-08-12)

### Fixed

- `log` 与 `tracing` 特性可共存（`log` 优先），不再静默丢失 `core`/`preload`/`panic` API。
- 自引用 dev-dependency 关闭 default features，tracing 组合下 examples 可编译。
- examples 空 `main` 兜底门控与实际特性组合一致。
- `max_log_files(0)` 不再触发下溢或误删旧日志（含回归测试）。
- 移除未接线的 `PANIC_SIGNAL` 死代码；panic 弹窗线程 `join` 不再 `unwrap`。
- `init_subscriber` 设置全局 subscriber 失败时打印错误降级，不再 panic。
- `ep!` 改输出到 stderr。
- Windows 下 rolling 测试释放文件句柄后再关闭临时目录。
