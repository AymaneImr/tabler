#![allow(unused)]

use clap::error::ErrorKind;
use clap::parser::MatchesError;
use clap::{crate_authors, parser, value_parser, Arg, ArgAction, ArgMatches, Command};
use relative_path::{RelativePath, RelativePathBuf};
use std::path::{Path, PathBuf};
use table_structure::design;
use tabler::file_extension::*;

#[derive(Debug)]
pub struct Args {
    file_path: PathBuf,
    pub rows: Option<usize>,
    pub sheet_name: Option<String>,
    pub nested_json: bool,
}

fn main() {
    let args = arguments();
    let parsed_args = parse_args(args);

    let df = FileInfo::get_df(parsed_args.file_path, parsed_args.sheet_name);
    match df {
        Ok(df) => {
            design(df, parsed_args.rows);
        }
        Err(er) => eprintln!("{er} "),
    }
}

fn arguments() -> ArgMatches {
    Command::new("tabler")
        .author("Archon => https://github.com/AymaneImr")
        .version("0.1.0")
        .about("tabler is a terminal-based application to open and view structured data files like CSV, Excel, and JSON in a tabular format.")
        .arg(Arg::new("filename")
            .help("file")
            .value_parser(value_parser!(PathBuf))
            .required(true))
        .arg(
            Arg::new("rows")
                .short('r')
                .long("rows")
                .alias("row")
                .action(ArgAction::Set)
                .value_parser(value_parser!(usize))
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

fn parse_args(args: ArgMatches) -> Args {
    let mut file_path: PathBuf = PathBuf::new();
    if let Some(path) = args.get_one::<PathBuf>("filename") {
        file_path = path.to_path_buf()
    }
    let mut rows: Option<usize> = args.get_one::<usize>("rows").map(|f| f.to_owned() as usize);

    let mut sheet_name: Option<String> =
        args.get_one::<String>("sheet-name").map(|f| f.to_string());

    let mut nested_json: bool = args.get_flag("nested-json");

    Args {
        file_path,
        rows,
        sheet_name,
        nested_json,
    }
}
