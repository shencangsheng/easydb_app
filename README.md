# EasyDB

<div align="center">

![EasyDB Logo](public/128x128.png)

**一个简约强大的 SQL 桌面客户端，支持多种文件格式查询**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-0.5.0-blue.svg)](https://github.com/shencangsheng/easydb_app)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows-lightgrey.svg)](https://github.com/shencangsheng/easydb_app)

[English](README_EN.md) | [中文](README.md)

</div>

## 📖 简介

EasyDB 是一个简约强大的 SQL 桌面客户端，基于 Rust 构建，具备高性能的文件查询能力，轻松处理数百兆乃至数 GB 的大型文本文件，仅需极少的硬件资源。支持 CSV、NdJson、JSON、Excel 和 Parquet 文件格式，无需进行文件转换，开箱即用。

### 🎯 设计理念

EasyDB 旨在简化文本文件查询过程，让您能够将多个文本文件视为一个数据库，并使用熟悉的 SQL 语法进行查询。无论是数据分析师、开发者还是普通用户，都能轻松上手。

## ✨ 核心特性

- 🚀 **高性能**: 基于 Rust 和 Polars 引擎，处理大型文件游刃有余
- 💾 **低内存占用**: 流式计算能力，仅需极少的硬件资源
- 📁 **多格式支持**: CSV、NdJson、JSON、Excel、Parquet 文件格式
- 🔧 **开箱即用**: 无需文件转换，直接查询
- 🖥️ **跨平台**: 支持 macOS 和 Windows 平台
- 🎨 **现代界面**: 基于 Tauri 构建的现代化桌面应用

## 🗺️ 功能与路线图

- [x] 支持 CSV 文件查询
- [x] 支持 TSV 文件查询
- [x] 支持 JSON 文件查询
- [x] 支持 Excel 文件查询
- [ ] 支持 Parquet 文件查询
- [ ] Excel 实现懒加载性能优化
- [ ] Excel 兼容更多数据类型
- [ ] 支持多会话窗口
- [x] 支持拖拽文件自动生成 SQL 语句
- [ ] 支持目录浏览
- [ ] 支持 S3 远程文件
- [ ] 支持直接查询服务器上的文件
- [ ] 支持数据可视化
- [ ] 支持查询结果导出

## 🛠️ 技术架构

### 核心技术栈

- **前端**: React + TypeScript + Vite
- **后端**: Rust + Tauri
- **查询引擎**: [pola-rs/polars](https://github.com/pola-rs/polars)
- **UI 框架**: HeroUI + Tailwind CSS

### 查询引擎选择

**当前使用**: Polars

与 DataFusion 相比，Polars 具备更高的轻量性和流式计算能力，显著降低了内存占用，更加适合个人电脑使用。

**技术考虑**: 在深入使用 Polars 后发现其技术短板也很明显，无法支持复杂 SQL 查询，并且社区的开发资源主要集中在 Python 上，很多功能需要自己开发或兼容。因此正在考虑换回 DataFusion 以获得更完整的 SQL 支持。

## 📚 使用指南

### 基本语法

```sql
-- 查询 CSV 文件
SELECT *
FROM read_csv('/path/to/file.csv', infer_schema => false)
WHERE age > 30
LIMIT 10;

-- 查询 Excel 文件
SELECT *
FROM read_excel('/path/to/file.xlsx', sheet_name => 'Sheet2')
WHERE age > 30
LIMIT 10;

-- 查询 JSON 文件
SELECT *
FROM read_json('/path/to/file.json')
WHERE status = 'active';
```

### 支持的文件格式

| 格式    | 函数             | 说明                   |
| ------- | ---------------- | ---------------------- |
| CSV     | `read_csv()`     | 支持自定义分隔符和编码 |
| Excel   | `read_excel()`   | 支持多工作表           |
| JSON    | `read_json()`    | 支持嵌套结构           |
| NdJson  | `read_ndjson()`  | 每行一个 JSON 对象     |
| Parquet | `read_parquet()` | 列式存储格式           |

## 🚀 快速开始

### 系统要求

- **macOS**: 10.15+ (Catalina 或更高版本)
- **Windows**: Windows 10 或更高版本
- **内存**: 建议 4GB 以上
- **存储**: 至少 100MB 可用空间

### 安装方式

1. **下载安装包**

   - 访问 [Releases](https://github.com/shencangsheng/easydb_app/releases) 页面
   - 下载适合您系统的安装包

2. **安装应用**

   - **macOS**: 下载 `.dmg` 文件，拖拽到应用程序文件夹
   - **Windows**: 下载 `.exe` 文件，运行安装程序

## ❓ 常见问题

### macOS 应用损坏问题

**问题**: 在 macOS 上打开 EasyDB 时提示"应用已损坏，无法打开"

**解决方案**: 这是由于 macOS 的安全机制（Gatekeeper）阻止了未签名的应用。请按以下步骤解决：

1. 打开终端（Terminal）
2. 执行以下命令移除隔离属性：
   ```bash
   xattr -r -d com.apple.quarantine /Applications/EasyDB.app
   ```
3. 重新尝试打开应用

**替代方案**: 如果上述方法无效，可以尝试在系统偏好设置中允许该应用：

1. 打开"系统偏好设置" > "安全性与隐私"
2. 在"通用"标签页中，找到被阻止的应用
3. 点击"仍要打开"按钮

### JOIN 查询错误

**问题**: 在执行 JOIN 查询时出现 `unsupported SQL join constraint` 异常

**解决方案**: 去掉 ON 表达式的括号。这是因为 Polars 的限制：它目前的 join constraint 只支持最简单的等值连接。

```sql
-- ❌ 错误写法
SELECT *
FROM table1 t1
JOIN table2 t2 ON (t1.id = t2.id);

-- ✅ 正确写法
SELECT *
FROM table1 t1
JOIN table2 t2 ON t1.id = t2.id;
```

## 📖 项目背景

### 从 Server 到 App

[EasyDB Server](https://github.com/shencangsheng/easy_db) 主要部署于 Linux 服务器，作为 Web 服务支持大规模文本文件的高效查询。尽管已提供 Docker 部署方案，但在 macOS 上的使用仍不够便捷。

为此，我开发了 EasyDB App 客户端，专门为 macOS 和 Windows 平台优化，改善个人用户的本地使用体验。

### 项目命名

为了更好地区分两个项目：

- **EasyDB Server**: 服务器端版本，基于 DataFusion
- **EasyDB App**: 桌面客户端版本，基于 Polars

## 🤝 贡献指南

我们欢迎各种形式的贡献！

### 如何贡献

1. **Fork** 本仓库
2. 创建您的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交您的更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开一个 **Pull Request**

### 开发环境

```bash
# 克隆仓库
git clone https://github.com/shencangsheng/easydb_app.git
cd easydb_app

# 启动开发服务器
cargo tauri dev

# 构建应用
cargo tauri build
```

## 📄 许可证

A short snippet describing the license (MIT)

MIT © Cangsheng Shen

## 👨‍💻 作者

**Cangsheng Shen**

- GitHub: [@shencangsheng](https://github.com/shencangsheng)
- Email: shencangsheng@126.com

## 🙏 致谢

感谢以下开源项目的支持：

- [pola-rs/polars](https://github.com/pola-rs/polars) - 高性能数据处理引擎
- [Tauri](https://tauri.app/) - 现代桌面应用框架
- [React](https://reactjs.org/) - 用户界面库
- [HeroUI](https://heroui.com/) - 现代化 UI 组件库

## 📞 联系我们

- 🐛 **问题反馈**: [GitHub Issues](https://github.com/shencangsheng/easydb_app/issues)
- 💬 **讨论交流**: [GitHub Discussions](https://github.com/shencangsheng/easydb_app/discussions)
- 📧 **邮件联系**: shencangsheng@126.com

---

<div align="center">

**⭐ 如果这个项目对您有帮助，请给我们一个 Star！**

Made with ❤️ by [Cangsheng Shen](https://github.com/shencangsheng)

</div>
