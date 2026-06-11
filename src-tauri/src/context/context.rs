use crate::commands::query::{arrow_type_to_sql_type, ColumnTypeInfo};
use crate::context::error::AppError;
use crate::context::schema::AppResult;
use crate::reader::excel::ExcelReader;
use crate::utils::file_utils::ensure_path_exists;
use crate::sql::parse::{get_function_args, parse_statements};
use async_recursion::async_recursion;
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::dataframe::DataFrame;
use datafusion::prelude::{CsvReadOptions, NdJsonReadOptions, ParquetReadOptions, SessionContext};
use datafusion::sql::TableReference;
use datafusion_table_providers::{
    mysql::MySQLTableFactory,
    postgres::PostgresTableFactory,
    sql::db_connection_pool::{
        mysqlpool::MySQLConnectionPool, postgrespool::PostgresConnectionPool,
    },
    util::secrets::to_secret_map,
};
use sqlparser::ast::SetExpr::Select;
use sqlparser::ast::{
    Expr, FunctionArg, FunctionArgExpr, Offset, OffsetRows, Query, Statement, TableFactor,
    TableFunctionArgs, Value,
};
use std::collections::HashMap;
use std::sync::Arc;

pub fn get_sql_context() -> SessionContext {
    SessionContext::new()
}

pub async fn get_data_frame(ctx: &mut SessionContext, sql: &String) -> AppResult<DataFrame> {
    ctx.sql(sql).await.map_err(AppError::from)
}

pub async fn collect(
    ctx: &mut SessionContext,
    sql: &String,
) -> AppResult<(Vec<ColumnTypeInfo>, Vec<RecordBatch>)> {
    let df = get_data_frame(ctx, sql).await?;

    // Get column metadata (name + type) from DataFrame's schema so the frontend can
    // reuse the type information (e.g. for SQL export) without a second round-trip.
    // Column information can be retrieved even if there's no data.
    let columns: Vec<ColumnTypeInfo> = df
        .schema()
        .fields()
        .iter()
        .map(|f| ColumnTypeInfo {
            column_name: f.name().to_string(),
            arrow_type: f.data_type().to_string(),
            default_sql_type: arrow_type_to_sql_type(f.data_type()),
        })
        .collect();

    let records = df.collect().await.map_err(AppError::from)?;

    Ok((columns, records))
}

/// Parse delimiter string to u8 byte value
/// Supports escape sequences like \t, \n, \r, \\, and single characters
fn parse_delimiter(value: &str) -> AppResult<u8> {
    // Handle escape sequences (e.g., "\t" -> tab character)
    if value.starts_with('\\') && value.len() == 2 {
        match value.chars().nth(1) {
            Some('t') => return Ok(b'\t'),
            Some('n') => return Ok(b'\n'),
            Some('r') => return Ok(b'\r'),
            Some('\\') => return Ok(b'\\'),
            Some('0') => return Ok(b'\0'),
            _ => {}
        }
    }

    // Handle single character (including space) - directly convert to u8
    if value.len() == 1 {
        return Ok(value.as_bytes()[0]);
    }

    // Try parsing as numeric string (e.g., "9", "65")
    if let Ok(num) = value.parse::<u8>() {
        return Ok(num);
    }

    Err(AppError::BadRequest {
        message: format!("Invalid delimiter format: '{}'. Expected a single character, escape sequence (\\t, \\n, \\r), or numeric value (0-255)", value),
    })
}

pub fn get_csv_read_options<'a>(
    args: &'a mut Option<TableFunctionArgs>,
    mut options: CsvReadOptions<'a>,
) -> AppResult<CsvReadOptions<'a>> {
    let args = get_function_args(args);
    if let Some(args) = args {
        for arg in args {
            if let FunctionArg::Named { name, arg, .. } = arg {
                match name.value.as_str() {
                    "infer_schema" => {
                        if let FunctionArgExpr::Expr(Expr::Value(Value::Boolean(value))) = arg {
                            if !value {
                                options.schema_infer_max_records = 0;
                            }
                        }
                    }
                    "has_header" => {
                        if let FunctionArgExpr::Expr(Expr::Value(Value::Boolean(value))) = arg {
                            if !value {
                                options.has_header = *value;
                            }
                        }
                    }
                    "delimiter" => {
                        if let FunctionArgExpr::Expr(Expr::Value(Value::SingleQuotedString(
                            value,
                        ))) = arg
                        {
                            options.delimiter = parse_delimiter(&value)?;
                        }
                    }
                    "file_extension" => {
                        if let FunctionArgExpr::Expr(Expr::Value(Value::SingleQuotedString(
                            value,
                        ))) = arg
                        {
                            options.file_extension = value;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(options)
}

pub fn get_table_path(args: &mut Option<TableFunctionArgs>) -> AppResult<String> {
    if args.is_none() {
        return Err(AppError::BadRequest {
            message: "The file path is missing.".to_string(),
        });
    }

    let value = &args
        .as_ref()
        .unwrap()
        .args
        .get(0)
        .ok_or(AppError::BadRequest {
            message: "The file path is missing. 2".to_string(),
        })?;

    match value {
        FunctionArg::Unnamed(FunctionArgExpr::Expr(Expr::Value(Value::SingleQuotedString(
            value,
        )))) => Ok(value.to_string()),
        _ => Err(AppError::BadRequest {
            message: "The file path is missing. 3".to_string(),
        }),
    }
}

pub fn get_path(args: &mut Option<TableFunctionArgs>) -> AppResult<String> {
    if args.is_none() {
        return Err(AppError::BadRequest {
            message: "The file path is missing.".to_string(),
        });
    }

    let value = &args
        .as_ref()
        .unwrap()
        .args
        .get(0)
        .ok_or(AppError::BadRequest {
            message: "The file path is missing. 2".to_string(),
        })?;

    match value {
        FunctionArg::Unnamed(FunctionArgExpr::Expr(Expr::Value(Value::SingleQuotedString(
            value,
        )))) => Ok(value.to_string()),
        _ => Err(AppError::BadRequest {
            message: "The file path is missing. 3".to_string(),
        }),
    }
}

pub fn read_excel(
    mut reader: ExcelReader,
    args: &mut Option<TableFunctionArgs>,
) -> AppResult<RecordBatch> {
    let args = get_function_args(args);

    if let Some(args) = args {
        for arg in args {
            if let FunctionArg::Named { name, arg, .. } = arg {
                match name.value.as_str() {
                    "sheet_name" => {
                        if let FunctionArgExpr::Expr(Expr::Value(Value::SingleQuotedString(
                            value,
                        ))) = arg
                        {
                            reader = reader.with_sheet_name(value.to_string());
                        }
                    }
                    "infer_schema" => {
                        if let FunctionArgExpr::Expr(Expr::Value(Value::Boolean(value))) = arg {
                            if !value {
                                reader = reader.with_infer_schema_length(0);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    reader.finish().map_err(|e| e.into())
}

pub async fn register_mysql(
    ctx: &mut SessionContext,
    table_name: &String,
    table_path: &String,
    args: &mut Option<TableFunctionArgs>,
) -> AppResult<()> {
    let args = get_function_args(args);
    let mut conn: Option<String> = None;

    if let Some(args) = args {
        for arg in args {
            if let FunctionArg::Named { name, arg, .. } = arg {
                match name.value.as_str() {
                    "conn" => {
                        if let FunctionArgExpr::Expr(Expr::Value(Value::SingleQuotedString(
                            value,
                        ))) = arg
                        {
                            conn = Some(value.to_string());
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    if conn.is_none() {
        return Err(AppError::BadRequest {
            message: "'conn' parameter is required".to_string(),
        });
    }

    let mysql_params = to_secret_map(HashMap::from([
        ("connection_string".to_string(), conn.unwrap()),
        ("sslmode".to_string(), "disabled".to_string()),
    ]));

    // Create MySQL connection pool
    let mysql_pool = Arc::new(MySQLConnectionPool::new(mysql_params).await?);

    // Create MySQL table provider factory
    // Used to generate TableProvider instances that can read MySQL table data
    let table_factory = MySQLTableFactory::new(mysql_pool);

    ctx.register_table(
        table_name,
        table_factory
            .table_provider(TableReference::bare(table_path.clone()))
            .await?,
    )?;

    Ok(())
}

pub async fn register_postgres(
    ctx: &mut SessionContext,
    table_name: &String,
    table_path: &String,
    args: &mut Option<TableFunctionArgs>,
) -> AppResult<()> {
    let args = get_function_args(args);
    let mut host: Option<String> = None;
    let mut username: Option<String> = None;
    let mut db: Option<String> = None;
    let mut pass: Option<String> = None;
    let mut port: Option<String> = None;
    let mut sslmode: Option<String> = None;

    if let Some(args) = args {
        for arg in args {
            if let FunctionArg::Named { name, arg, .. } = arg {
                if let FunctionArgExpr::Expr(Expr::Value(Value::SingleQuotedString(value))) = arg {
                    match name.value.as_str() {
                        "host" => host = Some(value.to_string()),
                        "username" => username = Some(value.to_string()),
                        "db" => db = Some(value.to_string()),
                        "pass" => pass = Some(value.to_string()),
                        "port" => port = Some(value.to_string()),
                        "sslmode" => sslmode = Some(value.to_string()),
                        _ => {}
                    }
                }
            }
        }
    }

    if host.is_none() {
        return Err(AppError::BadRequest {
            message: "'host' parameter is required".to_string(),
        });
    }
    if username.is_none() {
        return Err(AppError::BadRequest {
            message: "'username' parameter is required".to_string(),
        });
    }
    if db.is_none() {
        return Err(AppError::BadRequest {
            message: "'db' parameter is required".to_string(),
        });
    }

    let mut params = HashMap::from([
        ("host".to_string(), host.unwrap()),
        ("user".to_string(), username.unwrap()),
        ("db".to_string(), db.unwrap()),
    ]);

    if let Some(pass) = pass {
        params.insert("pass".to_string(), pass);
    }
    params.insert(
        "port".to_string(),
        port.unwrap_or_else(|| "5432".to_string()),
    );
    params.insert(
        "sslmode".to_string(),
        sslmode.unwrap_or_else(|| "disable".to_string()),
    );

    let postgres_params = to_secret_map(params);

    let postgres_pool = Arc::new(PostgresConnectionPool::new(postgres_params).await?);

    let table_factory = PostgresTableFactory::new(postgres_pool);

    ctx.register_table(
        table_name,
        table_factory
            .table_provider(TableReference::bare(table_path.clone()))
            .await?,
    )?;

    Ok(())
}

pub async fn register_table(
    ctx: &mut SessionContext,
    relation: &mut TableFactor,
    table_count: i32,
) -> AppResult<i32> {
    if let TableFactor::Table { name, args, .. } = relation {
        let table_name = format!("table{}", table_count);
        let table_path = get_table_path(args)?;
        let reader_name = name.to_string();

        if matches!(
            reader_name.as_str(),
            "read_csv"
                | "read_tsv"
                | "read_ndjson"
                | "read_parquet"
                | "read_text"
                | "read_excel"
                | "read_xlsx"
        ) {
            ensure_path_exists(&table_path)?;
        }

        match reader_name.as_str() {
            "read_csv" => {
                ctx.register_csv(
                    &table_name,
                    &table_path,
                    get_csv_read_options(args, CsvReadOptions::default())?,
                )
                .await?
            }
            "read_tsv" => {
                let mut options = CsvReadOptions::default();
                options.delimiter = b'\t';
                options.file_extension = ".tsv";
                ctx.register_csv(
                    &table_name,
                    &table_path,
                    get_csv_read_options(args, options)?,
                )
                .await?
            }
            "read_ndjson" => {
                ctx.register_json(&table_name, &table_path, NdJsonReadOptions::default())
                    .await?
            }
            "read_parquet" => {
                ctx.register_parquet(&table_name, &table_path, ParquetReadOptions::default())
                    .await?
            }
            "read_excel" | "read_xlsx" => {
                ctx.register_batch(&table_name, read_excel(ExcelReader::new(table_path), args)?)?;
            }
            "read_mysql" => {
                register_mysql(ctx, &table_name, &table_path, args).await?;
            }
            "read_postgres" => {
                register_postgres(ctx, &table_name, &table_path, args).await?;
            }
            "read_text" => {
                let mut options = CsvReadOptions::default();
                options.delimiter = b'\t';
                options.file_extension = ".txt";
                ctx.register_csv(
                    &table_name,
                    &table_path,
                    get_csv_read_options(args, options)?,
                )
                .await?
            }
            _ => {
                return Err(AppError::BadRequest {
                    message: format!("'{}' is not a supported table function", table_name),
                })
            }
        }

        *name = sqlparser::ast::ObjectName(vec![table_name.as_str().into()]);
        *args = None;
    }

    Ok(table_count + 1)
}

#[async_recursion]
pub async fn convert_table_name(
    ctx: &mut SessionContext,
    query: &mut Box<Query>,
    mut table_count: i32,
) -> AppResult<i32> {
    if let Select(select) = &mut *query.body {
        for table_with_joins in &mut select.from {
            match &mut table_with_joins.relation {
                TableFactor::Derived { subquery, .. } => {
                    table_count = convert_table_name(ctx, subquery, table_count).await?;
                }
                relation => {
                    table_count = register_table(ctx, relation, table_count).await?;
                }
            }
            for join in &mut table_with_joins.joins {
                match &mut join.relation {
                    TableFactor::Derived { subquery, .. } => {
                        table_count = convert_table_name(ctx, subquery, table_count).await?;
                    }
                    relation => {
                        table_count = register_table(ctx, &mut join.relation, table_count).await?;
                    }
                }
            }
        }
    }

    Ok(table_count)
}

pub async fn register(
    ctx: &mut SessionContext,
    sql: &str,
    limit: Option<usize>,
    offset: Option<usize>,
) -> AppResult<String> {
    let mut ast = parse_statements(sql)?;

    let statement = ast.get_mut(0).ok_or(AppError::BadRequest {
        message: "invalid SQL statement".to_string(),
    })?;

    if let Statement::Query(query) = statement {
        convert_table_name(ctx, query, 0).await?;
        if limit.is_some() && query.limit.is_none() {
            query.limit = Some(Expr::Value(Value::Number(limit.unwrap().to_string(), true)));
        }
        if offset.is_some()
            && query.limit.is_some()
            && query.offset.is_none()
            && offset.unwrap() > 0
        {
            if let Some(Expr::Value(Value::Number(value, _))) = &query.limit {
                query.offset = Some(Offset {
                    value: Expr::Value(Value::Number(
                        (value.parse::<i64>().unwrap() * offset.unwrap() as i64).to_string(),
                        true,
                    )),
                    rows: OffsetRows::None,
                });
            }
        }

        Ok(query.to_string())
    } else {
        Err(AppError::BadRequest {
            message: "Only supports Select statements.".to_string(),
        })
    }
}
