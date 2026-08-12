# 产品卡：e-log

- **一句话定位**：Rust 桌面应用的一站式日志框架——统一 `log`/`tracing` 入口、panic 崩溃落盘与弹窗、滚动文件输出。
- **目标用户**：公司内部 iced/egui/Tauri 桌面应用（etest、e-autotest 等）及 Rust 生态个人项目。
- **痛点**：多套日志接口割裂；崩溃信息丢失；文件日志无轮转。
- **边界**：只做日志与崩溃处理，不做日志检索/分析；Windows 优先（`FileShare`），Unix/macOS 可用。
- **成功指标**：所有 feature 组合可编译、示例可运行、`cargo test` 全绿、README 真实可用。
