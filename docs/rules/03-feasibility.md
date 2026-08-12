# 方案对比

## 决策 1：feature 互斥 vs 共存

- **A 严格互斥**：保留 compile_error。成本 1 行，但 tracing 示例仍因 dev-dep 默认 features 而失败，未达目标。
- **B log 优先共存（采纳）**：`cfg_must!`/宏/`to_level_filter` 门控从"恰好一个"改为"log 优先、tracing-only 兜底"；tracing 的 `LevelFilter`/宏再导出在 log 开启时不导出。成本约 10 处门控，全组合可编译。
- **否决条件**：若未来要求 tracing 宏在共存时也生效，需重新设计宏选择层（成本高，暂不做）。

## 决策 2：vendored tracing-appender 去留

- **A 保持 vendored（采纳）**：appender/ 约 69KB 为上游 `tracing-appender` 复制，含 `FileShare`（Windows 共享读）自定义。上游不支持 FileShare，替换需保留自定义层，成本大于收益。
- **B 依赖上游**：待上游出关键修复或需要新特性时执行，删除 appender/ 并保留 FileShare wrapper。

## 决策 3：中文乱码

- 排查结论：文件均为正常 UTF-8，乱码为 PowerShell 5.1 按 ANSI 读取 UTF-8 的显示假象（见 ADR-003）。无需批量转码。
- 遗留：README 事实错误（default tracing、版本 0.3、LICENSE 链接、大小写链接）已修正。
