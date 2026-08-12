# ADR-003：中文注释"乱码"为显示假象

- **背景**：审查时 `Get-Content` 显示注释为乱码（如 `鏃ュ織`），怀疑 GBK 双重编码损坏。
- **调查**：PowerShell 5.1 的 `Get-Content` 对无 BOM 的 UTF-8 文件按系统 ANSI（GBK）解码；字节级检查确认文件为正常 UTF-8，无乱码字节。
- **结论**：无需转码。后续检查本仓库文件使用 `Get-Content -Encoding UTF8` 或字节级验证。
- **教训**：中文 Windows 环境下，工具输出需先排除编码显示假象再判断内容损坏。
