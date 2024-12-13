<div align="center">
  <h1>E-LOG</h1>
  <p><strong>A Log Frame</strong></p>
</div>

<div align="center">
  
[![Test Status](https://github.com/rust-random/rand/workflows/Tests/badge.svg?event=push)](https://github.com/eternalnight996/hw/actions)[![Book](https://img.shields.io/badge/book-master-yellow.svg)](https://doc.rust-lang.org/book/)[![API](https://img.shields.io/badge/api-master-yellow.svg)](https://github.com/eternalnight996/hw)
[![API](https://docs.rs/hw/badge.svg)](https://docs.rs/rand)[![Rust Version](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

English | [简体中文](readme.zh.md)


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
# default tracing
e-log = { version = "0.3" }
# log
e-log = {version = "0.3", default-features = false, features = ["log","panic","dialog"]}
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

This project is dual-licensed under [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE).

---

<div align="center">
  <sub>Built with ❤️ by eternalnight996 and contributors.</sub>
</div>

