#![allow(unused)]
use calamine::{open_workbook, Reader as red, Xlsx};
use colored::*;
use csv::{Reader, ReaderBuilder, StringRecord};
use relative_path::RelativePath;
use serde_json::Value;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    ffi::OsStr,
    fmt::{Display, Formatter},
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};

pub struct FileInfo {
    pub file_path: PathBuf,
    pub file_ext: FileExtension,
}

#[derive(Debug)]
pub struct DataFrame<T, U> {
    pub columns: Vec<T>,
    pub rows: Vec<HashMap<T, U>>,
}

//TODO: Add support for other file types
pub enum FileExtension {
    Json,
    Csv,
    Excel,
}

#[derive(Debug)]
pub enum FileError {
    FileNotFound,
    EmptyFile,
    InvalidFile,
    UnsupportedExt,
    SheetNotFound,
}
impl Error for FileError {}

//TODO: Improve the error messages by providing more context, information
// and provide suggestions for the user if possible
impl Display for FileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::EmptyFile => "File is empty",
            Self::FileNotFound => "File not found",
            Self::InvalidFile => "Invalid file",
            Self::UnsupportedExt => "Unsupported file extension",
            Self::SheetNotFound => "Sheet not found",
        };
        write!(f, "Error: {}!", message)
    }
}

impl From<io::Error> for FileError {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::NotFound => Self::FileNotFound,
            _ => Self::InvalidFile, // more specific errors will be implemented later
        }
    }
}

impl From<serde_json::Error> for FileError {
    fn from(_: serde_json::Error) -> Self {
        Self::InvalidFile // more specific errors will be implemented later
    }
}

impl From<calamine::XlsxError> for FileError {
    fn from(value: calamine::XlsxError) -> Self {
        match value {
            calamine::XlsxError::FileNotFound(_) => Self::FileNotFound,
            calamine::XlsxError::WorksheetNotFound(_) => Self::SheetNotFound,
            calamine::XlsxError::TableNotFound(_) => Self::EmptyFile,
            _ => Self::InvalidFile, // more specific errors will be implemented later
        }
    }
}

type Results = Result<DataFrame<String, String>, FileError>;

impl FileInfo {
    //improve error handling
    //get the file extension from the file name

    pub fn get_file_extension(filename: PathBuf) -> Result<FileInfo, FileError> {
        match filename.extension().and_then(OsStr::to_str) {
            Some("json") => Ok(FileInfo {
                file_path: filename,
                file_ext: FileExtension::Json,
            }),
            Some("csv") => Ok(FileInfo {
                file_path: filename,
                file_ext: FileExtension::Csv,
            }),
            Some("xlsx") | Some("xls") => Ok(FileInfo {
                file_path: filename,
                file_ext: FileExtension::Excel,
            }),
            _other => Err(FileError::UnsupportedExt),
        }
    }

    //a simple way to avoid long cells which ends up extending the table and loses
    //the readability of the columns.
    //simply divide the row content by 10 `chars` separated by a Dash '-'
    //and the print out the rest of the row
    // Example => In a given row that contains "Wallpaper Engine Team"
    // this row will be displayed at this format
    //                                             +========================+
    //      ======>                                |   Wallpaper Engine Te- |
    //      ======>                                |            am          |
    //                                             +------------------------+
    fn responsive_row(row: &str) -> String {
        let mut responsive_row: String = String::new();

        if row.len() >= 35 {
            let first_slice: String = row.chars().take(20).collect();
            let second_slice: String = row.chars().skip(20).take(20).collect();
            let third_slice: String = row.chars().skip(20).skip(20).collect();

            responsive_row = format!("{}-\n{}-\n{}", first_slice, second_slice, third_slice)
                .trim()
                .to_string();
        } else if row.len() >= 20 {
            let first_slice: String = row.chars().take(20).collect();
            let second_slice: String = row.chars().skip(20).collect();

            responsive_row = format!("{}-\n{}", first_slice, second_slice)
                .trim()
                .to_string();
        } else {
            responsive_row = row.trim().to_string();
        }
        responsive_row.to_string()
    }

    //TODO: improve error handling
    //read csv files
    pub fn read_csv(&self) -> Results {
        let file = File::open(&self.file_path)?;
        let mut reader: Reader<BufReader<File>> = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(BufReader::new(file));
        let mut columns: Vec<String> = Vec::new();

        //store the columns
        if let Ok(headers) = reader.headers() {
            columns = headers.iter().map(|col| col.trim().to_string()).collect();
        }

        //store the rows
        let mut rows: Vec<HashMap<String, String>> = Vec::new();

        for result in reader.records().flatten() {
            let record: StringRecord = result;
            let mut row_map: HashMap<String, String> = HashMap::new();

            //iterate through each enumerated row and assign each row to it's column
            for (i, row) in record.iter().enumerate() {
                row_map.insert(columns[i].to_string(), Self::responsive_row(row));
            }
            rows.push(row_map);
        }

        if !rows.is_empty() {
            Ok(DataFrame { columns, rows })
        } else {
            Err(FileError::EmptyFile)
        }
    }

    //TODO: improve error handling
    //read json file
    pub fn read_json(&self) -> Results {
        let file = File::open(&self.file_path)?;
        let content = serde_json::from_reader(file)?;
        let mut rows: Vec<HashMap<String, String>> = Vec::new();

        let mut columns: HashSet<String> = HashSet::new();

        if let Value::Array(array) = content {
            for item in array {
                if let Value::Object(map) = item {
                    let mut row_map: HashMap<String, String> = HashMap::new();
                    for (key, value) in map {
                        columns.insert(key.clone());
                        row_map.insert(key, value.to_string());
                    }
                    rows.push(row_map);
                }
            }
        }

        let columns: Vec<String> = columns.into_iter().collect();

        if !rows.is_empty() {
            Ok(DataFrame { columns, rows })
        } else {
            Err(FileError::EmptyFile)
        }
    }

    // read nested json files and display it in a styled colored tree format
    pub fn read_nested_json(file_path: PathBuf) -> Result<(), FileError> {
        let file = File::open(file_path)?;
        let content = serde_json::from_reader(file)?;

        fn print_json_tree(value: &Value, indent: usize) {
            let prefix = " ".repeat(indent);

            match value {
                Value::Object(map) => {
                    for (key, val) in map {
                        println!("{}{}:", prefix, key.cyan().bold());
                        print_json_tree(val, indent + 4);
                    }
                }
                Value::Array(arr) => {
                    for (i, val) in arr.iter().enumerate() {
                        println!("{}[{}]:", prefix, (i + 1).to_string().yellow());
                        print_json_tree(val, indent + 4);
                    }
                }
                Value::String(s) => println!("{}└── {}", prefix, format!("\"{}\"", s).green()),
                Value::Number(num) => println!("{}└── {}", prefix, num.to_string().yellow()),
                Value::Bool(b) => println!("{}└── {}", prefix, b.to_string().red()),
                Value::Null => println!("{}└── {}", prefix, "null".red()),
            }
        }

        print_json_tree(&content, 4);
        Ok(())
    }

    //TODO: improve error handling
    //read excel file
    pub fn read_excel(&self, sheet_name: Option<String>) -> Results {
        let mut workbook: Xlsx<_> = open_workbook(&self.file_path)?;
        let range = workbook.worksheet_range(&sheet_name.unwrap_or("Sheet1".to_string()))?;

        //if the there are no headers in sheet, default columns are provided "1", "2", "3" etc.
        //more improvements will be made in the future.
        let mut columns: Vec<String> = Vec::new();
        if let Some(col) = range.headers() {
            columns = col.iter().map(|f| f.trim().to_string()).collect()
        }

        let mut rows: Vec<HashMap<String, String>> = Vec::new();

        // Iterate through the rows and columns of the range
        for row in range.rows() {
            let mut rows_map: HashMap<String, String> = HashMap::new();

            for (i, cell) in row.iter().enumerate() {
                rows_map.insert(
                    columns[i].to_string(),
                    Self::responsive_row(&cell.to_string()),
                );
            }
            rows.push(rows_map);
        }
        //check if the data is empty
        if !rows.is_empty() {
            //for some reason calamine treats headers as rows so we remove the first row(headers)
            rows.remove(0);
            Ok(DataFrame { columns, rows })
        } else {
            Err(FileError::EmptyFile)
        }
    }

    //TODO: improve error handling
    pub fn get_df(path: PathBuf, sheet_name: Option<String>) -> Results {
        let file_data = Self::get_file_extension(path)?;
        match file_data.file_ext {
            FileExtension::Csv => file_data.read_csv(),
            FileExtension::Json => file_data.read_json(),
            FileExtension::Excel => file_data.read_excel(sheet_name),
        }
    }
}

pub mod table_structure;

#[cfg(test)]
mod tests {

    use std::{ffi::OsStr, path::Path, str::FromStr};

    use super::*;

    #[test]
    fn works() {}
}
