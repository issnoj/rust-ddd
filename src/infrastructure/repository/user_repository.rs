use std::cell::RefCell;
use std::collections::HashMap;
use crate::domain::entity::user::User;

#[derive(Clone)]
pub struct UserRepository {
    users: RefCell<HashMap<String, User>>,
}

impl UserRepository {
    pub fn new() -> Self {
        let users: HashMap<String, User> = HashMap::new();

        Self {
            users: RefCell::new(users),
        }
    }

    pub fn create(&self, name: &str) -> String {
        let user = User::create(name);
        let key = user.id.to_string().clone();
        self.users.borrow_mut().insert(key, user.clone());
        user.id.to_string()
    }

    pub fn update(&self, user: User) {
        self.users.borrow_mut().insert(user.id.to_string(), user);
    }

    pub fn get_by_id(&self, id: &str) -> Result<User, String> {
        let user = self.users.borrow().get(id).cloned();

        match user {
            Some(c) => Ok(c),
            None => Err(String::from("No user found for given ID")),
        }
    }
}