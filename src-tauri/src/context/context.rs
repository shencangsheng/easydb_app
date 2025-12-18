use crate::context::error::AppError;
use crate::context::schema::AppResult;
use crate::reader::excel::ExcelReader;
use crate::sql::parse::{get_function_args, parse_statements};
use async_recursion::async_recursion;
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::dataframe::DataFrame;
use datafusion::prelude::{CsvReadOptions, NdJsonReadOptions, ParquetReadOptions, SessionContext};
use datafusion::sql::TableReference;
use datafusion_table_providers::{
    mysql::MySQLTableFactory, sql::db_connection_pool::mysqlpool::MySQLConnectionPool,
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
) -> AppResult<(Vec<String>, Vec<RecordBatch>)> {
    let df = get_data_frame(ctx, sql).await?;

    // Get header from DataFrame's schema (column information can be retrieved even if there's no data)
    let header: Vec<String> = df
        .schema()
        .fields()
        .iter()
        .map(|f| f.name().to_string())
        .collect();

    let records = df.collect().await.map_err(AppError::from)?;

    Ok((header, records))
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

pub fn get_csv_read_options(
    args: &'_ mut Option<TableFunctionArgs>,
) -> AppResult<CsvReadOptions<'_>> {
    let args = get_function_args(args);
    let mut options = CsvReadOptions::default();
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

pub async fn register_table(
    ctx: &mut SessionContext,
    relation: &mut TableFactor,
    table_count: i32,
) -> AppResult<i32> {
    if let TableFactor::Table { name, args, .. } = relation {
        let table_name = format!("table{}", table_count);
        let table_path = get_table_path(args)?;

        match name.to_string().as_str() {
            "read_csv" => {
                ctx.register_csv(&table_name, &table_path, get_csv_read_options(args)?)
                    .await?
            }
            "read_tsv" => {
                let mut options = get_csv_read_options(args)?;
                options.delimiter = b'\t';
                options.file_extension = ".tsv";
                ctx.register_csv(&table_name, &table_path, options).await?
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
