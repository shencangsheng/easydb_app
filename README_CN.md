<div align="center">
<h1>EasyDB</h1>

<img src="public/readme-logo.svg?v=2" alt="EasyDB Logo" width="80" height="80">

**轻量级桌面数据查询工具，用 SQL 直接查询本地文件，内置查询引擎**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Release](https://img.shields.io/badge/release-2.12.0-blue.svg)](https://github.com/shencangsheng/easydb_app)
[![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Windows-brightgreen)](https://github.com/shencangsheng/easydb_app)
![Stars](https://img.shields.io/github/stars/shencangsheng/easydb_app?logo=github)
![Total Downloads](https://img.shields.io/github/downloads/shencangsheng/easydb_app/total)

[English](README.md) | [中文](README_CN.md)

</div>

---

## 简介

EasyDB 是一个轻量级桌面数据查询工具，基于 Rust 与 Tauri 构建，内置 Apache DataFusion 查询引擎，无需安装数据库或其他依赖即可使用 SQL 直接查询本地文件。

将文件视为数据库表，支持 CSV、TSV、Text、JSON、NdJson、Excel、Parquet、MySQL 以及 PostgreSQL 等多种数据源，支持复杂的多表 JOIN、子查询、窗口函数等高级 SQL 特性。轻松处理数百 MB 乃至数 GB 的数据文件，仅需少量硬件资源。

![demo.gif](assets/demo.gif)

## 核心特性

- **高性能** — 基于 Rust 和 DataFusion 引擎，处理大型文件游刃有余
- **低内存占用** — 仅需少量硬件资源即可运行
- **多格式支持** — CSV、TSV、Text、JSON、NdJson、Excel、Parquet、MySQL、PostgreSQL
- **开箱即用** — 无需文件转换，直接查询本地文件
- **跨平台** — 支持 macOS 和 Windows
- **完整 SQL 支持** — 多表 JOIN、子查询、窗口函数、正则匹配等
- **智能编辑器** — SQL 语法高亮、自动补全（函数名 + 列名）、格式化
- **拖拽生成 SQL** — 拖拽文件至编辑器自动生成查询语句
- **国际化** — 支持简体中文与英文，自动检测浏览器语言
- **虚拟滚动分页** — 大数据集流畅渲染，滚动加载下一页
- **查询结果导出** — 支持导出为 CSV、TSV、SQL（INSERT / UPDATE），可选 MySQL 或 PostgreSQL 方言
- **查询历史** — 自动记录最近 50 条查询及执行状态
- **现代界面** — 基于 Tauri v2 + HeroUI 构建的原生桌面应用

## 更新日志

详见 [CHANGELOG.md](CHANGELOG.md)

## 功能与路线图

### 文件读取函数

- [x] `read_csv()` — 读取 CSV 文件，支持自定义分隔符、表头、Schema 推断
- [x] `read_tsv()` — 读取 TSV 文件
- [x] `read_text()` — 读取文本文件，支持自定义分隔符
- [x] `read_json()` — 读取 JSON 文件，支持标准 JSON 数组与 NDJSON（自动检测格式）
- [x] `read_ndjson()` — 读取 NDJSON 文件（每行一个 JSON 对象）
- [x] `read_excel()` / `read_xlsx()` — 读取 Excel 文件，支持指定工作表
- [x] `read_parquet()` — 读取 Parquet 列式存储文件
- [x] `read_mysql()` — 读取 MySQL 数据库表
- [x] `read_postgres()` — 读取 PostgreSQL 数据库表

### 标量函数

- [x] `REGEXP_LIKE()` — 正则表达式匹配

### 编辑器与交互

- [x] 拖拽文件自动生成 SQL 语句（可选插入完整 SQL 或仅插入 `read_xxx()` 函数）
- [x] SQL 智能补全（函数名、参数名、查询结果列名）
- [x] 选中片段执行
- [x] SQL 格式化与清空

### 数据导出

- [x] 查询结果导出为 CSV / TSV / SQL
- [x] SQL 导出支持 INSERT 与 UPDATE 语句
- [x] SQL 导出支持 MySQL 和 PostgreSQL 方言
- [x] 查询历史记录

### 计划中

- [ ] Excel 懒加载性能优化
- [ ] Excel 兼容更多数据类型
- [ ] 多会话窗口
- [ ] 目录浏览
- [ ] S3 远程文件
- [ ] 远程服务器文件查询
- [ ] 数据可视化

## 技术架构

### 核心技术栈

| 层级       | 技术                                                           |
| ---------- | -------------------------------------------------------------- |
| 前端       | React 18 + TypeScript + Vite                                   |
| 后端       | Rust + Tauri v2                                                |
| 查询引擎   | [Apache DataFusion](https://github.com/apache/datafusion) 50.3 |
| UI 框架    | HeroUI + Tailwind CSS                                          |
| 虚拟滚动   | @tanstack/react-virtual + @tanstack/react-table                |
| SQL 编辑器 | Ace Editor (react-ace)                                         |
| SQL 解析   | sqlparser-rs (Rust) + node-sql-parser (JS)                     |
| 国际化     | 自建轻量 i18n，支持 zh-CN / en-US                              |
| 历史存储   | SQLite (rusqlite)                                              |

### 查询引擎选择

**当前使用**: Apache DataFusion

DataFusion 是 Apache Arrow 项目的一部分，提供完整的 SQL 查询能力，支持多表 JOIN、子查询、窗口函数等高级特性。相比 Polars，DataFusion 在 SQL 兼容性方面更加完善。

**版本演进**: v1.0 曾使用 Polars 引擎，其在流式计算和内存占用方面表现优异，但复杂 SQL 支持存在限制。v2.0 切换回 DataFusion，获得更完整的 SQL 支持，同时保持了良好的性能与资源效率。

## 使用指南

### 基本语法

```sql
-- 查询 CSV 文件
SELECT *
FROM read_csv('/path/to/file.csv', infer_schema => false)
WHERE "age" > 30
LIMIT 10;

-- 查询 TSV 文件
SELECT *
FROM read_tsv('/path/to/file.tsv');

-- 查询文本文件（自定义分隔符）
SELECT *
FROM read_text('/path/to/file.txt', delimiter => '\t');

-- 查询 Excel 文件（指定工作表）
SELECT *
FROM read_excel('/path/to/file.xlsx', sheet_name => 'Sheet2')
WHERE "age" > 30;

-- 查询 JSON 文件（标准 JSON 数组或 NDJSON，自动检测）
SELECT *
FROM read_json('/path/to/file.json')
WHERE "status" = 'active';

-- 查询 NDJSON 文件
SELECT *
FROM read_ndjson('/path/to/file.ndjson')
WHERE "status" = 'active';

-- 查询 Parquet 文件
SELECT *
FROM read_parquet('/path/to/file.parquet');

-- 查询 MySQL 数据库
SELECT *
FROM read_mysql('users', conn => 'mysql://user:password@localhost:3306/mydb')
WHERE "age" > 30;

-- 查询 PostgreSQL 数据库
SELECT *
FROM read_postgres('users', host => 'localhost', username => 'postgres', db => 'mydb', pass => 'password')
WHERE "age" > 30;

-- 多源联合查询（Excel + MySQL）
SELECT *
FROM read_excel('/path/to/file.xlsx', sheet_name => 'Sheet1') AS t1
INNER JOIN
read_mysql('users', conn => 'mysql://user:password@localhost:3306/mydb') AS t2
ON t1."user_id" = t2."id"
WHERE t1."age" > 30;

-- 正则匹配
SELECT *
FROM read_csv('/path/to/file.csv')
WHERE REGEXP_LIKE("Distance", '^([0-9]+)\.([0-9]+)?$');
```

### 支持的数据源

| 格式       | 函数                           | 说明                                |
| ---------- | ------------------------------ | ----------------------------------- |
| CSV        | `read_csv()`                   | 支持自定义分隔符、表头、Schema 推断 |
| TSV        | `read_tsv()`                   | Tab 分隔文件                        |
| Text       | `read_text()`                  | 通用文本文件，支持自定义分隔符      |
| Excel      | `read_excel()` / `read_xlsx()` | 支持 `.xlsx`，可选工作表            |
| JSON       | `read_json()`                  | 标准 JSON 数组与 NDJSON，自动检测格式 |
| NdJson     | `read_ndjson()`                | 每行一个 JSON 对象                  |
| Parquet    | `read_parquet()`               | 列式存储格式                        |
| MySQL      | `read_mysql()`                 | 直连 MySQL 数据库表                 |
| PostgreSQL | `read_postgres()`              | 直连 PostgreSQL 数据库表            |

### 函数参数说明

<details>
<summary><code>read_csv()</code> 参数</summary>

| 参数             | 类型    | 默认值 | 说明                                  |
| ---------------- | ------- | ------ | ------------------------------------- |
| `infer_schema`   | boolean | true   | 是否自动推断数据类型（基于前 100 行） |
| `has_header`     | boolean | true   | 文件是否包含表头行                    |
| `delimiter`      | string  | `,`    | 字段分隔符，支持转义序列如 `\t`、`\n` |
| `file_extension` | string  | `.csv` | 文件扩展名                            |

</details>

<details>
<summary><code>read_excel()</code> 参数</summary>

| 参数           | 类型    | 默认值       | 说明                 |
| -------------- | ------- | ------------ | -------------------- |
| `sheet_name`   | string  | 第一个 Sheet | 要读取的工作表名称   |
| `infer_schema` | boolean | true         | 是否自动推断数据类型 |

</details>

<details>
<summary><code>read_mysql()</code> 参数</summary>

| 参数   | 类型   | 默认值   | 说明                                                            |
| ------ | ------ | -------- | --------------------------------------------------------------- |
| `conn` | string | **必需** | MySQL 连接字符串，如 `mysql://user:password@host:port/database` |

</details>

<details>
<summary><code>read_postgres()</code> 参数</summary>

| 参数       | 类型   | 默认值    | 说明                  |
| ---------- | ------ | --------- | --------------------- |
| `host`     | string | **必需**  | PostgreSQL 服务器地址 |
| `username` | string | **必需**  | PostgreSQL 用户名     |
| `db`       | string | **必需**  | PostgreSQL 数据库名称 |
| `pass`     | string | 可选      | PostgreSQL 密码       |
| `port`     | string | `5432`    | PostgreSQL 端口号     |
| `sslmode`  | string | `disable` | SSL 模式              |

</details>

<details>
<summary><code>read_json()</code> 参数</summary>

| 参数             | 类型   | 默认值   | 说明                                                                 |
| ---------------- | ------ | -------- | -------------------------------------------------------------------- |
| `file_extension` | string | 路径扩展名 | 文件扩展名，用于覆盖路径中的扩展名（如 NDJSON 内容存储在 `.json` 文件中） |

`read_json()` 会根据文件内容自动判断格式：以 `[` 开头解析为标准 JSON 数组，以 `{` 开头解析为 NDJSON。`.json` 与 `.ndjson` 文件均可使用。

</details>

<details>
<summary><code>read_ndjson()</code> 参数</summary>

| 参数             | 类型   | 默认值   | 说明         |
| ---------------- | ------ | -------- | ------------ |
| `file_extension` | string | `.ndjson` | 文件扩展名   |

</details>

<details>
<summary><code>read_text()</code> 参数</summary>

| 参数             | 类型    | 默认值 | 说明                 |
| ---------------- | ------- | ------ | -------------------- |
| `infer_schema`   | boolean | true   | 是否自动推断数据类型 |
| `has_header`     | boolean | true   | 文件是否包含表头行   |
| `delimiter`      | string  | `\t`   | 字段分隔符           |
| `file_extension` | string  | `.txt` | 文件扩展名           |

</details>

## 快速开始

### 系统要求

- **macOS**: 10.15+ (Catalina 或更高版本)
- **Windows**: Windows 10 或更高版本
- **内存**: 建议 4 GB 以上
- **存储**: 至少 100 MB 可用空间

### 安装

1. 访问 [Releases](https://github.com/shencangsheng/easydb_app/releases) 页面，下载适合您系统的安装包
2. **macOS**: 下载 `.dmg` 文件，拖拽到应用程序文件夹
3. **Windows**: 下载 `.exe` 文件，运行安装程序

## 常见问题

### macOS 提示"应用已损坏，无法打开"

这是 macOS Gatekeeper 安全机制阻止未签名应用所致。请在终端执行：

```bash
xattr -r -d com.apple.quarantine /Applications/EasyDB.app
```

如果上述方法无效，可在 **系统偏好设置 > 安全性与隐私 > 通用** 中点击"仍要打开"。

### SQL 语法注意事项

字段名推荐使用双引号包裹：

```sql
SELECT "id", "name" FROM table WHERE "id" = 1;
```

也可使用反引号：

```sql
SELECT `id`, `name` FROM table WHERE `id` = 1;
```

WHERE 子句中的字符串值使用单引号：

```sql
SELECT * FROM table WHERE "id" = '1';
```

### 非标准文件扩展名

对于非 CSV、XLSX、JSON、Parquet 后缀的文件，EasyDB 会自动使用 `read_text()` 函数查询。

## 项目背景

### 从 Server 到 App

[EasyDB Server](https://github.com/shencangsheng/easy_db) 主要部署于 Linux 服务器，作为 Web 服务支持大规模文本文件查询。尽管提供了 Docker 部署方案，但在 macOS 和 Windows 上的本地使用体验不够便捷。

EasyDB App 专为 macOS 和 Windows 优化，改善个人用户的本地查询体验。

### 项目命名

- **EasyDB Server** — 服务器端版本，基于 DataFusion
- **EasyDB App** — 桌面客户端版本，基于 DataFusion（v2.0+）

## 贡献指南

欢迎各种形式的贡献！

1. **Fork** 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 **Pull Request**

### 开发环境

```bash
# 克隆仓库
git clone https://github.com/shencangsheng/easydb_app.git
cd easydb_app

# 安装前端依赖
yarn

# 启动开发服务器
cargo tauri dev

# 构建应用
cargo tauri build
```

### 开发前置条件

- Rust 1.89+
- Node.js 18+
- Yarn

## 许可证

[MIT License](LICENSE) © Cangsheng Shen

## 作者

**Cangsheng Shen**

- GitHub: [@shencangsheng](https://github.com/shencangsheng)
- Email: shencangsheng@126.com

## 致谢

感谢以下开源项目：

- [Apache DataFusion](https://github.com/apache/datafusion) — 高性能 SQL 查询引擎
- [datafusion-table-providers](https://github.com/apache/arrow-datafusion-table-providers) — DataFusion 扩展
- [Tauri](https://tauri.app/) — 现代桌面应用框架
- [React](https://reactjs.org/) — 用户界面库
- [HeroUI](https://heroui.com/) — UI 组件库
- [calamine](https://github.com/calamine/calamine) — Excel 文件解析
- [Ace Editor](https://ace.c9.io/) — 代码编辑器

### 贡献者

<a href="https://github.com/shencangsheng/easydb_app/contributors">
  <img src="https://contrib.rocks/image?repo=shencangsheng/easydb_app" />
</a>

## 联系我们

- 问题反馈: [GitHub Issues](https://github.com/shencangsheng/easydb_app/issues)
- 讨论交流: [GitHub Discussions](https://github.com/shencangsheng/easydb_app/discussions)
- 邮件联系: shencangsheng@126.com

---

<div align="center">

**如果这个项目对您有帮助，请给我们一个 Star**

Made with ❤️ by [Cangsheng Shen](https://github.com/shencangsheng)

</div>
