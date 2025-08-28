// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use polars::prelude::PlPath::Local;
use polars::prelude::{CsvReader, CsvWriterOptions, IntoLazy, LazyCsvReader, LazyFileListReader, LazyFrame, PlPath, SinkOptions, SinkTarget};
use polars::sql::SQLContext;
use std::path::Path;
use std::sync::Arc;

mod context;

fn main() {
    let mut context = SQLContext::new();

    let path: PlPath = Local(Arc::from(Path::new("/Users/shencangsheng/Code Repository/easy_db/example/order_*.csv")));

    // let sink = SinkTarget::Path(path);

    let lf = LazyCsvReader::new(path)
        .with_try_parse_dates(true)
        .with_missing_is_null(true)
        .finish().unwrap();

    // let lazy_frame = LazyFrame::default()
    //     .sink_csv(
    //         sink,
    //         CsvWriterOptions::default(),
    //         None,
    //         SinkOptions::default(),
    //     )
    //     .unwrap();

    context.register("order", lf);

    let sql = r#"
            SELECT *
            FROM order"#;
    let df_sql = context.execute(sql).unwrap().collect().unwrap();

    df_sql.iter().for_each(|row| {
        println!("{:?}", row);
    });
    // app_lib::run();
}
