<div align="center">

<h1>EasyDB</h1>

<img src="public/readme-logo.svg?v=2" alt="EasyDB Logo" width="80" height="80">

**A lightweight desktop data query tool that uses SQL to query local files directly, with a built-in query engine**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Release](https://img.shields.io/badge/release-2.7.0-blue.svg)](https://github.com/shencangsheng/easydb_app)
[![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Windows-brightgreen)](https://github.com/shencangsheng/easydb_app)
![Stars](https://img.shields.io/github/stars/shencangsheng/easydb_app?logo=github)
![Total Downloads](https://img.shields.io/github/downloads/shencangsheng/easydb_app/total)

[English](README.md) | [中文](README_CN.md)

</div>

---

## Introduction

EasyDB is a lightweight desktop data query tool built with Rust and Tauri, featuring a built-in Apache DataFusion query engine. No need to install a database or any other dependencies — just use SQL to query local files directly.

It treats files as database tables, supporting CSV, TSV, Text, NdJson, Excel, Parquet, and MySQL as data sources. It supports complex multi-table JOINs, subqueries, window functions, and other advanced SQL features. It effortlessly handles data files from hundreds of MB to multiple GB with minimal hardware resources.

![demo.gif](assets/demo.gif)

## Core Features

- **High Performance** — Built on Rust and DataFusion, effortlessly handles large files
- **Low Memory Usage** — Runs with minimal hardware resources
- **Multi-format Support** — CSV, TSV, Text, NdJson, Excel, Parquet, MySQL
- **Ready to Use** — No file conversion needed, query directly
- **Cross-platform** — Supports macOS and Windows
- **Full SQL Support** — Multi-table JOINs, subqueries, window functions, regex matching, and more
- **Smart Editor** — SQL syntax highlighting, autocomplete (function names + column names), formatting
- **Drag & Drop SQL Generation** — Drag files into the editor to auto-generate query statements
- **Internationalization** — Supports Simplified Chinese and English, auto-detects browser language
- **Virtual Scrolling & Pagination** — Smooth rendering for large datasets with scroll-to-load
- **Result Export** — Export to CSV, TSV, or SQL (INSERT/UPDATE), with MySQL or PostgreSQL dialect options
- **Query History** — Automatically records the last 50 queries with execution status
- **Modern Interface** — Native desktop app built with Tauri v2 + HeroUI

## Changelog

See [CHANGELOG_EN.md](CHANGELOG_EN.md)

## Features & Roadmap

### File Reader Functions

- [x] `read_csv()` — Read CSV files with custom delimiter, header, and schema inference options
- [x] `read_tsv()` — Read TSV files
- [x] `read_text()` — Read text files with custom delimiter
- [ ] `read_json()` — Read JSON files (temporarily removed in v2.0, planned for reimplementation)
- [x] `read_excel()` / `read_xlsx()` — Read Excel files with worksheet selection
- [x] `read_parquet()` — Read Parquet columnar storage files
- [x] `read_ndjson()` — Read NDJSON files
- [x] `read_mysql()` — Read MySQL database tables

### Scalar Functions

- [x] `REGEXP_LIKE()` — Regular expression matching

### Editor & Interaction

- [x] Drag & drop file auto-generate SQL (option to insert full SQL or just `read_xxx()` function)
- [x] SQL smart autocomplete (function names, parameter names, result column names)
- [x] Execute selected SQL fragment
- [x] SQL formatting and clearing

### Data Export

- [x] Export query results to CSV / TSV / SQL
- [x] SQL export supports INSERT and UPDATE statements
- [x] SQL export supports MySQL and PostgreSQL dialects
- [x] Query history recording

### Planned

- [ ] Excel lazy loading performance optimization
- [ ] Excel enhanced data type compatibility
- [ ] Multi-session window support
- [ ] Directory browsing
- [ ] S3 remote file support
- [ ] Direct querying of server files
- [ ] Data visualization

## Technical Architecture

### Core Tech Stack

| Layer           | Technology                                                                                     |
| --------------- | ---------------------------------------------------------------------------------------------- |
| Frontend        | React 18 + TypeScript + Vite                                                                   |
| Backend         | Rust + Tauri v2                                                                                |
| Query Engine    | [Apache DataFusion](https://github.com/apache/datafusion) 50.3                                 |
| DB Connector    | [datafusion-table-providers](https://github.com/apache/arrow-datafusion-table-providers) MySQL |
| UI Framework    | HeroUI + Tailwind CSS                                                                          |
| Virtual Scroll  | @tanstack/react-virtual + @tanstack/react-table                                                |
| SQL Editor      | Ace Editor (react-ace)                                                                         |
| SQL Parsing     | sqlparser-rs (Rust) + node-sql-parser (JS)                                                     |
| i18n            | Lightweight custom i18n, zh-CN / en-US                                                         |
| History Storage | SQLite (rusqlite)                                                                              |

### Query Engine Selection

**Currently Using**: Apache DataFusion

DataFusion is part of the Apache Arrow project, providing complete SQL query capabilities and supporting complex SQL syntax including multi-table JOINs, subqueries, window functions, and other advanced features. Compared to Polars, DataFusion offers more comprehensive SQL compatibility.

**Version Evolution**: v1.0 previously used the Polars engine, which excelled in stream processing and memory usage but had limitations in complex SQL support. v2.0 switched back to DataFusion for more complete SQL support while maintaining good performance and resource efficiency.

## User Guide

### Basic Syntax

```sql
-- Query CSV files
SELECT *
FROM read_csv('/path/to/file.csv', infer_schema => false)
WHERE "age" > 30
LIMIT 10;

-- Query TSV files
SELECT *
FROM read_tsv('/path/to/file.tsv');

-- Query text files (custom delimiter)
SELECT *
FROM read_text('/path/to/file.txt', delimiter => '\t');

-- Query Excel files (specific worksheet)
SELECT *
FROM read_excel('/path/to/file.xlsx', sheet_name => 'Sheet2')
WHERE "age" > 30;

-- Query NDJSON files
SELECT *
FROM read_ndjson('/path/to/file.json')
WHERE "status" = 'active';

-- Query Parquet files
SELECT *
FROM read_parquet('/path/to/file.parquet');

-- Query MySQL database
SELECT *
FROM read_mysql('users', conn => 'mysql://user:password@localhost:3306/mydb')
WHERE "age" > 30;

-- Cross-source join (Excel + MySQL)
SELECT *
FROM read_excel('/path/to/file.xlsx', sheet_name => 'Sheet1') AS t1
INNER JOIN
read_mysql('users', conn => 'mysql://user:password@localhost:3306/mydb') AS t2
ON t1."user_id" = t2."id"
WHERE t1."age" > 30;

-- Regex matching
SELECT *
FROM read_csv('/path/to/file.csv')
WHERE REGEXP_LIKE("Distance", '^([0-9]+)\.([0-9]+)?$');
```

### Supported Data Sources

| Format  | Function                       | Description                                |
| ------- | ------------------------------ | ------------------------------------------ |
| CSV     | `read_csv()`                   | Custom delimiter, header, schema inference |
| TSV     | `read_tsv()`                   | Tab-separated files                        |
| Text    | `read_text()`                  | General text files with custom delimiter   |
| Excel   | `read_excel()` / `read_xlsx()` | `.xlsx` support, optional worksheet        |
| NdJson  | `read_ndjson()`                | One JSON object per line                   |
| Parquet | `read_parquet()`               | Columnar storage format                    |
| MySQL   | `read_mysql()`                 | Direct MySQL database table connection     |

### Function Parameters

<details>
<summary><code>read_csv()</code> parameters</summary>

| Parameter        | Type    | Default | Description                                                |
| ---------------- | ------- | ------- | ---------------------------------------------------------- |
| `infer_schema`   | boolean | true    | Auto-infer data types (based on first 100 rows)            |
| `has_header`     | boolean | true    | Whether the file contains a header row                     |
| `delimiter`      | string  | `,`     | Field delimiter, supports escape sequences like `\t`, `\n` |
| `file_extension` | string  | `.csv`  | File extension                                             |

</details>

<details>
<summary><code>read_excel()</code> parameters</summary>

| Parameter      | Type    | Default     | Description                   |
| -------------- | ------- | ----------- | ----------------------------- |
| `sheet_name`   | string  | First sheet | Name of the worksheet to read |
| `infer_schema` | boolean | true        | Auto-infer data types         |

</details>

<details>
<summary><code>read_mysql()</code> parameters</summary>

| Parameter | Type   | Default      | Description                                                              |
| --------- | ------ | ------------ | ------------------------------------------------------------------------ |
| `conn`    | string | **Required** | MySQL connection string, e.g. `mysql://user:password@host:port/database` |

</details>

<details>
<summary><code>read_text()</code> parameters</summary>

| Parameter        | Type    | Default | Description                            |
| ---------------- | ------- | ------- | -------------------------------------- |
| `infer_schema`   | boolean | true    | Auto-infer data types                  |
| `has_header`     | boolean | true    | Whether the file contains a header row |
| `delimiter`      | string  | `\t`    | Field delimiter                        |
| `file_extension` | string  | `.txt`  | File extension                         |

</details>

## Quick Start

### System Requirements

- **macOS**: 10.15+ (Catalina or higher)
- **Windows**: Windows 10 or higher
- **Memory**: 4 GB or more recommended
- **Storage**: At least 100 MB available space

### Installation

1. Visit the [Releases](https://github.com/shencangsheng/easydb_app/releases) page and download the installer for your system
2. **macOS**: Download the `.dmg` file, drag to Applications folder
3. **Windows**: Download the `.exe` file, run the installer

## FAQ

### macOS "Application is damaged and cannot be opened"

This is caused by macOS Gatekeeper blocking unsigned applications. Run the following command in Terminal:

```bash
xattr -r -d com.apple.quarantine /Applications/EasyDB.app
```

If this doesn't work, go to **System Preferences > Security & Privacy > General** and click "Open Anyway".

### SQL Syntax Notes

Field names should be wrapped in double quotes:

```sql
SELECT "id", "name" FROM table WHERE "id" = 1;
```

Backticks also work:

```sql
SELECT `id`, `name` FROM table WHERE `id` = 1;
```

String values in WHERE clauses use single quotes:

```sql
SELECT * FROM table WHERE "id" = '1';
```

### Non-standard File Extensions

For files without CSV, XLSX, JSON, or Parquet extensions, EasyDB automatically uses the `read_text()` function.

## Project Background

### From Server to App

[EasyDB Server](https://github.com/shencangsheng/easy_db) is primarily deployed on Linux servers as a web service for efficient querying of large-scale text files. Although Docker deployment is available, the local experience on macOS and Windows is not as convenient.

EasyDB App is specifically optimized for macOS and Windows to improve the local user experience.

### Project Naming

- **EasyDB Server** — Server-side version, based on DataFusion
- **EasyDB App** — Desktop client version, based on DataFusion (v2.0+)

## Contributing

Contributions are welcome in all forms!

1. **Fork** this repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a **Pull Request**

### Development Environment

```bash
# Clone repository
git clone https://github.com/shencangsheng/easydb_app.git
cd easydb_app

# Install frontend dependencies
yarn

# Start development server
cargo tauri dev

# Build application
cargo tauri build
```

### Prerequisites

- Rust 1.89+
- Node.js 18+
- Yarn

## License

[MIT License](LICENSE) © Cangsheng Shen

## Author

**Cangsheng Shen**

- GitHub: [@shencangsheng](https://github.com/shencangsheng)
- Email: shencangsheng@126.com

## Acknowledgments

Thanks to the following open source projects:

- [Apache DataFusion](https://github.com/apache/datafusion) — High-performance SQL query engine
- [datafusion-table-providers](https://github.com/apache/arrow-datafusion-table-providers) — DataFusion extension (MySQL support)
- [Tauri](https://tauri.app/) — Modern desktop application framework
- [React](https://reactjs.org/) — User interface library
- [HeroUI](https://heroui.com/) — UI component library
- [calamine](https://github.com/calamine/calamine) — Excel file parsing
- [Ace Editor](https://ace.c9.io/) — Code editor

### Contributors

<a href="https://github.com/shencangsheng/easydb_app/contributors">
  <img src="https://contrib.rocks/image?repo=shencangsheng/easydb_app" />
</a>

## Contact

- Bug Reports: [GitHub Issues](https://github.com/shencangsheng/easydb_app/issues)
- Discussions: [GitHub Discussions](https://github.com/shencangsheng/easydb_app/discussions)
- Email: shencangsheng@126.com

---

<div align="center">

**If this project helps you, please give us a Star**

Made with ❤️ by [Cangsheng Shen](https://github.com/shencangsheng)

</div>
