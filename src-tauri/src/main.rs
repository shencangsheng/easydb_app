// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use polars::prelude::PlPath::Local;
use polars::prelude::{
    CsvReader, CsvWriterOptions, IntoLazy, LazyCsvReader, LazyFileListReader, LazyFrame, PlPath,
    SinkOptions, SinkTarget,
};
use polars::sql::SQLContext;
use std::any::Any;

use crate::context::context::register_table;
use polars::prelude::Column::{Scalar, Series};
use sqlparser::ast::{Statement, TableFactor};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

mod context;

mod sql;

fn main() {
    app_lib::run();
}
