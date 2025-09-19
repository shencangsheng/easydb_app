use app_lib::context::context::{collect, register};
use calamine::{open_workbook, HeaderRow, Reader, Xlsx, XlsxError};
use polars::sql::SQLContext;
use std::error::Error;
use polars::datatypes::AnyValue;
use app_lib::context::schema::AppResult;

#[test]
fn test_1() {
    let sql = "select * from read_excel('/Users/shencangsheng/Downloads/20250916111918.xlsx', sheet_name => '其他')";

    let mut context = SQLContext::new();

    let new_sql = register(&mut context, &sql, Some("200".to_string())).unwrap();

    let df = collect(&mut context, &new_sql).unwrap();
}

#[test]
fn test_2() -> AppResult<()> {
    let sql = "select * from read_csv('/tmp/sequences.fa_user.tn93output.csv')";

    let mut context = SQLContext::new();

    let new_sql = register(&mut context, &sql, Some("200".to_string()))?;

    let df = collect(&mut context, &new_sql)?;

    let header: Vec<String> = df.column_iter().map(|c| c.name().to_string()).collect();

    let height = df.height();
    let width = df.width();

    let mut rows: Vec<Vec<String>> = vec![vec![String::new(); width]; height];

    let mut row_i = 0;
    df.iter().for_each(|col| {
        col.iter().enumerate().for_each(|(index, value)| {
            rows.get_mut(index).unwrap()[row_i] = match value {
                AnyValue::Null => "NULL".to_string(),
                _ => value.to_string().replace("\"", ""),
            }
        });
        row_i += 1;
    });

    println!("{:?}", rows);

    Ok(())
}