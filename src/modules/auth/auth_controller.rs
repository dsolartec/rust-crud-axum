use std::sync::Arc;

use axum::{body, response::Response};
use hyper::{Body, StatusCode};
use tokio::sync::RwLock;

use crate::{modules::{users::UsersService, permissions::PermissionsService}, internal::{apperror::AppError, platform::{encryption::{JwtEncryption, AccessTokenPayload}, logger::Logger}}};

use super::models::{LogInBody, AuthResponse, SignUpBody};

#[derive(Clone)]
pub struct AuthController {
    logger: Logger,
    jwt_encryption: JwtEncryption,
    users_service: Arc<RwLock<UsersService>>,
    permissions_service: Arc<RwLock<PermissionsService>>,
}

impl AuthController {
    pub fn new(
        logger: &Logger,
        jwt_encryption: &JwtEncryption,
        users_service: &Arc<RwLock<UsersService>>,
        permissions_service: &Arc<RwLock<PermissionsService>>,
    ) -> AuthController {
        AuthController {
            logger: logger.clone(),
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

        let access_token = self.jwt_encryption.generate_access_token(
            AccessTokenPayload::new(user.id, &user.username, permissions, user.otp_enabled, false),
        )?;

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

        let access_token = self.jwt_encryption.generate_access_token(
            AccessTokenPayload::new(user.id, &user.username, permissions, false, false),
        )?;

        Ok(AuthResponse { access_token })
    }

    pub async fn get_otp_qr_code(&self, user_id: i64) -> Result<Response, AppError> {
        let users_service_read = self.users_service.read().await;

        let user = users_service_read.get_by_id(user_id);
        if user.is_none() {
            return Err(AppError::UserNotFound);
        }

        let user = user.unwrap();

        let qr_code = user.otp.get_qr_png();
        if let Err(err) = qr_code {
            self.logger.error(err, "An error ocurred while generating OTP QR Code");
            return Err(AppError::InternalServerError);
        }

        Ok(
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "image/png")
                .body(body::boxed(Body::from(qr_code.unwrap())))
                .unwrap()
        )
    }

    pub async fn verify_otp_code(
        &mut self,
        user_id: i64,
        otp_code: String,
        change_status: bool,
    ) -> Result<AuthResponse, AppError> {
        let mut users_service_write = self.users_service.write().await;

        let user = users_service_write.get_by_id(user_id);
        if user.is_none() {
            return Err(AppError::UserNotFound);
        }

        let mut user = user.unwrap();

        let totp_status = user.otp.check_current(&otp_code);
        if let Err(err) = totp_status {
            self.logger.error(err, "OTP invalid");
            return Err(AppError::InvalidOTP);
        }

        if !totp_status.unwrap() {
            self.logger.error("OTP", "Invalid OTP code");
            return Err(AppError::InvalidOTP);
        }

        if change_status {
            user = users_service_write.change_otp_status_for_user(user_id)?;
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

        let access_token = self.jwt_encryption.generate_access_token(
            AccessTokenPayload::new(user.id, &user.username, permissions, user.otp_enabled, true),
        )?;

        Ok(AuthResponse { access_token })
    }
}
