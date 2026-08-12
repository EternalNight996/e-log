# ADR-002：保留 vendored tracing-appender

- **背景**：`src/core/_tracing/appender/`（约 69KB）复制自上游 `tracing-appender`，文档仍引用 `tracing_appender` 路径。
- **方案**：A 保留并修 bug；B 依赖上游并删 69KB。
- **选型**：A。上游不支持 `FileShare`（Windows 共享读），替换需自定义 writer 层；且本轮已修复 `max_log_files(0)` 下溢等本地问题。
- **理由**：收益低于成本；本仓库以 Windows 场景为主。
- **否决项/触发条件**：上游修复了本地依赖的 bug，或需要上游新特性（如 json 轮转）时，重新评估 B。
