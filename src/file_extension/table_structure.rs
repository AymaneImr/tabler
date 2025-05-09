// create a custom design for a dataframe
// +------+------+
// |column|column|
// +======+======+
// | row  | row  |
// +------+------+

// Note: large files dataframes are not supported yet
// 90% FIXED ==> if any cell contains a long String or Integer this may cause in a readability mess
// more dataframe improvements will be implemented soon

use std::default;

use super::DataFrame;
use prettytable::{
    color,
    format::{self, Alignment},
    Attr, Cell, Row, Slice, Table,
};

#[derive(Debug, PartialEq)]
struct Indentation {
    indent: usize,
}

impl Indentation {
    fn get_indentation(row_length: usize, indentation: bool) -> Self {
        /*a simple way to make the table size responsive manually
        Note: the size of the table will be determined by the column's data length
         => a parameter will be provided to set the indent of the table if there is a readability issue
        there will be a more stable way to do this in the future. */
        let mut indent: usize = 50;

        let length = &row_length;
        if indentation {
            indent = 0
        } else if *length >= 6 {
            for i in 6..15 {
                if *length >= i {
                    indent -= 10
                }
            }
        } else {
            indent = 40;
            for i in 2..8 {
                if *length <= i {
                    indent += 5
                }
            }
        }
        Indentation { indent }
    }
}

//A table design for csv and excel files
pub fn design(
    df: DataFrame<String, String>,
    rows: Option<usize>,
    default_rows: bool,
    indent: bool,
) {
    let mut table = Table::new();

    //column's style
    //give the columns a unique borders for better readability
    let col_row = Row::new(
        df.columns
            .iter()
            .map(|col| {
                Cell::new_align(col, Alignment::CENTER)
                    .with_style(Attr::Bold)
                    .with_style(Attr::ForegroundColor(color::CYAN))
            })
            .collect(),
    );
    let row_length = col_row.len();
    table.set_titles(col_row);

    //add rows
    for row in df.rows.iter().as_ref() {
        let row_vec: Vec<String> = df
            .columns
            .iter()
            .map(|col| row.get(col).unwrap_or(&String::from("NaN")).to_string())
            .collect();

        //row's style
        table.add_row(Row::new(
            row_vec
                .iter()
                .map(|f| Cell::new_align(f, Alignment::CENTER))
                .collect(),
        ));
    }

    let mut format = format::FormatBuilder::new()
        .column_separator('│')
        .borders('│')
        .separators(
            &[format::LinePosition::Top, format::LinePosition::Bottom],
            format::LineSeparator::new('-', '+', '+', '+'),
        )
        .separator(
            format::LinePosition::Intern,
            format::LineSeparator::new('-', '-', '+', '+'),
        )
        .separator(
            format::LinePosition::Title,
            format::LineSeparator::new('=', '+', '+', '+'),
        )
        .padding(1, 1)
        .build();

    let indentation = Indentation::get_indentation(row_length, indent);
    format.indent(indentation.indent);

    table.set_format(format);

    let mut slice = if default_rows && table.len() >= 200 {
        table.slice(..200)
    } else {
        table.slice(..)
    };
    if let Some(row) = rows {
        if table.len() >= row {
            slice = table.slice(..row);
        } else {
            println!(
                "Requested {} rows, but only {} are available. Displaying all rows.",
                row,
                table.len(),
            );
            slice = table.slice(..)
        }
    }
    slice.printstd();
}
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {

    }
}*/
