#[derive(Clone)]
enum Genre {
    Fiction,
    NonFiction,
    SciFi,
    Fantasy,
    Religion,
}

#[derive(Clone)]
struct Book {
    title: String,
    author: String,
    genre: Genre,
    availability: bool
}

impl Book {
    fn new(title: String, author: String, genre: Genre) -> Book {
        Book {
            title:title, 
            author:author, 
            genre:genre, 
            availability:true,
        }
    }

    fn check_in(&mut self) {
        match self.availability {
            true => { 
                println!("{} is already checked in.", self.title)
            }
            false => {
                println!("{} has been checked in.", self.title);
                self.availability = true;
            }
        }
    }

    fn check_out(&mut self) {
        match self.availability {
            true => {
                self.availability = false;
                println!("{} has been checked out.", self.title)
            }
            false => println!("{} is already checked out.", self.title)
        }
    }
}

fn get_books() -> Vec<Book> {
    let mut book1 = Book::new(
        String::from("To Kill a Mockingbird"), 
        String::from("Harper Lee"), 
        Genre::Fiction, 
        );

    let mut book2 = Book::new(
        String::from("1984"), 
        String::from("George Orwell"), 
        Genre::SciFi, 
        );

    let mut book3 = Book::new(
        String::from("The Hobbit"), 
        String::from("J.R.R. Tolkien"), 
        Genre::Fantasy, 
        );

    let mut book4 = Book::new(
        String::from("The Great Gatsby"), 
        String::from("F. Scott Fitzgerald"), 
        Genre::Fiction, 
        );

    let mut book5 = Book::new(
        String::from("A Brief History of Time"), 
        String::from("Stephen Hawking"), 
        Genre::NonFiction, 
        );

    let mut book6: Book = Book::new (
        String::from("Bible"),
        String::from("Jesus"),
        Genre::Religion,
    );
    return vec![book1, book2, book3, book4, book5, book6];
}

struct Library{books:Vec<Book>}

impl Library {
    fn new(book:Vec<Book>) -> Library { 
        Library{books:book}
    }

    fn add_book(&mut self, book:Book) {
        println!("New book {} was added to the library.", book.title);
        self.books.push(book)
    }

    fn add_books(&mut self, vec_books:Vec<Book>) {
        println!("Extracting book(s)");
        for book in vec_books {
            println!("Adding {} to the library.", book.title);
            self.books.push(book)
        }
        println!("Extraction was successful, all book(s) added to the library.")
    }

    fn remove_book(&mut self, book_title: String) {
        let book_index = {
            self.books
                .iter()
                .position(|lib_book| lib_book.title==book_title)
        };

        match book_index {
             Some(index) => {
                let bt = self.books[index].title.clone();
                self.books.remove(index);
                println!("{} has been removed.", bt);
            },
            _ => println!("{} does not exist.", book_title)
        }
    }
    
    fn check_in_book(&mut self, book_title: String) {
        let book_index = {
            self.books
                .iter()
                .position(|lib_book| lib_book.title==book_title)
        };

        match book_index {
             Some(index) => {
                 self.books[index].check_in();
            },
            _ => println!("{} does not exist.", book_title)
        }
    }

    fn check_out_book(&mut self, book_title: String) {
        let book_index = {
            self.books
                .iter()
                .position(|lib_book| lib_book.title==book_title)
        };

        match book_index {
             Some(index) => {
                 self.books[index].check_out();
            },
            _ => println!("{} does not exist.", book_title)
        }
    }
}

fn main() {
    // Generating sample books
    let books = get_books();

    // Vec slicing to get a sample book(s) that will be used as a test
    let last_index = books.len();
    let second_last_index = last_index - 1;
    let library_books: Vec<Book> = (&books[0..second_last_index]).into();
    let test_books = books[second_last_index..last_index].to_vec();

    // Building the library
    let mut alexandria_library = Library::new(library_books);

    // Adding one test book
    let book7 = Book::new(
        String::from("The Catcher in the Rye"), 
        String::from("J.D. Salinger"), 
        Genre::Fiction, 
    );
    alexandria_library.add_book(book7);

    // Adding test vec books
    alexandria_library.add_books(test_books);

    // Test book
    let bible = String::from("Bible");

    // Attempting to check out a book 2 times
    alexandria_library.check_out_book(bible.clone());
    alexandria_library.check_out_book(bible.clone());

    // Attempting to check in the book 2 times
    alexandria_library.check_in_book(bible.clone());
    alexandria_library.check_in_book(bible.clone());

    // Attempting to remove the book 2 times
    alexandria_library.remove_book(bible.clone());
    alexandria_library.remove_book(bible.clone());
}
