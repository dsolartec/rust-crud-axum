use crate::internal::{platform::logger::Logger, apperror::AppError};

use super::models::{Permission, UserPermission};

#[derive(Clone)]
pub struct PermissionsService {
    logger: Logger,
    permissions: Vec<Permission>,
    user_permissions: Vec<UserPermission>,
}

impl PermissionsService {
    pub fn new(logger: &Logger) -> PermissionsService {
        PermissionsService {
            logger: logger.clone(),
            permissions: vec![
                Permission {
                    id: 1,
                    name: String::from("users_full"),
                    description: String::from("Full access to users endpoints"),
                    deletable: false,
                },
                Permission {
                    id: 2,
                    name: String::from("users_read"),
                    description: String::from("Only access to users GET endpoints"),
                    deletable: false,
                },
                Permission {
                    id: 3,
                    name: String::from("users_write"),
                    description: String::from("Only access to users POST, PUT and DELETE endpoints"),
                    deletable: false,
                },
                Permission {
                    id: 4,
                    name: String::from("permissions_full"),
                    description: String::from("Full access to permissions endpoints"),
                    deletable: false,
                },
                Permission {
                    id: 5,
                    name: String::from("permissions_read"),
                    description: String::from("Only access to permissions GET endpoints"),
                    deletable: false,
                },
                Permission {
                    id: 6,
                    name: String::from("permissions_write"),
                    description: String::from("Only access to permissions POST, PUT and DELETE endpoints"),
                    deletable: false,
                },
                Permission {
                    id: 7,
                    name: String::from("grant_permission"),
                    description: String::from("Grant a permission to an user"),
                    deletable: false,
                },
                Permission {
                    id: 8,
                    name: String::from("revoke_permission"),
                    description: String::from("Revoke a permission to an user"),
                    deletable: false,
                },
            ],
            user_permissions: vec![
                UserPermission {
                    user_id: 1,
                    permission_id: 1,
                },
                UserPermission {
                    user_id: 1,
                    permission_id: 4,
                },
                UserPermission {
                    user_id: 1,
                    permission_id: 7,
                },
                UserPermission {
                    user_id: 1,
                    permission_id: 8,
                },
                UserPermission {
                    user_id: 2,
                    permission_id: 2,
                },
                UserPermission {
                    user_id: 2,
                    permission_id: 5,
                },
            ],
        }
    }

    pub fn create(&mut self, name: &str, description: &str) -> Result<Permission, AppError> {
        let mut last_id = 0_i64;
        for permission in self.permissions.clone() {
            if permission.name.to_lowercase() == name.to_lowercase() {
                return Err(AppError::PermissionAlreadyExists);
            }

            if permission.id > last_id {
                last_id = permission.id;
            }
        }

        let permission = Permission {
            id: last_id + 1,
            name: name.to_string(),
            description: description.to_string(),
            deletable: true,
        };

        self.permissions.push(permission.clone());
        self.logger.info(format!("[PermissionsService] New permission created {}!", name));

        Ok(permission)
    }

    pub fn get_by_id(&self, id: i64) -> Option<&Permission> {
        self.permissions.iter().find(|permission| permission.id == id)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Permission> {
        self.permissions.iter().find(|permission| permission.name.to_lowercase() == name.to_lowercase())
    }

    pub fn get_all(&self) -> Vec<Permission> {
        self.permissions.clone()
    }

    pub fn delete(&mut self, name: &str) -> Result<(), AppError> {
        let index = self.permissions.iter().position(|permission| permission.name.to_lowercase() == name.to_lowercase());
        if index.is_none() {
            return Err(AppError::PermissionNotFound);
        }

        let index = index.unwrap();

        let permission = self.permissions[index].clone();
        if !permission.deletable {
            return Err(AppError::PermissionNotDeletable);
        }

        self.permissions.remove(index);

        let mut user_permissions = self.user_permissions.clone().into_iter().enumerate();
        
        let mut up = user_permissions.next();
        while up.is_some() {
            let (index, user_permission) = up.unwrap();

            if user_permission.permission_id == permission.id {
                self.user_permissions.remove(index);
            }

            up = user_permissions.next();
        }

        Ok(())
    }

    pub fn get_permissions_for_user(self, user_id: i64) -> Vec<UserPermission> {
        self.user_permissions
            .clone()
            .into_iter()
            .filter(|user_permission| user_permission.user_id == user_id)
            .collect()
    }

    pub fn user_has_permission(&self, user_id: i64, permission_id: i64) -> bool {
        self.user_permissions
            .clone()
            .into_iter()
            .find(|user_permission| user_permission.user_id == user_id && user_permission.permission_id == permission_id)
            .is_some()
    }

    pub fn grant_permission(&mut self, user_id: i64, permission_name: &str) -> Result<(), AppError> {
        let permission = self.get_by_name(permission_name);
        if permission.is_none() {
            return Err(AppError::PermissionNotFound);
        }

        let permission = permission.unwrap();
        if self.user_has_permission(user_id, permission.id) {
            return Err(AppError::UserAlreadyHasPermission);
        }

        self.user_permissions.push(UserPermission {
            user_id,
            permission_id: permission.id,
        });

        self.logger.info(format!("[PermissionsService] Permission '{}' granted to '{}' user!", permission_name, user_id));

        Ok(())
    }

    pub fn revoke_permission(&mut self, user_id: i64, permission_name: &str) -> Result<(), AppError> {
        let permission = self.get_by_name(permission_name);
        if permission.is_none() {
            return Err(AppError::PermissionNotFound);
        }

        let permission = permission.unwrap();
        let index = self.user_permissions.iter().position(|user_permission| user_permission.permission_id == permission.id && user_permission.user_id == user_id);

        if index.is_none() {
            return Err(AppError::UserPermissionNotFound);
        }

        self.user_permissions.remove(index.unwrap());
        self.logger.info(format!("[PermissionsService] Permission '{}' revoked to '{}' user!", permission_name, user_id));

        Ok(())
    }
}
