use crate::context::schema::AppResult;
use sqlparser::ast::{
    FunctionArg, Statement,
    TableFunctionArgs,
};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

pub fn parse_statements(sql: &str) -> AppResult<Vec<Statement>> {
    let parser = Parser::new(&GenericDialect);
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
