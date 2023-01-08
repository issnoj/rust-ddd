use std::panic::RefUnwindSafe;
use std::rc::Rc;
use chrono::{DateTime, Local};
use crate::domain::entity::user::User;
use crate::infrastructure::repository::book_circulation_repository::BookCirculationRepository;
use crate::infrastructure::repository::book_repository::BookRepository;

pub struct ReturnBookUseCase {
    book_repository: Rc<BookRepository>,
    book_circulation_repository: Rc<BookCirculationRepository>,
}

impl ReturnBookUseCase {
    pub fn new(
        book_repository: Rc<BookRepository>,
        book_circulation_repository: Rc<BookCirculationRepository>,
    ) -> Self {
        Self {
            book_repository,
            book_circulation_repository,
        }
    }

    pub fn execute(&self, user: User, book_id: String, date: DateTime<Local>) {
        let mut book = self.book_repository.get_by_id(book_id.as_str()).unwrap();

        if book.can_borrow() {
            panic!("この本は貸出中ではありません");
        }

        let mut book_circulation = self.book_circulation_repository.get_by_user_and_book(
            user.id,
            book.id,
        ).unwrap();

        // if !book_circulation {
        //     panic!("この本は別の人が借りています");
        // }

        book.return_book();
        book_circulation.set_return_date(date);

        self.book_repository.update(book.clone());
        self.book_circulation_repository.update(book_circulation);

        println!("{}を返却しました", book.title);
    }
}

impl RefUnwindSafe for ReturnBookUseCase {}