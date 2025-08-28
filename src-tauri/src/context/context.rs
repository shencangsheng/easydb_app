use polars::frame::DataFrame;
use polars::sql::SQLContext;
use crate::context::error::AppError;
use crate::context::schema::EasyDBResult;

pub fn get_sql_context() -> SQLContext {
    SQLContext::new()
}

pub fn get_data_frame(ctx: &mut SQLContext, sql: &str) -> EasyDBResult<DataFrame> {
    ctx.execute(sql)?.collect().map_err(AppError::from)
}

