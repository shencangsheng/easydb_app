# Changelog

v2.13.0 - 2026-06-24

- Excel large-file performance: first query converts (auto or `build_index => true`) to Parquet cache under AppData; subsequent queries use `register_parquet` for faster paging and repeat access
- Added `build_index => true/false` parameter to `read_excel` / `read_xlsx` for explicit cache control
- Settings: new Excel section with auto-index toggle, threshold (1/5/10/20/50 MB), cache usage display, and one-click clear
- Improved Excel type inference: boolean columns (TRUE/FALSE, 0/1, column-wide bool-like detection), Error cell string fallback
- Added `excel_cache` module with regression tests

v2.12.0 - 2026-06-22

- Added multi-select WHERE columns for SQL UPDATE export; generated statements join multiple conditions with AND
- Selected WHERE columns are automatically excluded from the SET clause to avoid redundant updates
- Added validation and error messages for duplicate or empty WHERE columns

v2.11.0 - 2026-06-16

- Added "Copy SQL Statement" in the SQL export dialog to copy generated INSERT/UPDATE statements directly to the clipboard
- Export and copy operations now show in-progress, success, and failure status toasts, with improved loading indicators and error feedback
- SQL copy is automatically truncated at 10,000 rows, with an amber warning toast when truncation occurs
- Optimized SQL generation performance using RecordBatch batch processing to reduce memory usage
- Restored read_json() function, supporting standard JSON array format queries
- read_json() auto-detects JSON arrays vs NDJSON (content sniffing with extension fallback when sniffing is unavailable)
- Editor and drag-and-drop templates now uniformly use read_json() for JSON/NDJSON files
- Fixed JSON arrays with a UTF-8 BOM being misclassified as NDJSON
- Optimized Excel date-time parsing to support space-separated date-time strings
- DataFusion upgraded to 53.1.0

v2.10.0 - 2026-06-11

- Added saved SQL queries: name and save queries, search and filter the list, delete entries, and load saved queries quickly from the left sidebar
- Query history supports keyword search and configurable display limits (50 / 100 / 200 / 500 / all)
- Query history supports cleanup by age (older than 7, 30, or 90 days, or clear all)
- Improved query history table with index, time, status, and SQL column headers, plus better loading and search interactions
- Automatically switch to the results tab when a query runs, and show query duration at the bottom
- Improved editor synchronization with external SQL state, preventing stale content after switching history or loading saved queries
- Fixed false "file not found" errors when registering remote file paths (HTTP / S3 URLs, etc.)

v2.9.0 - 2026-05-29

- Show Arrow column types in query result headers for better data structure recognition
- Query interface (fetch / fetch_page) returns column metadata (column name, Arrow type, default SQL type)
- Optimized SQL export column type configuration, directly reusing column types from query results, reducing one backend request
- Fixed read_excel() date time column parsing error (previously mistaking Excel serial numbers as date strings)
- Optimized Excel date time conversion, supporting DateTimeIso format strings

v2.8.0 - 2026-05-20

- Added read_postgres() function, supporting PostgreSQL database queries
- Added SQL export column configuration, supporting renaming export column names and removing unnecessary columns
- Added BOOL type support, boolean values are exported as true/false (not wrapped in quotes)
- Added "Export empty text as NULL" option, when enabled empty TEXT strings will be exported as NULL
- Added feedback link in About page, redirecting to GitHub Issues
- Optimized SQL export type inference, supporting more SQL type keywords (BIGINT, SMALLINT, DECIMAL, NUMERIC, REAL, CHAR, VARCHAR, etc.)
- Optimized SQL type parsing performance, using pre-parsed enum instead of repeated string allocations in the hot loop
- Fixed the issue where non-numeric data in INT/FLOAT columns generates invalid SQL during export, now outputs NULL
- Fixed the issue of precision loss when exporting unsigned large integers (UInt64)
- Refactored SQL export configuration into ExportColumnConfig structure, supporting column renaming, column removal, and type mapping

v2.7.0 - 2026-05-18

- Added column type configuration for SQL export, supporting INT/DOUBLE/TEXT target types per column; numeric types are not wrapped in quotes, text types are always wrapped in quotes
- Added editor keyboard shortcuts: ⌘Enter/F5 to execute query, ⌘K to format SQL
- Optimized Excel numeric type inference, integer values are no longer misidentified as Float64 type
- Refactored SQL export dialog into a left-right dual-panel layout, with basic settings on the left and column type configuration panel on the right

v2.6.0 - 2026-02-27

- Optimized query result paging loading, loading 200 rows per page
- Optimized column name completion to automatically use double quotes to wrap, avoiding SQL syntax errors
- Optimized the problem that the right side will appear blank when the data column is not filled

v2.5.0 - 2026-01-06

- Optimized automatic use of read_text() function when dragging non-csv, xlsx, json, parquet file suffixes
- Optimized completion prompts, will use header names already in the list as completion prompts
- The default number of query rows has been changed from 200 to 1000

v2.4.1 - 2025-12-18

- Fixed the problem that the delimiter parameter in read_csv() and read_tsv() functions supports escape sequences (e.g., \t, \n, \r, \\)

v2.4.0 - 2025-12-05

- Fixed the problem that the data table rendering is stuck
- Fixed the problem that when the file has no data, it returns an error instead of displaying an empty table
- Fixed the problem that `read_ndjson()` function is mistakenly written as `read_dnjson()`

v2.3.0 - 2025-11-26

- Fixed the problem that nested queries cannot be executed
- Optimized read_excel() to select the first sheet instead of Sheet1 as the default sheet

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
