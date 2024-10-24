//extract all the data as a simple dataframe 
// |------|------|
// |column|column|
// |------|------|
// | row  | row  |
// |------|------|

#![allow(unused)]

use std::{error::Error, fmt::format, fs::File};
use std::io::BufReader;
use csv::{Reader, ReaderBuilder, StringRecord};
use serde_json::{Result as re, Value};
use calamine::{open_workbook, Error as er, Xlsx, Reader as red, RangeDeserializerBuilder};
use std::collections::HashMap;

struct Files{
    filename: FileExtension,
    file_ext: String
}

#[derive(Debug)]
pub struct DataFrame<T, U>{
    pub columns: Vec<T>,
    pub rows:  Box<Vec<HashMap<T, U>>>,
}

pub enum FileExtension {
    Json,
    Csv,
    Excel
}

impl FileExtension {
    //get the file extension from the file name
    pub fn get_file_extension(filename:  &str) -> Result<FileExtension, Box<dyn Error>> {
        let extension: Vec<_> = filename.split(".").collect();
        match extension.last().unwrap()
        .to_lowercase().as_str(){
            "json" => {
                Files{ filename: FileExtension::Json, 
                    file_ext: String::from("json")
                };
                Ok(FileExtension::Json)
            },
            "csv"  => {
                Files{ filename: FileExtension::Csv,
                    file_ext: String::from("csv")
                    };
                Ok(FileExtension::Csv)
            },
            "xlsx" | "xls" => {
                Files{ filename: FileExtension::Excel,
                    file_ext: String::from("xlsx")
                    };
                Ok(FileExtension::Excel)
                },
            nts => Err(From::from(format!("Unsupported file extension : {}", nts)))
        }
    }

    //read csv files 
    pub fn read_csv(filename: &str) -> Result<DataFrame<String, String>, Box<dyn Error>> {
        let file = File::open(filename)?;
        let mut reader: Reader<BufReader<File>> = ReaderBuilder::new()
            .has_headers(true).from_reader(BufReader::new(file));

        //store the columns
        let headers = reader.headers()?;
        let mut columns: Vec<String> = headers.iter().map(|col| col.to_string()).collect();
        
        //store the rows
        let mut rows: Vec<HashMap<String, String>> = Vec::new();
        
        for result in reader.records() {
            let record:StringRecord = result?;
            let mut row_map: HashMap<String, String> = HashMap::new();

            //iterate through each enumerated row and assign each row to it's column
            for (i,  row) in record.iter().enumerate() {
                row_map.insert(columns[i].to_string(),  row.to_string());
            }
            rows.push(row_map);
        }
        Ok(
            DataFrame {
                columns: columns,
                rows: Box::new(rows),
            }
        )
    }

    //FIX LATER -> improve the structured dataframe for json files to read a complex format
    //read json file
    pub fn read_json(filename: &str) -> Result<DataFrame<String, String>, Box<dyn Error>> {
        let file: File  = File::open(filename)?;
        let content: Value = serde_json::from_reader(file)?;
        let columns: Vec<String> = content.as_object().unwrap().keys()
        .map(|v| v.to_string()).collect();
        
        let rows_content: Vec<String> = content.as_object().unwrap().values()
        .map(|v| v.to_string()).collect();
        
        let mut rows: Vec<HashMap<String, String>> = Vec::new();
        let mut row_map: HashMap<String, String> = HashMap::new();

        for (i, row) in rows_content.iter().enumerate(){
            row_map.insert(columns[i].to_string(), row.to_string());
        }
        rows.push(row_map);

        Ok(
            DataFrame {
                columns: columns,
                rows: Box::new(rows),
            }
        )
    }
    
    //read excel file
    pub fn read_excel(filename: &str)  -> Result<DataFrame<String, String>, Box<dyn Error>> {
        let mut workbook: Xlsx<_> = open_workbook(filename)?;
        let range = workbook.worksheet_range("Sheet1")?;

        let columns = range.headers().unwrap();
        let mut rows: Vec<HashMap<String, String>> = Vec::new();
        
        // Iterate through the rows and columns of the range
        for row in range.rows(){
            let mut rows_map: HashMap<String, String> = HashMap::new();

            for (i, cell) in row.iter().enumerate(){
                rows_map.insert(columns[i].to_string(), cell.to_string());
            }
            rows.push(rows_map);
        }
        
        Ok(
            DataFrame {
            columns: columns,
            rows: Box::new(rows),
            }
        )
    }
}


mod table_structure;

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn it_works() {
//        let df = FileExtension::read_excel("src/file_extension/file.xlsx").unwrap();
//        println!("{:#?}", df)
//    }
//}
