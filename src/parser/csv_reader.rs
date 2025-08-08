use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct CsvObject {
    pub header: Vec<String>,
    pub data: Vec<Vec<String>>,
}

impl CsvObject {
    pub fn read_from_file(path: &str) -> Result<CsvObject, String> {
        let csv_file = match fs::File::open(path) {
            Ok(f) => f,
            Err(e) => return Err(format!("Unable to open file {}. {}", path, e)),
        };

        let buffer = BufReader::new(csv_file);

        let mut csv_object = CsvObject {
            header: Vec::new(),
            data: Vec::new(),
        };

        let mut is_header = true;
        for line in buffer.lines() {
            match line {
                Ok(l) => {
                    if is_header {
                        l.split(',').for_each(|ele| {
                            csv_object.header.push(String::from(ele));
                        });

                        is_header = false;
                    } else {
                        let mut data_row = Vec::new();
                        l.split(',').for_each(|ele| {
                            data_row.push(String::from(ele));
                        });
                        csv_object.data.push(data_row);
                    }
                },
                Err(e) => return Err(format!("Error reading CSV file. {}", e)),
            }
        }

        Ok(csv_object)
    }
}