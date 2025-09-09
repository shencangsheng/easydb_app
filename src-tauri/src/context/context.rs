use crate::context::error::AppError;
use crate::context::schema::EasyDBResult;
use polars::frame::DataFrame;
use polars::sql::SQLContext;
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

pub fn table_function(sql: &str) -> EasyDBResult<String> {
    let mut parser = Parser::new(&GenericDialect);
    Ok("".to_string())
}
