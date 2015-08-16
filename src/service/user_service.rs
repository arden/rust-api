use mysql::conn::pool::MyPool;
use mysql::value::from_row;
use std::sync::Arc;

use ::model::user::*;

pub struct UserService {
    db_pool: Arc<MyPool>
}

impl UserService {
    pub fn default(db_pool: Arc<MyPool>) -> UserService {
        UserService {
            db_pool: db_pool,
        }
    }

    pub fn new(db_pool: Arc<MyPool>) -> UserService {
        Self::default(db_pool)
    }

    pub fn get_users(&mut self) -> Vec<User> {
        let users: Vec<User> =  self.db_pool.prep_exec("SELECT id, email from t_users", ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (id, email) = from_row(row);
                User {
                    id: id,
                    email: email,
                    ..Default::default()
                }
            }).collect()
        }).unwrap();
        return users;
    }
}
