mod csv_processor;
// implement in future
// mod db_management;
//mod cli;

use csv_processor::{DataFrame, PathType, Values};
//use db_management::sql;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let _string_path = PathType::String("data/imdb_clean.csv".to_string());
    let test_path = { PathType::Path(PathBuf::from("data/imdb_clean.csv".to_string())) };
    let getter = DataFrame::run(test_path);
    let df = getter.df;

    let column: String = "release_year".to_string();

    match &df[&column] {
        Values::String(_) => panic!("You cannot perform that operation on this column"),
        Values::Float(vf) => println!("{}", vf.iter().sum::<f32>()),
        Values::Integer(vi) => println!("{}", vi.iter().sum::<i32>()),
    }

    Ok(())
}
