use std::sync::Arc;

use tokio::sync::RwLock;

use crate::internal::apperror::AppError;

use super::{UsersService, models::User};

#[derive(Clone)]
pub struct UsersController {
    users_service: Arc<RwLock<UsersService>>,
}

impl UsersController {
    pub fn new(users_service: &Arc<RwLock<UsersService>>) -> UsersController {
        UsersController {
            users_service: users_service.clone(),
        }
    }

    pub async fn get_all(&self) -> Vec<User> {
        self.users_service.read().await.get_all()
    }

    pub async fn get_by_id(&self, id: i64) -> Result<User, AppError> {
        match self.users_service.read().await.get_by_id(id) {
            Some(user) => Ok(user.clone()),
            None => Err(AppError::UserNotFound),
        }
    }

    pub async fn get_by_username(&self, username: String) -> Result<User, AppError> {
        match self.users_service.read().await.get_by_username(&username) {
            Some(user) => Ok(user.clone()),
            None => Err(AppError::UserNotFound),
        }
    }

    pub async fn delete(&self, username: String) -> Result<(), AppError> {
        self.users_service.write().await.delete(&username)
    }
}
