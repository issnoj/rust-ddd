use std::collections::HashMap;
use std::rc::Rc;
use crate::infrastructure::repository::book_circulation_repository::BookCirculationRepository;
use crate::infrastructure::repository::book_repository::BookRepository;
use crate::infrastructure::repository::user_repository::UserRepository;

pub struct GetBorrowingBookListQuery {
    user_repository: Rc<UserRepository>,
    book_repository: Rc<BookRepository>,
    book_circulation_repository: Rc<BookCirculationRepository>,
}

impl GetBorrowingBookListQuery {
    pub fn new(
        user_repository: Rc<UserRepository>,
        book_repository: Rc<BookRepository>,
        book_circulation_repository: Rc<BookCirculationRepository>,
    ) -> Self {
        Self {
            user_repository,
            book_repository,
            book_circulation_repository,
        }
    }

    pub fn execute(&self, user_id: &str) -> Vec<HashMap<&'static str, String>> {
        let user = self.user_repository.get_by_id(user_id).unwrap();
        let borrowing_books = self.book_circulation_repository.get_borrowing_books_by_user(user.id);

        let mut result = vec!();

        for borrowing_book in borrowing_books {
            let mut map = HashMap::new();

            let book = self.book_repository.get_by_id(borrowing_book.book_id.to_string().as_str()
            ).unwrap();

            map.insert("id", borrowing_book.book_id.to_string());
            map.insert("title", book.title);
            map.insert("borrowDate", borrowing_book.borrow_date.unwrap()
                .format("%Y-%m-%d %H:%M:%S").to_string());

            result.push(map);
        }

        result
    }
}