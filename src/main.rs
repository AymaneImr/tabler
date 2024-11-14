#![allow(unused)]

use relative_path::{RelativePath, RelativePathBuf};
use std::env;
use std::env::current_dir;
use std::error::Error;
use std::path::Path;
use table_structure::design;
use tabler::file_extension::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let df = FileInfo::get_df(args);
    match df {
        Ok(df) => {
            design(df);
        }
        Err(er) => eprintln!("{er} "),
    }
}
