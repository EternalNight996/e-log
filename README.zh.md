<div align="center">
  <h1>E-LOG</h1>
  <p><strong>便捷日志框架</strong></p>
</div>

<div align="center">
  
[![API](https://img.shields.io/badge/api-master-yellow.svg)](https://github.com/eternalnight996/e-log)[![API](https://docs.rs/e-log/badge.svg)](https://docs.rs/e-log)[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

[English](readme.md) | 简体中文
</div>

### 支持 功能
<table>
  <tr>
    <th><h3>功能</h3></th>
    <th><h3>Windows 10</h3></th>
    <th><h3>Unix</h3></th>
    <th><h3>Macos</h3></th>
  </tr>
  <tr>
    <td>panic</td>
    <td><h4 style="color:green">√</h4></td>
    <td><h4 style="color:green">√</h4></td>
    <td><h4 style="color:green">√</h4></td>
  </tr>
  <tr>
  <tr>
    <td>log</td>
    <td><h4 style="color:green">√</h4></td>
    <td><h4 style="color:green">√</h4></td>
    <td><h4 style="color:green">√</h4></td>
  </tr>
  <tr>
    <td>tracing</td>
    <td><h4 style="color:green">√</h4></td>
    <td><h4 style="color:green">√</h4></td>
    <td><h4 style="color:green">√</h4></td>
  </tr>
  <tr>
  <tr>
    <td>_</td>
    <td><h4 style="color:red">×</h4></td>
    <td><h4 style="color:red">×</h4></td>
    <td><h4 style="color:red">×</h4></td>
  </tr>
</table>

# 📖 示例
```toml
[dependencies]
# default log
e-log = { version = "0.4" }
# log
e-log = {version = "0.4", default-features = false, features = ["log","panic","dialog"]}
```


---
## 许可证

[LICENSE](LICENSE)
[COPYRIGHT](COPYRIGHT)

## 🤝 参与贡献

我们欢迎任何形式的贡献！

- 提交 Issue 报告 bug 或提出新功能建议
- 提交 Pull Request 改进代码
- 完善项目文档
- 分享使用经验

在提交 PR 之前，请确保：
1. 代码符合项目规范
2. 添加必要的测试
3. 更新相关文档

## 📜 开源协议

本项目采用 MIT 和 Apache 2.0 双重协议，详见 [LICENSE](LICENSE) 与 [COPYRIGHT](COPYRIGHT)。

---

<div align="center">
  <sub>Built with ❤️ by eternalnight996 and contributors.</sub>
</div>
