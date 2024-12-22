#![allow(unused)]

use clap::error::ErrorKind;
use clap::parser::MatchesError;
use clap::{crate_authors, parser, value_parser, Arg, ArgAction, ArgMatches, Command};
use relative_path::{RelativePath, RelativePathBuf};
use std::env::current_dir;
use std::path::Path;
use std::process::exit;
use std::{env, process};
use table_structure::design;
use tabler::file_extension::*;

#[derive(Debug)]
pub struct Args {
    filename: Option<String>,
    pub rows: Option<u16>,
    pub sheet_name: Option<String>,
    pub nested_json: Option<bool>,
}

fn main() {
    /*
    let args: Vec<String> = env::args().collect();
    let df = FileInfo::get_df(args);
    match df {
        Ok(df) => {
            design(df);
        }
        Err(er) => eprintln!("{er} "),
    }*/

    let args = arguments();
}

fn arguments() -> ArgMatches {
    Command::new("tabler")
        .author("Archon => https://github.com/AymaneImr")
        .version("0.1.0")
        .about("tabler is a terminal-based application to open and view structured data files like CSV, Excel, and JSON in a tabular format.")
        .arg(Arg::new("filename").required(true))
        .arg(
            Arg::new("rows")
                .short('r')
                .long("rows")
                .alias("row")
                .action(ArgAction::Set)
                .value_parser(value_parser!(u16))
                .help("Specify the number of rows to display"),
        )
        .arg(
            Arg::new("sheet-name")
                .short('s')
                .long("sheet")
                .action(ArgAction::Set)
                .value_parser(value_parser!(String))
                .help("Specify sheet name ('Sheet1' is set as the default)"),
        )
        .arg(
            Arg::new("nested-json")
                .short('n')
                .long("nested")
                .action(ArgAction::SetTrue)
                .help("Recommended format for nested json structure"),
        )
        .get_matches()
}

//fn parse_args(args: ArgMatches) -> Args {}
