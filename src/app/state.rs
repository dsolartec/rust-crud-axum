use std::sync::Arc;

use tokio::sync::RwLock;

use crate::modules::{auth::AuthController, users::UsersController, permissions::PermissionsController};

#[derive(Clone)]
pub struct ControllersState {
    auth_controller: Arc<RwLock<AuthController>>,
    users_controller: Arc<RwLock<UsersController>>,
    permissions_controller: Arc<RwLock<PermissionsController>>,
}

impl ControllersState {
    pub fn new(
        auth_controller: AuthController,
        users_controller: UsersController,
        permissions_controller: PermissionsController,
    ) -> ControllersState {
        ControllersState {
            auth_controller: Arc::new(RwLock::new(auth_controller)),
            users_controller: Arc::new(RwLock::new(users_controller)),
            permissions_controller: Arc::new(RwLock::new(permissions_controller)),
        }
    }

    pub fn get_auth_controller(self) -> Arc<RwLock<AuthController>> {
        self.auth_controller
    }

    pub fn get_users_controller(self) -> Arc<RwLock<UsersController>> {
        self.users_controller
    }

    pub fn get_permissions_controller(self) -> Arc<RwLock<PermissionsController>> {
        self.permissions_controller
    }
}
