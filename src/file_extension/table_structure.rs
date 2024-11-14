// create a custom design for a dataframe
// +------+------+
// |column|column/
// +======+======+
// | row  | row  |
// +------+------+

// Note: large files dataframes are not supported yet
// if any cell contains a long String or Integer this may cause in a readability mess
// more dataframe improvements will be implemented soon

use super::DataFrame;
use prettytable::{
    color,
    format::{self, Alignment},
    Attr, Cell, Row, Slice, Table,
};

//A table design for csv and excel files
pub fn design(df: DataFrame<String, String>) {
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
        .indent(30)
        .build();

    //a simple way to make the table size responsive manually
    //Note: the size of the table will be determined by the column's data length
    // => a parameter will be provided to set the indent of the table if there is a readability issue
    //there will be a more stable way to do this in the future.
    let mut indent = 50;
    let length = &row_length;
    if *length >= 6 {
        for i in 6..15 {
            if *length >= i {
                indent -= 10
            }
        }
    } else {
        for i in 2..8 {
            if *length <= i {
                indent += 5
            }
        }
    }
    format.indent(indent);

    table.set_format(format);
    let slice = table.slice(..200);
    slice.printstd()
}
