use axum::{Json, extract::{State, Path}, response::Response};

use crate::{modules::{auth::models::{LogInBody, AuthResponse, SignUpBody}, users::models::User}, internal::{apperror::AppError, platform::{validator::ValidatedJson, encryption::AccessTokenPayload}}, app::state::ControllersState};

pub async fn log_in(
    State(controllers): State<ControllersState>,
    Json(data): Json<LogInBody>,
) -> Result<Json<AuthResponse>, AppError> {
    Ok(Json(controllers.get_auth_controller().read().await.log_in(data).await?))
}

pub async fn sign_up(
    State(controllers): State<ControllersState>,
    ValidatedJson(data): ValidatedJson<SignUpBody>,
) -> Result<Json<AuthResponse>, AppError> {
    Ok(Json(controllers.get_auth_controller().write().await.sign_up(data).await?))
}

pub async fn get_otp_qr_code(
    State(controllers): State<ControllersState>,
    current_user: AccessTokenPayload,
) -> Result<Response, AppError> {
    if current_user.otp_enabled || current_user.otp_passed {
        return Err(AppError::UnAuthorized);
    }

    let user_id: i64 = current_user.sub.parse().unwrap();

    controllers.get_auth_controller().read().await.get_otp_qr_code(user_id).await
}

pub async fn enable_otp(
    State(controllers): State<ControllersState>,
    Path(otp_code): Path<String>,
    current_user: AccessTokenPayload,
) -> Result<Json<AuthResponse>, AppError> {
    if current_user.otp_enabled || current_user.otp_passed {
        return Err(AppError::UnAuthorized);
    }

    let user_id: i64 = current_user.sub.parse().unwrap();

    Ok(Json(controllers.get_auth_controller().write().await.verify_otp_code(user_id, otp_code, true).await?))
}

pub async fn verify_otp_code(
    State(controllers): State<ControllersState>,
    Path(otp_code): Path<String>,
    current_user: AccessTokenPayload,
) -> Result<Json<AuthResponse>, AppError> {
    if !current_user.otp_enabled || current_user.otp_passed {
        return Err(AppError::UnAuthorized);
    }

    let user_id: i64 = current_user.sub.parse().unwrap();

    Ok(Json(controllers.get_auth_controller().write().await.verify_otp_code(user_id, otp_code, false).await?))
}

pub async fn me(
    State(controllers): State<ControllersState>,
    current_user: AccessTokenPayload,
) -> Result<Json<User>, AppError> {
    Ok(Json(controllers.get_users_controller().read().await.get_by_username(current_user.username).await?))
}
