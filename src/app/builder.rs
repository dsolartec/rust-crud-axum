use crate::{internal::{platform::{encryption::JwtEncryption, logger::Logger}, apperror::AppError}, modules::{users::UsersService, permissions::PermissionsService}};

use super::App;

pub struct AppBuilder {
    logger: Option<Logger>,

    // Encryption
    jwt_encryption: Option<JwtEncryption>,

    // Services
    users_service: Option<UsersService>,
    permissions_service: Option<PermissionsService>,
}

impl AppBuilder {
    pub fn new() -> AppBuilder {
        AppBuilder {
            logger: None,

            // Encryption
            jwt_encryption: None,

            // Services
            users_service: None,
            permissions_service: None,
        }
    }

    pub async fn build(self) -> Result<App, AppError> {
        App::new(
            self.logger,

            // Encryption
            self.jwt_encryption,

            // Services
            self.users_service,
            self.permissions_service,
        ).await
    }
}
