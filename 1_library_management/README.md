# Rust Library Management System Challenge

This challenge aims to provide you with an opportunity to practice concepts such as variables, data types, functions, control flow, enums, structs, and methods using the Rust programming language. You will be creating a simplified library management system.

## Overview

You'll be designing a simple library management system capable of tracking the books in the library and handling check-ins and check-outs.

## Challenge Requirements

1. **Create an Enum `Genre`**: The enum should represent the genre of a book. Some examples can be: Fiction, NonFiction, SciFi, Biography, etc. 

2. **Create a Struct `Book`**: The struct should represent a book. Each book should have a title (String), author (String), genre (from your enum), and availability (boolean).

3. **Implement Methods for `Book`**: Implement the following methods for `Book`:
    - `new(title: String, author: String, genre: Genre) -> Book`: This method should construct a new Book.
    - `check_out()`: This method should change the availability of the book to false, signifying that the book has been checked out.
    - `check_in()`: This method should change the availability of the book back to true, signifying that the book has been returned.

4. **Create a Struct `Library`**: The struct should represent your library. It should have one field, `books`, which is a Vector of Books.

5. **Implement Methods for `Library`**: Implement the following methods for `Library`:
    - `new() -> Library`: This method should construct a new Library.
    - `add_book(book: Book)`: This method should add a book to the library's books.
    - `remove_book(title: String)`: This method should find and remove a book from the library's books. If the book is not found, print a message saying so.
    - `check_out_book(title: String)`: This method should find a book in the library's books and call the `check_out` method on it. If the book is not found, or it is already checked out, print an appropriate message.
    - `check_in_book(title: String)`: This method should find a book in the library's books and call the `check_in` method on it. If the book is not found, or it is not currently checked out, print an appropriate message.

## Test Your Solution

Once you have implemented these structs and methods, create a main function that does the following:

- Creates a few books with different titles, authors, and genres.
- Adds these books to the library.
- Checks out a book.
- Attempts to check out the same book (this should fail and print an appropriate message).
- Checks the book back in.
- Attempts to check in the book again (this should fail and print an appropriate message).
- Removes a book from the library.
- Attempts to remove the same book again (this should fail and print an appropriate message).

## Conclusion

This challenge provides comprehensive practice of key Rust concepts. Enjoy coding!
