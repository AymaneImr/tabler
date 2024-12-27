#![allow(unused)]

use clap::builder::EnumValueParser;
use clap::error::ErrorKind;
use clap::parser::MatchesError;
use clap::{crate_authors, parser, value_parser, Arg, ArgAction, ArgMatches, Command};
use relative_path::{RelativePath, RelativePathBuf};
use std::path::{Path, PathBuf};
use std::string;
use table_structure::design;
use tabler::file_extension::*;

#[derive(Debug)]
pub struct Args {
    file_path: PathBuf,
    rows: Option<usize>,
    sheet_name: Option<String>,
    nested_json: bool,
    columns: Option<Vec<String>>,
    default_rows: bool,
    indent: bool,
}

fn main() {
    let args = arguments();
    let parsed_args = parse_args(args);

    let df = FileInfo::get_df(parsed_args.file_path, parsed_args.sheet_name);
    match df {
        Ok(mut df) => {
            df.columns = match parsed_args.columns {
                Some(col) => {
                    let mut new_columns: Vec<String> = Vec::new();
                    let mut misspelled_columns: Vec<String> = Vec::new();

                    for c in col {
                        if df.columns.contains(&c) {
                            new_columns.push(c);
                        } else {
                            misspelled_columns.push(c);
                        }
                    }
                    if !misspelled_columns.is_empty() {
                        eprintln!(
                            "Oops you have misspelled these columns names: {:?}",
                            misspelled_columns
                        )
                    }
                    if !new_columns.is_empty() {
                        new_columns
                    } else {
                        df.columns
                    }
                }
                _ => df.columns,
            };
            design(
                df,
                parsed_args.rows,
                parsed_args.default_rows,
                parsed_args.indent,
            );
        }
        Err(er) => eprintln!("{er} "),
    }
}

fn arguments() -> ArgMatches {
    Command::new("tabler")
        .author("Archon => https://github.com/AymaneImr")
        .version("0.1.0")
        .about("tabler is a terminal-based application to open and view structured data files like CSV, Excel, and JSON in a tabular format.")
        .arg(Arg::new("PATH")
            .help("Specify the file path")
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
            Arg::new("default-rows")
                .short('d')
                .long("default-rows")
                .alias("default")
                .conflicts_with("rows")
                .action(ArgAction::SetTrue)
                .help("Sets the number of rows to `200`")
        )
        .arg(
            Arg::new("indentation")
                .short('i')
                .long("indent")
                .action(ArgAction::SetTrue)
                .help("Sets the Indentation level to `0`. This is recommended for long table width")
        )
        .arg(
            Arg::new("sheet-name")
                .short('s')
                .long("sheet")
                .action(ArgAction::Set)
                .conflicts_with("nested-json")
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
        .subcommand(
            Command::new("columns")
            .about("Specify the columns to be displayed")
                .arg(
                     Arg::new("columns")
                          .action(ArgAction::Append)
                          .value_parser(value_parser!(String))
                          .help("Specify the columns to be displayed")
                )
        )
        .get_matches()
}

fn parse_args(args: ArgMatches) -> Args {
    let mut file_path: PathBuf = PathBuf::new();
    if let Some(path) = args.get_one::<PathBuf>("PATH") {
        file_path = path.to_path_buf()
    }
    let rows: Option<usize> = args.get_one::<usize>("rows").map(|f| f.to_owned() as usize);

    let sheet_name: Option<String> = args.get_one::<String>("sheet-name").map(|f| f.to_string());

    let nested_json: bool = args.get_flag("nested-json");

    let columns: Option<Vec<String>> = match args.subcommand() {
        Some(("columns", sub_matches)) => Some(
            sub_matches
                .get_many::<String>("columns")
                .unwrap_or_default()
                .map(|f| f.to_string())
                .collect::<Vec<_>>(),
        ),
        _ => None,
    };

    let default_rows: bool = args.get_flag("default-rows");

    let indent: bool = args.get_flag("indentation");

    Args {
        file_path,
        rows,
        sheet_name,
        nested_json,
        columns,
        default_rows,
        indent,
    }
}
