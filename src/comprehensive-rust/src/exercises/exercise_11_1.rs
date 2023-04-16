use std::cmp::min;

struct Library {
    books: Vec<Book>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Book {
    year: u16,
    title: String,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// This makes it possible to print Book values with {}.
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}

impl Library {
    fn new() -> Library {
        Library { books: vec![] }
    }

    #[allow(dead_code)]
    fn with_books(books: &Vec<Book>) -> Library {
        Library {
            books: books.clone(),
        }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn print_books(&self) {
        println!("{:?}", self.books);
    }

    fn oldest_book(&self) -> Option<&Book> {
        self.books.iter().reduce(min)
    }
}

// This shows the desired behavior. Uncomment the code below and
// implement the missing methods. You will need to update the
// method signatures, including the "self" parameter! You may
// also need to update the variable bindings within main.
#[test]
fn test() {
    let mut library = Library::new();

    assert!(library.is_empty(), "Our library is empty");

    let lord_of_the_rings = Book::new("Lord of the Rings", 1954);
    library.add_book(lord_of_the_rings.clone());

    let alice_in_wonderland = Book::new("Alice's Adventures in Wonderland", 1865);
    library.add_book(alice_in_wonderland.clone());

    library.print_books();
    assert!(library.books == vec![lord_of_the_rings.clone(), alice_in_wonderland.clone()]);

    let oldest_book = library.oldest_book();
    assert!(oldest_book.is_some(), "Library has oldest book");

    let oldest_book = oldest_book.unwrap();
    assert!(
        oldest_book.eq(&alice_in_wonderland),
        "Oldest book of library is 'Alice's Adventures in Wonderland'"
    );

    assert!(library.len() == 2, "Library has two books");

    for book in library.books {
        println!("{book}");
    }
}
