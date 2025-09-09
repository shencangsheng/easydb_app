use crate::context::error::AppError;
use crate::context::schema::AppResult;
use crate::sql::parse::{get_function_args, parse_statements};
use polars::frame::DataFrame;
use polars::prelude::{
    JsonReader, LazyCsvReader, LazyFileListReader, LazyFrame, LazyJsonLineReader, PlPath,
};
use polars::sql::SQLContext;
use sqlparser::ast::SetExpr::Select;
use sqlparser::ast::{
    Expr, FunctionArg, FunctionArgExpr, Statement, TableFactor, TableFunctionArgs, Value,
};

pub fn get_sql_context() -> SQLContext {
    SQLContext::new()
}

pub fn collect(ctx: &mut SQLContext, sql: &String) -> AppResult<DataFrame> {
    ctx.execute(sql)?.collect().map_err(AppError::from)
}

pub fn get_csv_reader(path: PlPath) -> LazyCsvReader {
    LazyCsvReader::new(path)
}

pub fn get_json_reader(path: PlPath) -> LazyJsonLineReader {
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

pub fn read_json(
    mut reader: LazyJsonLineReader,
    args: &mut Option<TableFunctionArgs>,
) -> AppResult<LazyFrame> {
    let args = get_function_args(args);
    reader.finish().map_err(|e| e.into())
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
                if let TableFactor::Table { name, args, .. } = &mut table_with_joins.relation {
                    let table_name = format!("t{}", table_count);
                    let path = get_path(args)?;
                    let lazy_frame: Option<LazyFrame> = match name.to_string().as_str() {
                        "read_csv" => Some(read_csv(get_csv_reader(path), args)?),
                        "read_tsv" => {
                            let mut reader = get_csv_reader(path);
                            reader = reader.with_separator(b'\t');
                            Some(read_csv(reader, args)?)
                        }
                        "read_json" => Some(read_json(get_json_reader(path), args)?),
                        _ => None,
                    };

                    if lazy_frame.is_none() {
                        return Err(AppError::BadRequest {
                            message: format!("'{}' is not a supported table function", table_name),
                        });
                    }

                    *name = sqlparser::ast::ObjectName(vec![table_name.as_str().into()]);
                    *args = None;
                    ctx.register(table_name.as_str(), lazy_frame.unwrap());
                }
                table_count += 1;
            }
        }
        if limit.is_some() && query.limit.is_none() {
            query.limit = Some(Expr::Value(Value::Number(limit.unwrap(), true)));
        }
        return Ok(query.to_string());
    } else {
        return Err(AppError::BadRequest {
            message: "Only supports Select statements.".to_string(),
        });
    }
}
