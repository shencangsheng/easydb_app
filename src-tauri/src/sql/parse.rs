use crate::context::error::AppError;
use crate::context::schema::EasyDBResult;
use polars::prelude::{CsvParseOptions, LazyCsvReader, LazyFileListReader, LazyFrame, PlPath};
use polars::sql::SQLContext;
use sqlparser::ast::FunctionArg::Named;
use sqlparser::ast::SetExpr::Select;
use sqlparser::ast::{
    Expr, FunctionArg, FunctionArgExpr, FunctionArgOperator, Statement, TableFactor,
    TableFunctionArgs, Value,
};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

pub fn parse_statements(sql: &str) -> EasyDBResult<Vec<Statement>> {
    let parser = Parser::new(&GenericDialect);
    parser
        .try_with_sql(sql)?
        .parse_statements()
        .map_err(|e| e.into())
}

pub fn get_path(args: &mut Option<TableFunctionArgs>) -> EasyDBResult<PlPath> {
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


pub fn get_function_args(args: &mut Option<TableFunctionArgs>) -> Option<&Vec<FunctionArg>> {
    if let Some(ref mut table_args) = args {
        return Some(&table_args.args);
    }
    None
}
