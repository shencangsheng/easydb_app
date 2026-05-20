# 更新日志

v2.8.0 - 2026-05-20

- 新增 read_postgres() 函数，支持 PostgreSQL 数据库查询
- 新增 SQL 导出列配置，支持修改导出字段名、移除不需要的列
- 新增 BOOL 类型支持，布尔值导出为 true/false（不被引号包裹）
- 新增「空文本输出为 NULL」选项，勾选后 TEXT 类型空字符串将输出 NULL
- 新增关于页面反馈入口，可直接跳转 GitHub Issues
- 优化 SQL 导出类型推断，支持更多 SQL 类型关键字（BIGINT、SMALLINT、DECIMAL、NUMERIC、REAL、CHAR、VARCHAR 等）
- 优化 SQL 类型解析性能，使用预解析枚举替代热循环中的重复字符串分配
- 修复 INT/FLOAT 列中非数值数据导出时生成无效 SQL 的问题，现输出 NULL
- 修复无符号大整数（UInt64）导出时精度丢失的问题
- 重构 SQL 导出配置为 ExportColumnConfig 结构，支持列重命名、列移除与类型映射

v2.7.0 - 2026-05-18

- 新增 SQL 导出时列类型配置，支持为每列选择 INT/DOUBLE/TEXT 目标类型，数值类型不被引号包裹，文本类型始终被引号包裹
- 新增编辑器快捷键支持：⌘Enter/F5 执行查询、⌘K 格式化 SQL
- 优化 Excel 数值类型推断，整数值不再被误判为 Float64 类型
- 重构 SQL 导出对话框为左右双栏布局，左侧为基本设置，右侧为列类型配置面板

v2.6.0 - 2026-02-27

- 优化查询结果分页加载，每页加载 200 行
- 优化列名补全时自动使用双引号包裹，避免 SQL 语法错误
- 优化数据列未填满时，右侧会出现空白区域的问题

v2.5.0 - 2026-01-06

- 优化非 csv、xlsx、json、parquet 文件名后缀时，自动使用 read_text() 函数查询
- 优化补全提示，会已出现在列表中 header 名称作为补全提示
- 默认查询行从 200 行改为 1000 行

v2.4.1 - 2025-12-18

- 修复 read_csv() 和 read_tsv() 函数中 delimiter 参数支持转义序列（如 \t、\n、\r、\\）的问题

v2.4.0 - 2025-12-05

- 修复数据表格渲染卡顿的问题
- 修复在文件无数据时返回错误而非显示空表格的问题
- 修复 `read_ndjson()` 函数被误写为 `read_dnjson()` 的问题

v2.3.0 - 2025-11-26

- 修复无法嵌套查询的问题
- 优化 read_excel() 不再默认使用 Sheet1 作为默认工作表，而是选择第一个工作表

v2.2.0 - 2025-11-25

- 新增 read_csv() 和 read_tsv() 函数的 has_header、delimiter 和 file_extension 参数
- 新增 SQL 语句导出时可以选择数据库方言，支持 MySQL 和 PostgreSQL
- 优化 SQL 大文件导出性能

v2.1.2 - 2025-11-25

- 修复 read_tsv() 扩展名要求是 .csv 的问题

v2.1.1 - 2025-11-24

- 修复 SQL 语句导出时表名输入自动补全导致表名首字母自动大写的问题
- 修复 read_excel() 使用 sheet_name 与 infer_schema 参数时，只能生效其一的问题

v2.1.0 - 2025-11-20

- 新增 read_mysql() 函数，支持 MySQL 数据库查询
- DataFusion 版本升级为 50.3.0
- SQL 智能补全功能优化

v2.0.1 - 2025-11-18

- 修正示例文档中的错误

v2.0.0 - 2025-11-15

- 查询引擎切换为 DataFusion
- 暂时移除 read_json() 函数
- 支持仅执行选中的 SQL，解决无法片段执行的问题
- 拖拽文件不再是直接生成完整 SQL，而是可以选择是添加文件路径还是完整 SQL，方便多表联查时获取文件路径

v1.0.0 - 2025-10-30

- 拖拽文件自动生成 SQL 语句
- 支持 CSV、Excel、NdJson、JSON、Parquet 文件格式
- 支持 SQL 历史记录
- 支持查询结果导出（CSV、TSV、SQL）
