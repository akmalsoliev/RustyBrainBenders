# Descriptive Dimensions: Traits and Generics in Rust

## Challenge

### Objective:

Create a trait named `Information` that will have a method named `info` which returns a String. Implement the `Information` trait for the following types:

1. i32
2. &str
3. A custom type `Point<T>`

The `Point<T>` type should represent a point in a 2D space with x and y coordinates, where T is a generic type that implements the `Copy` trait.

### Expected Output:

1. For `i32`, the `info` method should return a string saying: "This is an integer: [value]". Replace `[value]` with the integer's value.
2. For `&str`, the `info` method should return: "This is a string: '[value]'". Replace `[value]` with the string's value.
3. For `Point<T>`, the `info` method should return: "This is a point at (x, y)", replacing `x` and `y` with the coordinates of the point.

### Bonus:

Implement the `Add` trait for `Point<T>` to allow two points to be added together, resulting in a new point.

### Sample Usage:

```rust
let number = 5;
println!("{}", number.info());

let text = "Rust";
println!("{}", text.info());

let point = Point { x: 3.0, y: 4.0 };
println!("{}", point.info());

let another_point = Point { x: 1.0, y: 2.0 };
let sum_point = point + another_point;
println!("{}", sum_point.info());
```

### Expected Output:

```
This is an integer: 5
This is a string: 'Rust'
This is a point at (3, 4)
This is a point at (4, 6)
```

**Note:** The expected output for the `sum_point` assumes you've implemented the `Add` trait as part of the bonus.

Happy coding!
