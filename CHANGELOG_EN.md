# Changelog

v2.2.0 - 2025-11-25

- Added has_header, delimiter, and file_extension parameters to read_csv() and read_tsv() functions
- Added the ability to select database dialect when exporting SQL statements, supporting MySQL and PostgreSQL
- Optimized the performance of exporting large SQL files

v2.1.2 - 2025-11-25

- Fixed the problem that the extension of read_tsv() is required to be .csv

v2.1.1 - 2025-11-24

- Fixed the problem that the table name input automatically completes the first letter of the table name to uppercase when exporting SQL statements
- Fixed the problem that only one of the sheet_name and infer_schema parameters can take effect when using read_excel()

v2.1.0 - 2025-11-20

- Added read_mysql() function, supporting MySQL database queries
- DataFusion version upgraded to 50.3.0
- SQL intelligent completion function optimization

v2.0.1 - 2025-11-18

- Corrected errors in the example documentation

v2.0.0 - 2025-11-15

- Query engine switched to DataFusion
- Temporarily removed read_json() function
- Support only executing selected SQL, solving the problem of not being able to execute fragments
- Dragging files is no longer directly generating complete SQL, but instead choosing whether to add file paths or complete SQL, making it easier to get file paths when multiple table joins are required

v1.0.0 - 2025-10-30

- Dragging files automatically generates SQL statements
- Supports CSV, Excel, NdJson, JSON, Parquet file formats
- Supports SQL history
- Supports query result export (CSV, TSV, SQL)
