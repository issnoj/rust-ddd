use std::cell::RefCell;
use std::collections::HashMap;
use chrono::Local;
use ulid::Ulid;
use crate::domain::entity::book_circulation::BookCirculation;

#[derive(Clone)]
pub struct BookCirculationRepository {
    book_circulations: RefCell<HashMap<String, BookCirculation>>,
}

impl BookCirculationRepository {
    pub fn new() -> Self {
        let users: HashMap<String, BookCirculation> = HashMap::new();

        Self {
            book_circulations: RefCell::new(users),
        }
    }

    pub fn create(&self, user_id: Ulid, book_id: Ulid) -> String {
        let borrow_date = Local::now();
        let book_circulation = BookCirculation::create(user_id, book_id, borrow_date);
        let key = book_circulation.id.to_string().clone();
        self.book_circulations.borrow_mut().insert(key, book_circulation.clone());
        book_circulation.id.to_string()
    }

    pub fn update(&self, book_circulation: BookCirculation) {
        self.book_circulations.borrow_mut()
            .insert(book_circulation.id.to_string(), book_circulation);
    }

    pub fn borrowing_count(&self, user_id: Ulid) -> u32 {
        let book_circulations = self.book_circulations.borrow();
        book_circulations.values()
            .filter(|&v| v.user_id == user_id && v.return_date == None)
            .collect::<Vec<&BookCirculation>>().len() as u32
    }

    pub fn get_by_user_and_book(&self, user_id: Ulid, book_id: Ulid)
                                -> Result<BookCirculation, String>
    {
        let book_circulation = self.book_circulations.borrow()
            .values()
            .find(|&v| {
                v.user_id == user_id && v.book_id == book_id
            })
            .cloned();

        match book_circulation {
            Some(c) => Ok(c),
            None => Err(String::from("No user found for given ID")),
        }
    }

    pub fn get_borrowing_books_by_user(&self, user_id: Ulid) -> Vec<BookCirculation> {
        let book_circulations = self.book_circulations.borrow();
        let book_circulations = book_circulations
            .values()
            .filter(|v| v.user_id == user_id)
            .collect::<Vec<&BookCirculation>>();

        let mut result = vec!();

        for book_circulation in book_circulations {
            if book_circulation.user_id != user_id {
                continue;
            }

            if book_circulation.return_date != None {
                continue;
            }

            result.push(book_circulation.clone());
        }

        result
    }
}