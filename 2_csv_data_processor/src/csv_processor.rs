extern crate csv;

use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug)]
pub enum PathType {
    String(String),
    Path(PathBuf),
}

#[derive(Debug, Clone)]
pub enum Values {
    String(Vec<String>),
    Integer(Vec<i32>),
    Float(Vec<f32>),
}

pub struct DataFrame {
    pub df: HashMap<String, Values>,
}

impl DataFrame {
    pub fn new(df: HashMap<String, Values>) -> DataFrame {
        DataFrame { df }
    }

    pub fn run(path: PathType) -> DataFrame {
        let df: HashMap<String, Values> = DataFrame::get_csv(path).expect("Failed to extract CSV");
        DataFrame::new(df)
    }

    fn clean_path(path: PathType) -> PathBuf {
        match path {
            PathType::String(p) => PathBuf::from(p),
            PathType::Path(p) => p,
        }
    }

    fn parse_vectors(vector: Vec<String>) -> Values {
        let mut float_vals = false;
        let mut int_vals = false;
        let mut string_vals = true;

        for value in vector.clone() {
            if let Ok(_) = value.parse::<i32>() {
                int_vals = true;
                string_vals = false;
            } else if let Ok(_) = value.parse::<f32>() {
                float_vals = true;
                string_vals = false;
            } else {
                string_vals = true;
            }
        }

        // if there are floats in the vector then default type should be float
        if float_vals {
            int_vals = false
        }

        let mut return_vec: Values = Values::String(vector.clone());

        if int_vals && string_vals == false {
            return_vec = Values::Integer({
                let Ok(val) = vector.into_iter().map(|v| v.parse::<i32>()).collect() else {panic!("Failed integer conversion")};
                val
            })
        } else if float_vals && string_vals == false {
            return_vec = Values::Float({
                let Ok(val) = vector.into_iter().map(|v| v.parse::<f32>()).collect() else {panic!("Failed float conversion")};
                val
            })
        }

        return_vec
    }

    fn get_csv(path: PathType) -> Result<HashMap<String, Values>, Box<dyn Error>> {
        let processed_path = DataFrame::clean_path(path);
        let csv_file = csv::Reader::from_path(processed_path);
        let mut reader = csv_file.expect("File not found");
        let headers = reader.headers()?.clone();

        let mut df: HashMap<String, Vec<String>> = HashMap::new();

        for result in reader.records() {
            let record = result?;

            for (key, value) in headers.iter().zip(record.iter()) {
                {
                    df.entry(key.to_string())
                        .or_insert_with(Vec::new)
                        .push(value.to_string());
                }
            }
        }

        let mut df_processed: HashMap<String, Values> = HashMap::new();

        for key in df.keys() {
            let error_message: String = "Couldn't get the column".to_string();

            let vector: Vec<String> = df.get(key).expect(&error_message).clone();
            let processed_vector: Values = DataFrame::parse_vectors(vector);

            df_processed.insert(key.to_string(), processed_vector);
        }

        Ok(df_processed)
    }
}
