use axum::{Json, extract::State};

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

pub async fn me(
    State(controllers): State<ControllersState>,
    current_user: AccessTokenPayload,
) -> Result<Json<User>, AppError> {
    Ok(Json(controllers.get_users_controller().read().await.get_by_username(current_user.username).await?))
}
