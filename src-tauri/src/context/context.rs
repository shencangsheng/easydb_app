use crate::context::error::AppError;
use crate::context::schema::AppResult;
use crate::reader::excel::ExcelReader;
use crate::sql::parse::{get_function_args, parse_statements};
use polars::frame::DataFrame;
use polars::io::SerReader;
use polars::prelude::{
    IntoLazy, JsonReader, LazyCsvReader, LazyFileListReader, LazyFrame, LazyJsonLineReader, PlPath,
};
use polars::sql::SQLContext;
use sqlparser::ast::SetExpr::Select;
use sqlparser::ast::{
    Expr, FunctionArg, FunctionArgExpr, Join, Statement, TableFactor, TableFunctionArgs, Value,
};
use sqlparser::keywords::Keyword::JOIN;
use std::fs::File;

pub fn get_sql_context() -> SQLContext {
    SQLContext::new()
}

pub fn collect(ctx: &mut SQLContext, sql: &String) -> AppResult<DataFrame> {
    ctx.execute(sql)?.collect().map_err(AppError::from)
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
            message: "The file path is missing.".to_string(),
        })?;

    match value {
        FunctionArg::Unnamed(FunctionArgExpr::Expr(Expr::Value(Value::SingleQuotedString(
            value,
        )))) => Ok(PlPath::new(value)),
        _ => Err(AppError::BadRequest {
            message: "The file path is missing.".to_string(),
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

enum FrameType {
    Lazy(LazyFrame),
    Data(DataFrame),
}

fn register_table(
    ctx: &mut SQLContext,
    relation: &mut TableFactor,
    table_count: i32,
) -> AppResult<i32> {
    if let TableFactor::Table { name, args, .. } = relation {
        let table_name = format!("table{}", table_count);
        let path = get_path(args)?;
        let lazy_frame: Option<FrameType> = match name.to_string().as_str() {
            "read_csv" => Some(FrameType::Lazy(read_csv(get_csv_reader(path), args)?)),
            "read_tsv" => {
                let mut reader = get_csv_reader(path);
                reader = reader.with_separator(b'\t');
                Some(FrameType::Lazy(read_csv(reader, args)?))
            }
            "read_ndjson" => Some(FrameType::Lazy(read_ndjson(get_ndjson_reader(path), args)?)),
            "read_json" => Some(FrameType::Data(read_json(get_json_reader(path)?, args)?)),
            "read_excel" => Some(FrameType::Data(read_excel(get_excel_reader(path), args)?)),
            _ => None,
        };

        if lazy_frame.is_none() {
            return Err(AppError::BadRequest {
                message: format!("'{}' is not a supported table function", table_name),
            });
        }

        *name = sqlparser::ast::ObjectName(vec![table_name.as_str().into()]);
        *args = None;
        if let Some(frame) = lazy_frame {
            match frame {
                FrameType::Lazy(lazy) => ctx.register(table_name.as_str(), lazy),
                FrameType::Data(data) => ctx.register(table_name.as_str(), data.lazy()),
            }
        }
    }

    Ok(table_count + 1)
}

pub fn register(ctx: &mut SQLContext, sql: &str, limit: Option<String>) -> AppResult<String> {
    let mut ast = parse_statements(sql)?;

    let statement = ast.get_mut(0).ok_or(AppError::BadRequest {
        message: "invalid SQL statement".to_string(),
    })?;

    let mut table_count = 1;

    if let Statement::Query(query) = statement {
        if let Select(select) = &mut *query.body {
            for table_with_joins in &mut select.from {
                table_count = register_table(ctx, &mut table_with_joins.relation, table_count)?;

                for join in &mut table_with_joins.joins {
                    table_count = register_table(ctx, &mut join.relation, table_count)?;
                }
            }
        }
        if limit.is_some() && query.limit.is_none() {
            query.limit = Some(Expr::Value(Value::Number(limit.unwrap(), true)));
        }
        Ok(query.to_string())
    } else {
        Err(AppError::BadRequest {
            message: "Only supports Select statements.".to_string(),
        })
    }
}
