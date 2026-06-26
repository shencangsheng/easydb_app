# EasyDB App 产品与技术路线图

> 文档版本：2026-06-24  
> 基于当前代码库 v2.12.0 整理，随版本迭代更新。

## 1. 背景与定位

EasyDB App 是一款基于 **Rust + Tauri + Apache DataFusion** 的桌面 SQL 查询工具，核心能力是：**用 SQL 直接查询本地文件与数据库，无需安装独立数据库服务**。

与 [EasyDB Server](https://github.com/shencangsheng/easy_db) 的关系：

| 产品 | 场景 | 优势 |
|------|------|------|
| EasyDB Server | Linux 服务器、Web 服务、大规模文本文件 | 远程部署、集中查询 |
| EasyDB App | macOS / Windows 个人本地 | 开箱即用、文件拖拽、SQL 导出 |

App 不应重复 Server 的全部能力，而应聚焦 **个人本地数据分析工作流**。

---

## 2. 当前状态（v2.12.0）

### 2.1 已成熟能力

- **查询引擎**：DataFusion 53.1，支持 JOIN、子查询、窗口函数、片段执行
- **文件格式**：CSV / TSV / Text / JSON / NDJSON / Parquet 走 DataFusion 原生 `register_*`，流式友好
- **编辑器**：语法高亮、补全、格式化、拖拽生成 SQL
- **工作流**：查询历史（搜索、清理、条数配置）、保存查询、结果分页、列类型展示
- **导出**：CSV / TSV / SQL（INSERT / UPDATE，MySQL / PostgreSQL 方言，多 WHERE 字段）

### 2.2 已知短板

| 领域 | 现状 | 代码位置 |
|------|------|----------|
| Excel | Beta；整表读入内存后 `register_batch` | `src-tauri/src/reader/excel.rs`、`context.rs` 中 `read_excel` |
| 数据库连接 | Beta；连接串/密码写死在 SQL 中 | `context.rs` 中 `register_mysql` / `register_postgres` |
| 文件发现 | 依赖拖拽与手写绝对路径 | 左侧 Database 按钮暂无浏览能力 |
| 会话模型 | 单 notebook，SQL 存 localStorage | `src/pages/notebook/index.tsx` |
| 远程文件 | 后端已跳过 HTTP/S3 本地存在性检查，无用户向配置 | `src-tauri/src/utils/file_utils.rs` |

### 2.3 近期版本节奏（参考 CHANGELOG）

- **v2.10**：保存查询、历史管理、编辑器状态同步
- **v2.11**：SQL 复制、RecordBatch 批处理、JSON 格式嗅探、DataFusion 升级
- **v2.12**：UPDATE 多 WHERE 字段、校验与错误提示

趋势：**核心查询已稳定，迭代重心转向工作流打磨与 Beta 功能成熟化**。

---

## 3. 优先级总览

```
P0  Excel 性能与类型完善          → v2.13
P1  连接配置 + 目录浏览            → v2.14
P2  多会话 / 多标签                → v2.15
P3  远程数据源（S3 / HTTP）        → v2.16+
P4  数据可视化                     → 待用户反馈驱动
```

---

## 4. v2.13 — Excel 从 Beta 到生产可用

### 4.1 目标

- 大 Excel 文件可查询，内存与启动时间可接受
- 常见单元格类型解析正确，减少「全是字符串」或日期错误
- 移除或降低 UI 中 Excel 的 Beta 标记条件

### 4.2 问题根因

当前实现与 CSV/Parquet 路径不一致：

```rust
// src-tauri/src/context/context.rs
"read_excel" | "read_xlsx" => {
    ctx.register_batch(&table_name, read_excel(ExcelReader::new(table_path), args)?)?;
}
```

`ExcelReader::finish()` 使用 calamine 读取整个 worksheet，逐行构建 `RecordBatch`，无法利用 DataFusion 的下推过滤（`WHERE` / `LIMIT`）。

### 4.3 实施计划

#### 阶段 A：短期改进（可独立发布）

- [ ] **大文件提示**：sheet 行数/列数超过阈值时，执行前给出警告（建议加 `LIMIT`）
- [ ] **Schema 预览**：`infer_schema => false` 时仅读表头 + 前 N 行推断类型
- [ ] **类型补齐**：
  - [x] 布尔值（TRUE/FALSE、0/1、整列 bool-like 检测）
  - [x] 公式/错误单元格（calamine 返回计算值或 Error 字符串 fallback）
  - [x] 空单元格与空字符串区分策略（Empty → NULL，空字符串 → ""）
  - 日期/时间边界（已有 `excel_cell_to_timestamp_nanos`，补充回归测试）
- [x] **测试**：在 `src-tauri/src/reader/excel_test.rs` 增加混合类型、布尔、空值场景；`excel_cache_test.rs` 覆盖缓存 hit/miss/invalidation

#### 阶段 B：中期架构（性能核心）

- [x] **B2 临时 Parquet**：首次查询转写 AppData 下临时 Parquet，后续 `register_parquet`（v2.13）
  - SQL 参数 `build_index => true/false` 显式控制
  - 设置页：大文件自动索引阈值、缓存占用展示与一键清空
  - 缓存目录：`{AppData}/data/excel_cache/`

可选方案（benchmark 参考）：

| 方案 | 思路 | 优点 | 风险 |
|------|------|------|------|
| B1 分块 RecordBatch | 按行 chunk 注册为 `MemTable` 或自定义 `TableProvider` | 改动集中在 reader | 仍难下推 LIMIT |
| B2 临时 Parquet | 首次查询转写 AppData 下临时 Parquet，后续 `register_parquet` | 复用 DataFusion 流式能力 | 磁盘占用、首次延迟 |
| B3 导出为 CSV 缓存 | 类似 B2，实现更简单 | 落地快 | 类型可能丢失 |

**推荐路径（v2.13 起）**：B2 作为 Excel 大文件默认优化路径；B1 可作为后续补充。

### 4.4 验收标准

- 10 万行 × 20 列 xlsx：首次查询 P95 < 10s（M 系列 Mac 或同级 Windows）
- 100 万行：不 OOM（8 GB 内存机器），或明确报错并引导用户转 Parquet
- `excel_test.rs` 覆盖率包含日期、整型、浮点、空值、布尔

---

## 5. v2.14 — 连接配置与目录浏览

### 5.1 数据库连接管理

#### 目标

用户无需在 SQL 中重复粘贴密码；`read_mysql` / `read_postgres` 可引用命名连接。

#### 数据模型（SQLite，扩展现有 `db_utils.rs`）

```sql
CREATE TABLE IF NOT EXISTS connections (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  db_type TEXT NOT NULL,        -- 'mysql' | 'postgres'
  config_json TEXT NOT NULL,    -- 非敏感字段
  created_at TIMESTAMP,
  updated_at TIMESTAMP
);
```

- 密码/密钥：**不存明文**于 SQLite，使用系统 keychain（macOS Keychain / Windows Credential Manager）或 Tauri 安全存储插件
- 前端：设置页或左侧栏新增「连接管理」面板

#### SQL 语法扩展（示例）

```sql
-- 方案 A：命名连接参数
SELECT * FROM read_mysql('users', conn => '@prod_mysql');

-- 方案 B：全局默认连接（设置中配置）
SELECT * FROM read_mysql('users', conn => '@default:mysql');
```

实现要点：

- [ ] `context.rs` 解析 `conn` 参数时识别 `@name` 前缀，从连接表 + keychain 解析真实连接串
- [ ] 编辑器补全连接名称
- [ ] 连接测试按钮（`SELECT 1`）

### 5.2 目录浏览与最近文件

#### 目标

降低「找路径、记路径」成本；激活左侧 Database 区域。

#### 功能清单

- [ ] 文件/目录树（受 Tauri 权限 scope 约束，复用 `settings-modal` 已有授权流程）
- [ ] 单击文件 → 插入 `read_xxx('/path')` 或完整 SQL 模板（与拖拽行为一致）
- [ ] 最近打开文件列表（SQLite 或 localStorage，建议 SQLite 与历史统一）
- [ ] 常用目录 Pin

#### 涉及文件

- 前端：`src/pages/notebook/notebook-left/` 新增浏览组件
- 后端：可选新增 `list_directory` / `get_recent_files` Tauri command
- 权限：`src/components/common/settings-modal.tsx` 已有文件/目录授权逻辑

### 5.3 验收标准

- 创建 MySQL 连接后，SQL 中不再出现明文密码
- 从目录树点击 CSV，3 步内完成首次查询
- 连接配置 CRUD + 单元测试

---

## 6. v2.15 — 多会话 / 多标签

### 6.1 目标

支持同时打开多个分析上下文（不同 SQL、不同结果集），避免单 `localStorage` 键覆盖。

### 6.2 方案 sketch

- 会话状态：`{ id, title, sql, activeResultTab, ... }[]`
- 持久化：SQLite `sessions` 表或 IndexedDB
- UI：编辑器顶部 Tab 栏，类似 IDE
- 保存查询与会话关联（可选）

### 6.3 风险

- 编辑器（Ace）多实例内存占用
- 与现有 `notebook-sql` localStorage 迁移

建议 v2.14 完成后、根据 Issue 反馈再启动。

---

## 7. v2.16+ — 远程数据源

### 7.1 现状

`file_utils.rs` 已对 `http://`、`https://`、`s3://` 跳过本地存在性检查；v2.10 修复过远程 URL 注册问题。

### 7.2 分步计划

1. **文档与示例**：明确支持 `read_parquet('https://...')`、S3 路径写法及 credential 环境变量
2. **S3 配置 UI**：Access Key / Secret / Region / Endpoint（存 keychain）
3. **EasyDB Server 联动**（长期）：App 作为 Client 连接 Server API，避免在 App 内重复实现远程文件索引

### 7.3 与 Server 的边界

| 能力 | App 自建 | 通过 Server |
|------|----------|-------------|
| 本地文件 | ✅ 核心 | — |
| S3 单文件 URL | ✅ 可行 | ✅ |
| 远程目录索引 | ❌ 非核心 | ✅ 更合适 |
| 大规模并发 | ❌ | ✅ |

---

## 8. 暂缓项

### 8.1 数据可视化

- 图表（折线、柱状、饼图）能扩大受众，但偏离「SQL 查文件」核心定位
- **触发条件**：GitHub Issues / Discussions 中可视化需求 ≥ 5 个独立用户或赞助方明确需求

### 8.2 Linux 桌面版

- 当前发布目标为 macOS / Windows
- Tauri 支持 Linux，但需 CI、打包、签名策略；优先级低于上述 P0–P2

### 8.3 README 路线图条目迁移

README 中「计划中」列表与本文件对齐，Completed 项继续在 CHANGELOG 维护，计划项以 **本文档为准**。

---

## 9. 工程与质量要求

与现有仓库规范保持一致：

| 规范 | 适用 |
|------|------|
| `rust-no-unwrap` skill | 所有新增 Rust 代码 |
| `rust-sql-test` skill | `src-tauri/src/sql/`、`src-tauri/src/reader/` 变更必须补测试 |
| i18n | 新 UI 文案同步 `src/i18n/index.ts`（zh-CN / en-US） |
| CHANGELOG | 每个版本更新 `CHANGELOG.md` / `CHANGELOG_EN.md` |

---

## 10. 反馈与优先级调整

路线图不是固定承诺，按以下信号每季度回顾：

1. [GitHub Issues](https://github.com/shencangsheng/easydb_app/issues) 标签统计
2. [GitHub Discussions](https://github.com/shencangsheng/easydb_app/discussions) 功能投票
3. 下载量 / 版本升级率（如有）
4. Beta 功能（Excel、MySQL、Postgres）相关崩溃与性能报告

---

## 11. 版本里程碑一览

| 版本 | 主题 | 关键交付 |
|------|------|----------|
| **v2.13** | Excel 成熟化 | 分块/缓存读取、类型补齐、大文件提示、测试 |
| **v2.14** | 数据源管理 | 连接配置 + keychain、目录浏览、最近文件 |
| **v2.15** | 多会话 | Tab 式 notebook、会话持久化 |
| **v2.16+** | 远程数据 | S3 配置、HTTP 文档、Server 联动调研 |
| **TBD** | 可视化 | 待需求确认 |

---

## 附录 A：关键代码索引

| 模块 | 路径 |
|------|------|
| 表注册与 read_* 分发 | `src-tauri/src/context/context.rs` |
| Excel 读取 | `src-tauri/src/reader/excel.rs` |
| SQL 导出 | `src-tauri/src/sql/generator.rs` |
| 本地 SQLite | `src-tauri/src/utils/db_utils.rs` |
| 远程路径 | `src-tauri/src/utils/file_utils.rs` |
| Notebook 主页面 | `src/pages/notebook/index.tsx` |
| 函数文档面板 | `src/pages/notebook/notebook-right/notebook-right.tsx` |

## 附录 B：文档维护

- 每完成一个里程碑，更新本文档对应章节勾选状态
- 大方向变更时同步 README「功能与路线图」链接说明
- 英文摘要可后续补充 `docs/ROADMAP_EN.md`（非阻塞）
