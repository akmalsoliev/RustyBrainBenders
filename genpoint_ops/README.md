# Challenge: Generics with Points

**Objective**: Create a generic struct that can represent points in multiple dimensions using Rust's generic types.

## Steps:

1. **Generic Point Struct**:
    - Define a struct named `Point<T>` that has two fields, `x` and `y`, both of type `T`.
    
2. **Generic Method for Point Struct**:
    - Implement a method `distance_from_origin(&self) -> f64` for the `Point<T>` struct that calculates the Euclidean distance from the origin. Remember that for the computation to be valid, T should be a type that supports mathematical operations. You might want to use trait bounds for this.

3. **Multiple Dimensions**:
    - Extend your `Point` struct to `Point<T, U>` where `x` is of type `T` and `y` is of type `U`. Modify the `distance_from_origin` method to still compute the distance when `x` and `y` are of different types. This will involve type conversions.
    
4. **Generic Functions**:
    - Write a generic function `swap_coordinates<T>(point: Point<T, T>) -> Point<T, T>` that swaps the `x` and `y` coordinates of a point.

5. **Trait Implementation**:
    - Implement a trait `DisplayPoint` which provides a method `display(&self)`.
    - Implement this trait for several variations of the `Point` struct (e.g., `Point<i32, i32>`, `Point<f64, f64>`, etc.) to print the coordinates in a format like "(x, y)".

6. **Test Your Code**:
    - Create various instances of `Point` with different type parameters (e.g., `i32`, `f64`, etc.).
    - Use the `swap_coordinates` function to swap their coordinates.
    - Display the original and swapped coordinates using the `display` method.
    - Compute the distance from the origin for each point and print the result.

## Bonus:

1. **Option Generic**:
    - Create a function `midpoint<T>(point1: Option<Point<T, T>>, point2: Option<Point<T, T>>) -> Option<Point<T, T>>` that takes two Option-wrapped points and returns the midpoint. The function should return `None` if either of the input points is `None`.

2. **Error Handling**:
    - Modify the `distance_from_origin` method to return a `Result<f64, String>` instead of just `f64`. If the type doesn't support the required mathematical operation, return a descriptive error message.

By the end of this challenge, you'll have utilized generic types, trait bounds, associated functions, and more from Rust's powerful type system. Remember to use the documentation and Rust compiler messages to guide you. Happy coding!
