# EasyDB

<div align="center">

<img src="public/128x128.png" alt="EasyDB Logo" style="background-color: gray; padding: 10px; border-radius: 8px;">

**一个轻量级的桌面数据查询工具，使用 SQL 直接查询本地文件，内置查询引擎**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-2.5.0-blue.svg)](https://github.com/shencangsheng/easydb_app)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows-lightgrey.svg)](https://github.com/shencangsheng/easydb_app)

[English](README_EN.md) | [中文](README.md)

</div>

## 📖 简介

EasyDB 是一个轻量级桌面数据查询工具，基于 Rust 构建，可以使用 SQL 直接查询本地文件。内置 DataFusion 查询引擎，无需额外安装数据库或其他工具。它将文件视为数据库表，可以使用标准 SQL 查询 CSV、Excel、JSON 和其他格式，支持复杂的多表 JOIN、子查询、窗口函数等高级 SQL 特性。轻松处理数百兆乃至数 GB 的大型文本文件，仅需较少的硬件资源。

![demo.gif](assets/demo.gif)

## ✨ 核心特性

- 🚀 **高性能**: 基于 Rust 和 DataFusion 引擎，处理大型文件游刃有余
- 💾 **低内存占用**: 仅需较少的硬件资源
- 📁 **多格式支持**: CSV、NdJson、JSON、Excel、Parquet 文件格式
- 🔧 **开箱即用**: 无需文件转换，直接查询
- 🖥️ **跨平台**: 支持 macOS 和 Windows 平台
- 🎨 **现代界面**: 基于 Tauri 构建的现代化桌面应用
- 🔍 **完整 SQL 支持**: 支持复杂 SQL 查询，包括 JOIN、子查询、窗口函数等高级特性

## 📖 更新日志

[更新日志](CHANGELOG.md)

## 🗺️ 功能与路线图

- [x] read_csv()
- [x] read_tsv()
- [x] read_ndjson()
- [ ] read_json()
- [x] read_excel()
- [x] read_parquet()
- [ ] Excel 实现懒加载性能优化
- [ ] Excel 兼容更多数据类型
- [ ] 支持多会话窗口
- [x] 支持拖拽文件自动生成 SQL 语句
- [ ] 支持目录浏览
- [ ] 支持 S3 远程文件
- [ ] 支持直接查询服务器上的文件
- [ ] 支持数据可视化
- [x] 支持查询结果导出
- [x] 支持将查询结果导出为 SQL 语句（Insert、Update）
- [x] read_mysql()

## 🛠️ 技术架构

### 核心技术栈

- **前端**: React + TypeScript + Vite
- **后端**: Rust + Tauri
- **查询引擎**: [apache/datafusion](https://github.com/apache/datafusion)
- **UI 框架**: HeroUI + Tailwind CSS

### 查询引擎选择

**当前使用**: DataFusion

DataFusion 是 Apache Arrow 项目的一部分，提供了完整的 SQL 查询能力，支持复杂的 SQL 语法，包括多表 JOIN、子查询、窗口函数等高级特性。相比 Polars，DataFusion 在 SQL 兼容性方面更加完善，能够满足更复杂的查询需求。

**版本演进**: v1.0 版本曾使用 Polars 引擎，虽然 Polars 在流式计算和内存占用方面表现优异，但在复杂 SQL 查询支持上存在限制。v2.0 版本切换回 DataFusion，以获得更完整的 SQL 支持，同时保持了良好的性能和资源利用效率。

## 📚 使用指南

### 基本语法

```sql
-- 查询 CSV 文件
SELECT *
FROM read_csv('/path/to/file.csv', infer_schema => false)
WHERE `age` > 30
LIMIT 10;

-- 查询 Excel 文件
SELECT *
FROM read_excel('/path/to/file.xlsx', sheet_name => 'Sheet2')
WHERE `age` > 30
LIMIT 10;

-- 查询 JSON 文件
SELECT *
FROM read_ndjson('/path/to/file.json')
WHERE `status` = 'active';

-- 查询 MySQL 数据库
SELECT *
FROM read_mysql('users', conn => 'mysql://user:password@localhost:3306/mydb')
WHERE `age` > 30;

-- 联合查询
SELECT *
FROM read_excel('/path/to/file.xlsx', sheet_name => 'Sheet1') as t1
inner join
read_mysql('users', conn => 'mysql://user:password@localhost:3306/mydb') as t2
on (t1.`user_id` = t2.`id`)
WHERE t1.`age` > 30;

-- 查询文本文件
SELECT *
FROM read_text('/path/to/file.txt')
WHERE `age` > 30
LIMIT 10;

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

### 语法问题

字段名可以使用双引号包裹，例如：

```sql
SELECT "id", "name" FROM table WHERE "id" = 1;
```

也可以使用反引号包裹，例如：

```sql
SELECT `id`, `name` FROM table WHERE `id` = 1;
```

WHERE 子句中的字符串值使用单引号包裹，例如：

```sql
SELECT * FROM table WHERE "id" = '1';
```

## 📖 项目背景

### 从 Server 到 App

[EasyDB Server](https://github.com/shencangsheng/easy_db) 主要部署于 Linux 服务器，作为 Web 服务支持大规模文本文件的高效查询。尽管已提供 Docker 部署方案，但在 macOS 上的使用仍不够便捷。

为此，我开发了 EasyDB App 客户端，专门为 macOS 和 Windows 平台优化，改善个人用户的本地使用体验。

### 项目命名

为了更好地区分两个项目：

- **EasyDB Server**: 服务器端版本，基于 DataFusion
- **EasyDB App**: 桌面客户端版本，基于 DataFusion（v2.0+）

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

- [apache/datafusion](https://github.com/apache/datafusion) - 高性能 SQL 查询引擎
- [Tauri](https://tauri.app/) - 现代桌面应用框架
- [React](https://reactjs.org/) - 用户界面库
- [HeroUI](https://heroui.com/) - 现代化 UI 组件库
- [datafusion-contrib](https://github.com/datafusion-contrib) - DataFusion 扩展

### 贡献者

<a href="https://github.com/shencangsheng/easydb_app/contributors">
  <img src="https://contrib.rocks/image?repo=shencangsheng/easydb_app" /></a>

## 📞 联系我们

- 🐛 **问题反馈**: [GitHub Issues](https://github.com/shencangsheng/easydb_app/issues)
- 💬 **讨论交流**: [GitHub Discussions](https://github.com/shencangsheng/easydb_app/discussions)
- 📧 **邮件联系**: shencangsheng@126.com

---

<div align="center">

**⭐ 如果这个项目对您有帮助，请给我们一个 Star！**

Made with ❤️ by [Cangsheng Shen](https://github.com/shencangsheng)

</div>

[![Star History Chart](https://api.star-history.com/svg?repos=shencangsheng/easydb_app&type=date&logscale&legend=top-left)](https://www.star-history.com/#shencangsheng/easydb_app&type=date&logscale&legend=top-left)
