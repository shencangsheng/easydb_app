use app_lib::context::context::{collect, register};
use polars::sql::SQLContext;

#[test]
fn test_1() {

    let sql = "select * from read_excel('/Users/shencangsheng/Downloads/20250916111918.xlsx', sheet_name => '其他')";

    let mut context = SQLContext::new();

    let new_sql = register(&mut context, &sql, Some("200".to_string())).unwrap();

    let df = collect(&mut context, &new_sql).unwrap();
}

#[test]
fn test_2() {

    let sql = "select * from read_tsv('/Users/shencangsheng/Downloads/nightly-VariantSummaries.tsv', infer_schema => false)";

    let mut context = SQLContext::new();

    let new_sql = register(&mut context, &sql, Some("200".to_string())).unwrap();

    let df = collect(&mut context, &new_sql).unwrap();
}