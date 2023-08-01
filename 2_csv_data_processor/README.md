# Rust CSV Data Processor

This is a simple command-line-based CSV data processor implemented in Rust. The main aim of this project is to demonstrate the understanding of various concepts like enums, structs, methods, ownership, borrowing, slices, and more in Rust.

## Features

This data processor allows the user to:

- Load a CSV file into memory
- Perform operations like filter, sort and count on the data
- Save the processed data back to a new CSV file

## Implementation

The high-level implementation of the data processor involves:

1. A Struct `DataRecord` that has the fields matching your CSV columns.
2. A Struct `DataSet` that contains a Vector of `DataRecord` and the filename.
3. An Enum `Operation` that includes `Load`, `Filter`, `Sort`, `Count`, `Save`, and `Exit`.
4. Methods on `DataSet` corresponding to each operation.

In the `main` function, create a `DataSet` instance. In a loop, prompt the user to enter a command, match that command to one of your `Operation` enum variants, and call the corresponding `DataSet` method. If the user enters "exit", break the loop.

## Understanding Ownership and Borrowing

Ensure that you're not cloning data unnecessarily and that you're not creating any borrowing conflicts. Pay attention to how ownership and borrowing apply to more complex types like `Vec<DataRecord>`. Apply your understanding of slices when working with your `Vec<DataRecord>`, especially when you're implementing the `filter` and `sort` operations.

Ensure you understand and correctly implement the borrowing rules in Rust: Values can be borrowed immutably any number of times, but while something is borrowed immutably, the original data can't

