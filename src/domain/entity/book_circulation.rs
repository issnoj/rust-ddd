use chrono::{DateTime, Local};
use ulid::Ulid;

#[derive(Debug, Clone)]
pub struct BookCirculation {
    pub id: Ulid,
    pub user_id: Ulid,
    pub book_id: Ulid,
    pub borrow_date: Option<DateTime<Local>>,
    pub return_date: Option<DateTime<Local>>,
}

impl BookCirculation {
    fn new(id: Ulid, user_id: Ulid, book_id: Ulid, borrow_date: DateTime<Local>) -> Self {
        Self {
            id,
            user_id,
            book_id,
            borrow_date: Some(borrow_date),
            return_date: None,
        }
    }

    pub fn create(user_id: Ulid, book_id: Ulid, borrow_date: DateTime<Local>) -> Self {
        Self::new(Ulid::new(), user_id, book_id, borrow_date)
    }

    pub fn set_return_date(&mut self, date: DateTime<Local>) {
        if self.borrow_date.unwrap().gt(&date) {
            panic!("日付が正しくありません");
        }

        self.return_date = Some(date);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        // given
        let user_id = Ulid::new();
        let book_id = Ulid::new();
        let borrow_date = Local::now();

        // when
        let book = BookCirculation::create(user_id, book_id, borrow_date);

        // then
        assert_eq!(book.id.to_string().len(), 26);
        assert_eq!(book.user_id, user_id);
        assert_eq!(book.book_id, book_id);
        assert_eq!(book.borrow_date.unwrap(), borrow_date);
        assert_eq!(book.return_date, None);
    }
}