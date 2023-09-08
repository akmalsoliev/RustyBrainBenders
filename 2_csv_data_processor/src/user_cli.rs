use std::collections::HashMap;
use std::io::stdin;

use crate::csv_processor::Values;

pub fn user_cli(df: &HashMap<String, Values>, commands: &Vec<String>) -> (String, String) {
    let column_keys = df.keys();
    const INPUT_ERROR: &str = "Error in input, please try again later";

    // Column selection
    println!("Available columns, numerical selection:");
    for (index, column) in column_keys.clone().enumerate() {
        println!("{index}. {column}");
    }

    let mut column_number: String = String::new();
    println!("Please select the column: ");
    stdin().read_line(&mut column_number).expect(INPUT_ERROR);
    let parsed_input = column_number.trim().parse::<usize>().expect(INPUT_ERROR);
    let vec_columns: Vec<String> = column_keys.cloned().collect();
    let columns_name = vec_columns.get(parsed_input).expect(INPUT_ERROR);

    // Command selection
    for (index, command) in commands.iter().enumerate() {
        println!("{index}. {command}");
    }

    let mut column_command: String = String::new();
    println!("Please select the command: ");
    stdin().read_line(&mut column_command).expect(INPUT_ERROR);
    let parsed_command = column_command.trim().parse::<usize>().expect(INPUT_ERROR);
    let selected_command = commands.get(parsed_command).expect(INPUT_ERROR);

    return (columns_name.clone(), selected_command.clone());
}
