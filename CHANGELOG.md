# 更新日志

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
