use std::panic::RefUnwindSafe;
use std::rc::Rc;
use crate::domain::entity::user::User;
use crate::infrastructure::repository::book_circulation_repository::BookCirculationRepository;
use crate::infrastructure::repository::book_repository::BookRepository;

pub struct BorrowBookUseCase {
    book_repository: Rc<BookRepository>,
    book_circulation_repository: Rc<BookCirculationRepository>,
}

impl BorrowBookUseCase {
    pub fn new(
        book_repository: Rc<BookRepository>,
        book_circulation_repository: Rc<BookCirculationRepository>,
    ) -> Self {
        Self {
            book_repository,
            book_circulation_repository,
        }
    }

    pub fn execute(&self, user: User, book_id: String) {
        let mut book = self.book_repository.get_by_id(book_id.as_str()).unwrap();

        if !book.can_borrow() {
            panic!("貸出中です");
        }

        let max_borrowing_count = user.user_plan.plan.max_borrowing_count();

        let now_borrowing_count = self.book_circulation_repository.borrowing_count(user.id);

        if now_borrowing_count >= max_borrowing_count {
            panic!("{}では{}冊以上貸出できません", user.user_plan.plan.name(), max_borrowing_count);
        }

        book.borrow();

        self.book_repository.update(book.clone());
        self.book_circulation_repository.create(user.id, book.id.clone());

        println!("{}を借りました", book.title);
    }
}

impl RefUnwindSafe for BorrowBookUseCase {}