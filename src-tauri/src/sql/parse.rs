use crate::context::schema::AppResult;
use sqlparser::ast::{FunctionArg, Statement, TableFunctionArgs};
use sqlparser::dialect::{Dialect, GenericDialect};
use sqlparser::parser::Parser;

#[derive(Debug)]
pub struct EasyDBDialect;

impl Dialect for EasyDBDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        GenericDialect.is_identifier_start(ch)
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        GenericDialect.is_identifier_part(ch)
    }

    fn supports_named_fn_args_with_expr_name(&self) -> bool {
        false
    }
}

pub fn parse_statements(sql: &str) -> AppResult<Vec<Statement>> {
    let parser = Parser::new(&EasyDBDialect);
    parser
        .try_with_sql(sql)?
        .parse_statements()
        .map_err(|e| e.into())
}

pub fn get_function_args(args: &mut Option<TableFunctionArgs>) -> Option<&Vec<FunctionArg>> {
    if let Some(ref mut table_args) = args {
        return Some(&table_args.args);
    }
    None
}
