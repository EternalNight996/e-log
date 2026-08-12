<div align="center">
  <h1>E-LOG</h1>
  <p><strong>A Log Frame</strong></p>
</div>

<div align="center">
  
[![API](https://img.shields.io/badge/api-master-yellow.svg)](https://github.com/eternalnight996/e-log)[![API](https://docs.rs/e-log/badge.svg)](https://docs.rs/e-log)[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

English | [简体中文](README.zh.md)

</div>

### Support app
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

# 📖 Example
```toml
[dependencies]
# default log
e-log = { version = "0.4" }
# log
e-log = {version = "0.4", default-features = false, features = ["log","panic","dialog"]}
```


---
## License

[LICENSE](LICENSE)
[COPYRIGHT](COPYRIGHT)

## 🤝 Contributing

We welcome any form of contribution!

- Submit Issues to report bugs or suggest new features
- Submit Pull Requests to improve code
- Improve project documentation
- Share usage experiences

Before submitting a PR, please ensure:
1. Code complies with project standards
2. Add necessary tests
3. Update relevant documentation

## 📜 License

This project is dual-licensed under MIT and Apache 2.0. See [LICENSE](LICENSE) and [COPYRIGHT](COPYRIGHT).

---

<div align="center">
  <sub>Built with ❤️ by eternalnight996 and contributors.</sub>
</div>

