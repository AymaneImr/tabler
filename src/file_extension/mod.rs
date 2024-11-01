#![allow(unused)]

use std::{error::Error, fs::File};
use std::io::BufReader;
use csv::{Reader, ReaderBuilder, StringRecord};
use serde_json::Value;
use calamine::{open_workbook, Xlsx, Reader as red};
use std::collections::{HashMap, HashSet};

pub struct FileInfo{
    pub filename: String,
    pub file_ext: FileExtension
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

impl FileInfo {
    //get the file extension from the file name
    pub fn get_file_extension(filename:  &str) -> Result<FileInfo, Box<dyn Error>> {
        let extension: Vec<_> = filename.split(".").collect();
        match extension.last().unwrap()
        .to_lowercase().as_str(){
            "json" => {
                Ok(
                    FileInfo{ filename: filename.to_string(), 
                        file_ext: FileExtension::Json
                    }
                )
            },
            "csv"  => {
                Ok(
                    FileInfo{ filename: filename.to_string(),
                    file_ext: FileExtension::Csv
                    }
                )
            },
            "xlsx" | "xls" => {                
                Ok(
                    FileInfo{ filename: filename.to_string(),
                        file_ext: FileExtension::Excel
                        }
                )
            },
            other => Err(From::from(format!("Unsupported file extension : {}", other)))
        }
    }

    //read csv files 
    pub fn read_csv(&self) -> Result<DataFrame<String, String>, Box<dyn Error>> {
        let file = File::open(&self.filename)?;
        let mut reader: Reader<BufReader<File>> = ReaderBuilder::new()
            .has_headers(true).from_reader(BufReader::new(file));

        //store the columns
        let headers = reader.headers()?;
        let columns: Vec<String> = headers.iter().map(|col| col.to_string()).collect();
        
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

    //FIX LATER -> improve the structured dataframe for json files to read a complex/nested format
    //read json file
    pub fn read_json(&self) -> Result<DataFrame<String, String>, Box<dyn Error>> {
        let file: File  = File::open(&self.filename)?;
        let content: Value = serde_json::from_reader(file)?;
        let mut rows: Vec<HashMap<String, String>> = Vec::new();
        
        //i hate working with recursive
        fn iterate(value: &Value, prefix: String, rows: &mut Vec<HashMap<String, String>>) {
            match value {
                Value::Object(map) => {
                    for (key, val) in map{
                        if prefix.is_empty(){
                            iterate(val, format!("{}", key), rows);
                        }else {
                            iterate(val, format!("{}.{}", prefix, key), rows);
                        }
                    }
                }
                Value::Array(arr) => {
                    for (_, val) in arr.iter().enumerate(){
                        iterate(val, format!("{}", prefix), rows);
                    }
                }
                _ => {
                    let mut row_map: HashMap<String, String> = HashMap::new();
                    row_map.insert(prefix, value.to_string());
                    rows.push(row_map);
                }
            }
        }
        iterate(&content, String::new(), &mut rows);
        let columns: Vec<String> = rows.iter()
        .flat_map(|column| column.keys().cloned())
        .collect::<HashSet<String>>().into_iter().collect();
        
        Ok(
            DataFrame {
                columns: columns,
                rows: Box::new(rows),
            }
        )
    }
    
    //read excel file
    pub fn read_excel(&self)  -> Result<DataFrame<String, String>, Box<dyn Error>> {
        let mut workbook: Xlsx<_> = open_workbook(&self.filename)?;
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

    pub fn get_df(args: Vec<String>) -> DataFrame<String, String>{
        let filename = &args[1];
        let file_data = Self::get_file_extension(filename).unwrap();
    
        match file_data.file_ext {
            FileExtension::Csv => {
                let df = file_data.read_csv().unwrap();
                df
            },
            FileExtension::Json => {
                let df = file_data.read_json().unwrap();
                df
            },
            FileExtension::Excel => {
                let df = file_data.read_excel().unwrap();
                df
            }
        }
    }
}


pub mod table_structure;

//#[cfg(test)]
//mod tests {
    
    //use super::*;

    //#[test]
    //fn it_works() {
    //    let df = FileExtension::read_excel("src/file_extension/developers.xlsx").unwrap();
    //    let mut table = Table::new();

    //    for row in df.rows.iter().as_ref(){
    //        let row_vec: Vec<String> = df.columns.iter()
    //        .map(|col| row.get(col).unwrap_or(&String::from("NaN")).to_string())
    //        .collect();
    //        
    //        table.add_row(Row::new(row_vec.iter()
    //        .map(|f| Cell::new(f)).collect()));
    //        
    //    }
    //    table.printstd()
    //}
//}
