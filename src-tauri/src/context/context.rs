use crate::context::error::AppError;
use crate::context::schema::AppResult;
use crate::reader::excel::ExcelReader;
use crate::sql::parse::{get_function_args, parse_statements};
use datafusion::prelude::{CsvReadOptions, NdJsonReadOptions, SessionContext};
use polars::frame::DataFrame;
use polars::io::SerReader;
use polars::prelude::{
    IntoLazy, JsonReader, LazyCsvReader, LazyFileListReader, LazyFrame, LazyJsonLineReader,
    ParquetReader, PlPath,
};
use polars::sql::SQLContext;
use sqlparser::ast::SetExpr::{Query, Select};
use sqlparser::ast::{
    Expr, FunctionArg, FunctionArgExpr, Offset, OffsetRows, Statement, TableFactor,
    TableFunctionArgs, Value,
};
use std::fs::File;
use arrow_array::RecordBatch;

pub fn get_sql_context() -> SessionContext {
    SessionContext::new()
}

pub async fn collect(ctx: &mut SessionContext, sql: &String) -> AppResult<Vec<RecordBatch>> {
    ctx.sql(sql).await?.collect().await.map_err(AppError::from)
}

pub fn get_csv_reader(path: PlPath) -> LazyCsvReader {
    LazyCsvReader::new(path)
}
pub fn get_excel_reader(path: PlPath) -> ExcelReader {
    ExcelReader::new(path)
}

pub fn get_json_reader(path: PlPath) -> AppResult<JsonReader<'static, File>> {
    Ok(JsonReader::new(File::open(path.to_str())?))
}

pub fn get_ndjson_reader(path: PlPath) -> LazyJsonLineReader {
    LazyJsonLineReader::new(path)
}

pub fn get_parquet_reader(path: PlPath) -> AppResult<ParquetReader<File>> {
    Ok(ParquetReader::new(File::open(path.to_str())?))
}

pub fn get_csv_read_options(args: &mut Option<TableFunctionArgs>) -> AppResult<CsvReadOptions> {
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
                        break;
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

pub fn get_path(args: &mut Option<TableFunctionArgs>) -> AppResult<PlPath> {
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
        )))) => Ok(PlPath::new(value)),
        _ => Err(AppError::BadRequest {
            message: "The file path is missing. 3".to_string(),
        }),
    }
}

pub fn read_csv(
    mut reader: LazyCsvReader,
    args: &mut Option<TableFunctionArgs>,
) -> AppResult<LazyFrame> {
    let args = get_function_args(args);

    if let Some(args) = args {
        for arg in args {
            if let FunctionArg::Named { name, arg, .. } = arg {
                match name.value.as_str() {
                    "infer_schema" => {
                        if let FunctionArgExpr::Expr(Expr::Value(Value::Boolean(value))) = arg {
                            if !value {
                                reader = reader.with_infer_schema_length(Some(0));
                            }
                        }
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    reader.finish().map_err(|e| e.into())
}

pub fn read_excel(
    mut reader: ExcelReader,
    args: &mut Option<TableFunctionArgs>,
) -> AppResult<DataFrame> {
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
                        break;
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

pub fn read_json(
    mut reader: JsonReader<'static, File>,
    args: &mut Option<TableFunctionArgs>,
) -> AppResult<DataFrame> {
    let args = get_function_args(args);
    reader.finish().map_err(|e| e.into())
}

pub fn read_ndjson(
    mut reader: LazyJsonLineReader,
    args: &mut Option<TableFunctionArgs>,
) -> AppResult<LazyFrame> {
    let args = get_function_args(args);
    reader.finish().map_err(|e| e.into())
}

pub fn read_parquet(
    mut reader: ParquetReader<File>,
    args: &mut Option<TableFunctionArgs>,
) -> AppResult<DataFrame> {
    let _args = get_function_args(args);
    reader.finish().map_err(|e| e.into())
}

enum FrameType {
    Lazy(LazyFrame),
    Data(DataFrame),
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
                ctx.register_csv(&table_name, &table_path, options)
                    .await?
            }
            "read_ndjson" => {
                ctx.register_json(&table_name, &table_path, NdJsonReadOptions::default())
                    .await?
            }
            _ => {
                return Err(AppError::BadRequest {
                    message: format!("'{}' is not a supported table function", table_name),
                })
            }
        }

        // let lazy_frame: Option<FrameType> = match name.to_string().as_str() {
        //     "read_csv" => Some(FrameType::Lazy(read_csv(get_csv_reader(path), args)?)),
        //     "read_tsv" => {
        //         let mut reader = get_csv_reader(path);
        //         reader = reader.with_separator(b'\t');
        //         Some(FrameType::Lazy(read_csv(reader, args)?))
        //     }
        //     "read_ndjson" => Some(FrameType::Lazy(read_ndjson(get_ndjson_reader(path), args)?)),
        //     "read_json" => Some(FrameType::Data(read_json(get_json_reader(path)?, args)?)),
        //     "read_excel" => Some(FrameType::Data(read_excel(get_excel_reader(path), args)?)),
        //     "read_parquet" => Some(FrameType::Data(read_parquet(
        //         get_parquet_reader(path)?,
        //         args,
        //     )?)),
        //     _ => None,
        // };
        //
        // if lazy_frame.is_none() {
        //     return Err(AppError::BadRequest {
        //         message: format!("'{}' is not a supported table function", table_name),
        //     });
        // }

        *name = sqlparser::ast::ObjectName(vec![table_name.as_str().into()]);
        *args = None;
    }

    Ok(table_count + 1)
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

    let mut table_count = 1;

    if let Statement::Query(query) = statement {
        if let Select(select) = &mut *query.body {
            for table_with_joins in &mut select.from {
                table_count = register_table(ctx, &mut table_with_joins.relation, table_count).await?;

                for join in &mut table_with_joins.joins {
                    table_count = register_table(ctx, &mut join.relation, table_count).await?;
                }
            }
        }
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
