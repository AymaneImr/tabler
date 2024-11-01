// create a custom design for a dataframe
// +------+------+
// |column|column/
// +------+------+
// | row  | row  |
// +------+------+

use super::DataFrame;
use prettytable::{Table, Cell, Row, color, format::{self, Alignment}, Attr, Slice};

//A table design for csv and excel files 
pub fn design(df: DataFrame<String, String>) {
    let mut table = Table::new();

    //column's style
    table.set_titles(Row::new(
        df.columns.iter().map(|col| Cell::new_align(col, Alignment::CENTER)
        .with_style(Attr::Bold).with_style(Attr::ForegroundColor(color::CYAN))
    ).collect()
    ));

    //add rows 
    for row in df.rows.iter().as_ref(){
        let row_vec: Vec<String> = df.columns.iter()
        .map(|col| row.get(col).unwrap_or(&String::from("NaN")).to_string())
        .collect();
        
        //row's style
        table.add_row(Row::new(row_vec.iter()
        .map(|f| Cell::new_align(f, Alignment::CENTER)
            ).collect()
        ));
    }

    let format = format::FormatBuilder::new()
    .column_separator('|')
    .borders('|')
    .separators(&[format::LinePosition::Top, format::LinePosition::Bottom],
                format::LineSeparator::new('-', '+', '+', '+'))
    .separator(format::LinePosition::Intern, 
                format::LineSeparator::new('-', '-', '+', '+'))
    .padding(1, 1)
    .indent(30)
    .build();
    
    table.set_format(format);
    let slice = table.slice(..10);
    slice.printstd()
}
