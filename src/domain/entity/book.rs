use ulid::Ulid;
use crate::domain::value_object::book_status::BookStatus;

#[derive(Debug, Clone)]
pub struct Book {
    pub id: Ulid,
    pub title: String,
    pub status: BookStatus,
}

impl Book {
    fn new(id: Ulid, title: &str, status: BookStatus) -> Self {
        Self {
            id,
            title: title.to_string(),
            status,
        }
    }

    pub fn create(title: &str) -> Self {
        Self::new(Ulid::new(), title, BookStatus::AvailableForBorrow)
    }

    pub fn can_borrow(&self) -> bool {
        self.status == BookStatus::AvailableForBorrow
    }

    pub fn borrow(&mut self) {
        self.status = BookStatus::Borrowing;
    }

    pub fn return_book(&mut self) {
        self.status = BookStatus::AvailableForBorrow;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        // given
        let title = "title";

        // when
        let book = Book::create(title);

        // then
        assert_eq!(book.id.to_string().len(), 26);
        assert_eq!(book.title, title);
        assert_eq!(book.status, BookStatus::AvailableForBorrow);
    }
}