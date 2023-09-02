use crate::internal::{platform::logger::Logger, apperror::AppError};

use super::models::User;

#[derive(Clone)]
pub struct UsersService {
    logger: Logger,
    users: Vec<User>,
}

impl UsersService {
    pub fn new(logger: &Logger) -> UsersService {
        UsersService {
            logger: logger.clone(),
            users: vec![
                User {
                    id: 1,
                    username: String::from("admin"),
                    password: String::from("admin"),
                },
                User {
                    id: 2,
                    username: String::from("dsolarte"),
                    password: String::from("1234"),
                },
            ],
        }
    }

    pub fn create(&mut self, username: &str, password: &str) -> Result<User, AppError> {
        let mut last_id = 0_i64;
        for user in self.users.clone() {
            if user.username.to_lowercase() == username.to_lowercase() {
                return Err(AppError::UserAlreadyExists);
            }

            if user.id > last_id {
                last_id = user.id;
            }
        }

        let user = User {
            id: last_id + 1,
            username: username.to_string(),
            password: password.to_string(),
        };

        self.users.push(user.clone());
        self.logger.info(format!("[UsersService] New user created {}!", username));

        Ok(user)
    }

    pub fn get_by_id(&self, id: i64) -> Option<&User> {
        self.users.iter().find(|user| user.id == id)
    }

    pub fn get_by_username(&self, username: &str) -> Option<&User> {
        self.users.iter().find(|user| user.username.to_lowercase() == username.to_lowercase())
    }

    pub fn get_all(&self) -> Vec<User> {
        self.users.clone()
    }

    pub fn delete(&mut self, username: &str) -> Result<(), AppError> {
        let index = self.users.iter().position(|user| user.username.to_lowercase() == username.to_lowercase());
        if index.is_none() {
            return Err(AppError::UserNotFound);
        }

        self.users.remove(index.unwrap());

        Ok(())
    }
}
