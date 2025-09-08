// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::any::Any;
use polars::prelude::PlPath::Local;
use polars::prelude::{
    CsvReader, CsvWriterOptions, IntoLazy, LazyCsvReader, LazyFileListReader, LazyFrame, PlPath,
    SinkOptions, SinkTarget,
};
use polars::sql::SQLContext;

use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use sqlparser::ast::{Statement, TableFactor};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

mod context;

fn main() {
    let mut context = SQLContext::new();

    // let path: PlPath = Local(Arc::from(Path::new("/Users/shencangsheng/Code Repository/easy_db/example/order_*.csv")));
    //
    // // let sink = SinkTarget::Path(path);
    //
    // let lf = LazyCsvReader::new(path)
    //     .with_try_parse_dates(true)
    //     .with_missing_is_null(true)
    //     .finish().unwrap();

    // let lazy_frame = LazyFrame::default()
    //     .sink_csv(
    //         sink,
    //         CsvWriterOptions::default(),
    //         None,
    //         SinkOptions::default(),
    //     )
    //     .unwrap();

    // context.register("order", lf);



    let start = Instant::now();

    let sql = r#"
            SELECT *
            FROM read_tsv('/tmp/test.tsv', setnull(11))"#;

    let mut parser = Parser::new(&GenericDialect);

    let mut ast = parser
        .try_with_sql(sql).unwrap().parse_statements().unwrap();

    for stmt in &mut ast {
        if let Statement::Query(query) = stmt {
            if let sqlparser::ast::SetExpr::Select(select) = &mut *query.body {
                for table_with_joins in &mut select.from {
                    if let TableFactor::Table { name, args, .. } = &mut table_with_joins.relation {
                        // 修改表名
                        *name = sqlparser::ast::ObjectName(vec!["new_table_name".into()]);
                    }
                }
            }
        }
    }

    for stmt in ast {
        if let Statement::Query(query) = stmt {
            println!("{}", query.body.to_string());
        }
    }

    println!("Parse time: {:?}", start.elapsed());

    // let df_sql = SQLContext::new().execute(sql).unwrap().collect().unwrap();
    //
    // df_sql.iter().for_each(|row| {
    //     println!("{:?}", row);
    // });
    //
    // // 结束计时
    // let duration = start.elapsed();
    //
    // // 打印耗时
    // println!("代码执行时间: {:?}", duration);

    // app_lib::run();
}
