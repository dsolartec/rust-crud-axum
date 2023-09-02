use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{modules::users::UsersService, internal::apperror::AppError};

use super::{PermissionsService, models::{Permission, CreatePermissionBody}};

#[derive(Clone)]
pub struct PermissionsController {
    users_service: Arc<RwLock<UsersService>>,
    permissions_service: Arc<RwLock<PermissionsService>>,
}

impl PermissionsController {
    pub fn new(
        users_service: &Arc<RwLock<UsersService>>,
        permissions_service: &Arc<RwLock<PermissionsService>>,
    ) -> PermissionsController {
        PermissionsController {
            users_service: users_service.clone(),
            permissions_service: permissions_service.clone(),
        }
    }

    pub async fn create(&self, data: CreatePermissionBody) -> Result<Permission, AppError> {
        self.permissions_service.write().await.create(&data.name, &data.description)
    }

    pub async fn get_all(&self) -> Vec<Permission> {
        self.permissions_service.read().await.get_all()
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Permission, AppError> {
        match self.permissions_service.read().await.get_by_id(id) {
            Some(permission) => Ok(permission.clone()),
            None => Err(AppError::PermissionNotFound),
        }
    }

    pub async fn get_by_name(&self, permission_name: String) -> Result<Permission, AppError> {
        match self.permissions_service.read().await.get_by_name(&permission_name) {
            Some(permission) => Ok(permission.clone()),
            None => Err(AppError::PermissionNotFound),
        }
    }

    pub async fn delete(&self, permission_name: String) -> Result<(), AppError> {
        self.permissions_service.write().await.delete(&permission_name)
    }

    pub async fn get_permissions_for_user(&self, username: String) -> Result<Vec<String>, AppError> {
        let users_service_read = self.users_service.read().await;
        
        let user = users_service_read.get_by_username(&username);
        if user.is_none() {
            return Err(AppError::UserNotFound);
        }

        let user = user.unwrap();
        let permissions_service_read = self.permissions_service.read().await;

        let permissions = permissions_service_read
            .clone()
            .get_permissions_for_user(user.id)
            .into_iter()
            .map(|user_permission| permissions_service_read.get_by_id(user_permission.permission_id))
            .filter(|permission| permission.is_some())
            .map(|permission| permission.unwrap().name.clone())
            .collect::<Vec<String>>();

        Ok(permissions)
    }

    pub async fn grant_permission(&self, username: String, permission_name: String) -> Result<(), AppError> {
        let users_service_read = self.users_service.read().await;
        
        let user = users_service_read.get_by_username(&username);
        if user.is_none() {
            return Err(AppError::UserNotFound);
        }

        self.permissions_service.write().await.grant_permission(user.unwrap().id, &permission_name)
    }

    pub async fn revoke_permission(&self, username: String, permission_name: String) -> Result<(), AppError> {
        let users_service_read = self.users_service.read().await;
        
        let user = users_service_read.get_by_username(&username);
        if user.is_none() {
            return Err(AppError::UserNotFound);
        }

        self.permissions_service.write().await.revoke_permission(user.unwrap().id, &permission_name)
    }
}
