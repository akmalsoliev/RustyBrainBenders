mod csv_processor;
// implement in future
// mod db_management;
mod user_cli;

use std::error::Error;
use std::path::PathBuf;

use csv_processor::{DataFrame, PathType, Values};
//use db_management::sql;
use user_cli::user_cli;

fn main() -> Result<(), Box<dyn Error>> {
    let _string_path = PathType::String("data/imdb_clean.csv".to_string());
    let csv_path = { PathType::Path(PathBuf::from("data/imdb_clean.csv".to_string())) };
    let getter = DataFrame::run(csv_path);
    let df = getter.df;

    let commands: Vec<String> = {
        vec!["sum", "count"]
            .iter()
            .map(|&s| s.to_string())
            .collect()
    };

    let (column_name, column_command) = user_cli(&df, &commands);

    if column_command == "sum" {
        match &df[&column_name] {
            Values::String(_) => panic!("You cannot perform that operation on this column"),
            Values::Float(vf) => println!("Sum: {}", vf.iter().sum::<f32>()),
            Values::Integer(vi) => println!("Sum: {}", vi.iter().sum::<i32>()),
        }
    } else if column_command == "count" {
        match &df[&column_name] {
            Values::String(sval) => println!("Count: {}", sval.iter().len()),
            Values::Float(fval) => println!("Count: {}", fval.iter().len()),
            Values::Integer(ival) => println!("Count: {}", ival.iter().len()),
        }
    }

    Ok(())
}
