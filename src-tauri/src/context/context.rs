use crate::context::error::AppError;
use crate::context::schema::EasyDBResult;
use crate::sql::parse::{get_function_args, get_path, parse_statements};
use polars::frame::DataFrame;
use polars::prelude::{LazyCsvReader, LazyFileListReader, LazyFrame, PlPath};
use polars::sql::SQLContext;
use sqlparser::ast::SetExpr::Select;
use sqlparser::ast::{
    Expr, FunctionArg, FunctionArgExpr, Statement, TableFactor, TableFunctionArgs, Value,
};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

pub fn get_sql_context() -> SQLContext {
    SQLContext::new()
}

pub fn get_data_frame(ctx: &mut SQLContext, sql: &str) -> EasyDBResult<DataFrame> {
    ctx.execute(sql)?.collect().map_err(AppError::from)
}

pub fn parse_sql() -> Parser<'static> {
    Parser::new(&GenericDialect)
}

pub fn get_csv_reader(path: PlPath) -> LazyCsvReader {
    LazyCsvReader::new(path)
}

pub fn read_csv(
    mut reader: LazyCsvReader,
    args: &mut Option<TableFunctionArgs>,
) -> EasyDBResult<LazyFrame> {
    let args = get_function_args(args);

    if let Some(args) = args {
        for arg in args {
            if let FunctionArg::Named { name, arg, .. } = arg {
                match name.value.as_str() {
                    "infer_schema" => {
                        if let FunctionArgExpr::Expr(Expr::Value(Value::Boolean(value))) = arg {
                            if (!value) {
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

pub fn register_table(
    ctx: &mut SQLContext,
    sql: &str,
    limit: Option<String>,
) -> EasyDBResult<String> {
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
