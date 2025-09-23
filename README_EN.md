# EasyDB

<div align="center">

![EasyDB Logo](public/logo.png)

**A simple yet powerful SQL desktop client supporting multiple file formats**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/shencangsheng/easydb_app)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows-lightgrey.svg)](https://github.com/shencangsheng/easydb_app)

[English](README_EN.md) | [中文](README.md)

</div>

## 📖 Introduction

EasyDB is a simple yet powerful SQL desktop client built with Rust, featuring high-performance file querying capabilities that effortlessly handle hundreds of megabytes to multiple gigabytes of large text files with minimal hardware resources. It supports CSV, NdJson, JSON, Excel, and Parquet file formats without requiring file conversion, ready to use out of the box.

### 🎯 Design Philosophy

EasyDB aims to simplify the text file querying process, allowing you to treat multiple text files as a database and query them using familiar SQL syntax. Whether you're a data analyst, developer, or regular user, you can easily get started.

## ✨ Core Features

- 🚀 **High Performance**: Built on Rust and Polars engine, effortlessly handles large files
- 💾 **Low Memory Usage**: Stream processing capabilities, requires minimal hardware resources
- 📁 **Multi-format Support**: CSV, NdJson, JSON, Excel, Parquet file formats
- 🔧 **Ready to Use**: No file conversion required, query directly
- 🖥️ **Cross-platform**: Supports macOS and Windows platforms
- 🎨 **Modern Interface**: Modern desktop application built with Tauri

## 🛠️ Technical Architecture

### Core Technology Stack

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Tauri
- **Query Engine**: [pola-rs/polars](https://github.com/pola-rs/polars)
- **UI Framework**: HeroUI + Tailwind CSS

### Query Engine Selection

**Currently Using**: Polars

Compared to DataFusion, Polars has higher lightweight and stream processing capabilities, significantly reducing memory usage, making it more suitable for personal computers.

**Technical Considerations**: After in-depth use of Polars, its technical limitations have become apparent - it cannot support complex SQL queries, and the community's development resources are mainly focused on Python, requiring self-development or compatibility work for many features. Therefore, we are considering switching back to DataFusion for more complete SQL support.

## 📚 User Guide

### Basic Syntax

```sql
-- Query CSV files
SELECT *
FROM read_csv('/path/to/file.csv', infer_schema => false)
WHERE age > 30
LIMIT 10;

-- Query Excel files
SELECT *
FROM read_excel('/path/to/file.xlsx', sheet_name => 'Sheet2')
WHERE age > 30
LIMIT 10;

-- Query JSON files
SELECT *
FROM read_json('/path/to/file.json')
WHERE status = 'active';
```

### Supported File Formats

| Format  | Function         | Description                             |
| ------- | ---------------- | --------------------------------------- |
| CSV     | `read_csv()`     | Supports custom delimiters and encoding |
| Excel   | `read_excel()`   | Supports multiple worksheets            |
| JSON    | `read_json()`    | Supports nested structures              |
| NdJson  | `read_ndjson()`  | One JSON object per line                |
| Parquet | `read_parquet()` | Columnar storage format                 |

## 🚀 Quick Start

### System Requirements

- **macOS**: 10.15+ (Catalina or higher)
- **Windows**: Windows 10 or higher
- **Memory**: Recommended 4GB or more
- **Storage**: At least 100MB available space

### Installation

1. **Download Installer**

   - Visit [Releases](https://github.com/shencangsheng/easydb_app/releases) page
   - Download the installer for your system

2. **Install Application**

   - **macOS**: Download `.dmg` file, drag to Applications folder
   - **Windows**: Download `.exe` file, run the installer

3. **Launch Application**

   - Double-click the app icon to start EasyDB
   - Begin your data querying journey!

## ❓ Frequently Asked Questions

### JOIN Query Error

**Issue**: Getting `unsupported SQL join constraint` error when executing JOIN queries

**Solution**: Remove parentheses from ON expressions. This is due to Polars' limitation: it currently only supports the simplest equality joins for join constraints.

```sql
-- ❌ Incorrect syntax
SELECT *
FROM table1 t1
JOIN table2 t2 ON (t1.id = t2.id);

-- ✅ Correct syntax
SELECT *
FROM table1 t1
JOIN table2 t2 ON t1.id = t2.id;
```

## 📖 Project Background

### From Server to App

[EasyDB Server](https://github.com/shencangsheng/easy_db) is mainly deployed on Linux servers as a web service supporting efficient querying of large-scale text files. Although Docker deployment solutions are provided, usage on macOS is still not convenient enough.

For this reason, I developed the EasyDB App client, specifically optimized for macOS and Windows platforms to improve the local user experience.

### Project Naming

To better distinguish between the two projects:

- **EasyDB Server**: Server-side version, based on DataFusion
- **EasyDB App**: Desktop client version, based on Polars

## 🤝 Contributing

We welcome contributions in all forms!

### How to Contribute

1. **Fork** this repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a **Pull Request**

### Development Environment

```bash
# Clone repository
git clone https://github.com/shencangsheng/easydb_app.git
cd easydb_app

# Start development server
cargo tauri dev

# Build application
cargo tauri build
```

## 📄 License

This project is licensed under the [MIT License](LICENSE) - see the LICENSE file for details.

## 👨‍💻 Author

**Cangsheng Shen**

- GitHub: [@shencangsheng](https://github.com/shencangsheng)
- Email: shencangsheng@126.com

## 🙏 Acknowledgments

Thanks to the following open source projects:

- [pola-rs/polars](https://github.com/pola-rs/polars) - High-performance data processing engine
- [Tauri](https://tauri.app/) - Modern desktop application framework
- [React](https://reactjs.org/) - User interface library
- [HeroUI](https://heroui.com/) - Modern UI component library

## 📞 Contact Us

- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/shencangsheng/easydb_app/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/shencangsheng/easydb_app/discussions)
- 📧 **Email**: shencangsheng@126.com

---

<div align="center">

**⭐ If this project helps you, please give us a Star!**

Made with ❤️ by [Cangsheng Shen](https://github.com/shencangsheng)

</div>
