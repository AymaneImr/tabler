#![allow(unused)]
use rust_test::file_extension::*;
use table_structure;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let df = FileInfo::get_df(args);
    table_structure::design(df);
}



