use std::cell::RefCell;
use std::collections::HashMap;
use crate::domain::entity::book::Book;

#[derive(Clone)]
pub struct BookRepository {
    books: RefCell<HashMap<String, Book>>,
}

impl BookRepository {
    pub fn new() -> Self {
        let users: HashMap<String, Book> = HashMap::new();

        Self {
            books: RefCell::new(users),
        }
    }

    pub fn create(&self, title: &str) -> String {
        let book = Book::create(title);
        let key = book.id.to_string().clone();
        self.books.borrow_mut().insert(key, book.clone());
        book.id.to_string()
    }

    pub fn update(&self, book: Book) {
        self.books.borrow_mut().insert(book.id.to_string(), book);
    }

    pub fn get_by_id(&self, id: &str) -> Result<Book, String> {
        let book = self.books.borrow().get(id).cloned();

        match book {
            Some(c) => Ok(c),
            None => Err(String::from("No book found for given ID")),
        }
    }
}