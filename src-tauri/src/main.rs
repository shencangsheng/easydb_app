// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use polars::prelude::{
    IntoLazy, LazyFileListReader
    ,
};
use std::any::Any;

mod context;

mod sql;

fn main() {
    app_lib::run();
}
