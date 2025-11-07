// use app_lib::context::context::{collect, register};
// use app_lib::context::schema::AppResult;
// use polars::datatypes::AnyValue;
// use polars::sql::SQLContext;
//
// #[test]
// fn test_1() {
//     let sql = "select * from read_tsv('/Users/shencangsheng/Downloads/nightly-FeatureSummaries.tsv')";
//
//     let mut context = SQLContext::new();
//
//     let new_sql = register(&mut context, &sql, Some(200), None).unwrap();
//
//     let df = collect(&mut context, &new_sql).unwrap();
// }
//
// #[test]
// fn test_2() -> AppResult<()> {
//     let sql = r#"
//     SELECT * FROM read_csv('/Users/shencangsheng/Desktop/生成的CSV内容20250923180412.csv', infer_schema => FALSE)
//     "#;
//
//     let mut context = SQLContext::new();
//
//     let new_sql = register(&mut context, &sql, Some(200), None)?;
//
//     let df = collect(&mut context, &new_sql)?;
//
//     let header: Vec<String> = df.column_iter().map(|c| c.name().to_string()).collect();
//
//     let height = df.height();
//     let width = df.width();
//
//     let mut rows: Vec<Vec<String>> = vec![vec![String::new(); width]; height];
//
//     let mut row_i = 0;
//     df.iter().for_each(|col| {
//         col.rechunk().iter().enumerate().for_each(|(index, value)| {
//             rows.get_mut(index).unwrap()[row_i] = match value {
//                 AnyValue::Null => "NULL".to_string(),
//                 _ => value.to_string().replace("\"", ""),
//             }
//         });
//         row_i += 1;
//     });
//
//     println!("{:?}", rows);
//
//     Ok(())
// }
//
// #[test]
// fn test_3() -> AppResult<()> {
//     let sql = r#"
//     SELECT
//   *
// FROM
//   read_csv ('/tmp/output.csv', infer_schema => FALSE) as t1
//   inner join
//   read_tsv ('/tmp/o.tsv') as t2 on t1.ID1 = t2.order
// WHERE
//   REGEXP_LIKE (Distance, '^[0-9]+\.[0-9]+?$') = FALSE
//     "#;
//
//     let mut context = SQLContext::new();
//     let new_sql = register(&mut context, &sql, Some(200), None)?;
//     let df = collect(&mut context, &new_sql)?;
//
//     Ok(())
// }
