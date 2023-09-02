use axum::{Json, extract::State};

use crate::{modules::auth::models::{LogInBody, AuthResponse, SignUpBody}, internal::{apperror::AppError, platform::validator::ValidatedJson}, app::state::ControllersState};

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
