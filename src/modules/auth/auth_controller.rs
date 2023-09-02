use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{modules::{users::UsersService, permissions::PermissionsService}, internal::{apperror::AppError, platform::encryption::{JwtEncryption, AccessTokenPayload}}};

use super::models::{LogInBody, AuthResponse, SignUpBody};

#[derive(Clone)]
pub struct AuthController {
    jwt_encryption: JwtEncryption,
    users_service: Arc<RwLock<UsersService>>,
    permissions_service: Arc<RwLock<PermissionsService>>,
}

impl AuthController {
    pub fn new(
        jwt_encryption: &JwtEncryption,
        users_service: &Arc<RwLock<UsersService>>,
        permissions_service: &Arc<RwLock<PermissionsService>>,
    ) -> AuthController {
        AuthController {
            jwt_encryption: jwt_encryption.clone(),
            users_service: users_service.clone(),
            permissions_service: permissions_service.clone(),
        }
    }

    pub async fn log_in(&self, data: LogInBody) -> Result<AuthResponse, AppError> {
        let users_service_read = self.users_service.read().await;

        let user = users_service_read.get_by_username(&data.username);
        if user.is_none() {
            return Err(AppError::UserWrongAuthentication);
        }

        let user = user.unwrap();
        if user.password != data.password {
            return Err(AppError::UserWrongAuthentication);
        }

        let permissions_service_read = self.permissions_service.read().await;

        let permissions = permissions_service_read
            .clone()
            .get_permissions_for_user(user.id)
            .into_iter()
            .map(|user_permission| permissions_service_read.get_by_id(user_permission.permission_id))
            .filter(|permission| permission.is_some())
            .map(|permission| permission.unwrap().name.clone())
            .collect::<Vec<String>>();

        let access_token = self.jwt_encryption.generate_access_token(AccessTokenPayload::new(user.id, &user.username, permissions))?;

        Ok(AuthResponse { access_token })
    }

    pub async fn sign_up(&mut self, data: SignUpBody) -> Result<AuthResponse, AppError> {
        let mut users_service_write = self.users_service.write().await;
        let user = users_service_write.create(&data.username, &data.password)?;
        
        let permissions_service_read = self.permissions_service.read().await;

        let permissions = permissions_service_read
            .clone()
            .get_permissions_for_user(user.id)
            .into_iter()
            .map(|user_permission| permissions_service_read.get_by_id(user_permission.permission_id))
            .filter(|permission| permission.is_some())
            .map(|permission| permission.unwrap().name.clone())
            .collect::<Vec<String>>();

        let access_token = self.jwt_encryption.generate_access_token(AccessTokenPayload::new(user.id, &user.username, permissions))?;

        Ok(AuthResponse { access_token })
    }
}
