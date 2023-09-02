use axum::{Json, extract::{State, Path}, response::{Response, IntoResponse}};
use hyper::StatusCode;

use crate::{modules::users::models::User, app::state::ControllersState, internal::{apperror::AppError, platform::encryption::AccessTokenPayload}};

pub async fn get_users(
    State(controllers): State<ControllersState>,
    current_user: AccessTokenPayload,
) -> Result<Json<Vec<User>>, AppError> {
    current_user.check_permissions(vec!["users_read", "users_full"])?;

    Ok(Json(controllers.get_users_controller().read().await.get_all().await))
}

pub async fn get_user_by_id(
    State(controllers): State<ControllersState>,
    Path(id): Path<i64>,
    current_user: AccessTokenPayload,
) -> Result<Json<User>, AppError> {
    current_user.check_permissions(vec!["users_read", "users_full"])?;

    Ok(Json(controllers.get_users_controller().read().await.get_by_id(id).await?))
}

pub async fn get_user_by_username(
    State(controllers): State<ControllersState>,
    Path(username): Path<String>,
    current_user: AccessTokenPayload,
) -> Result<Json<User>, AppError> {
    current_user.check_permissions(vec!["users_read", "users_full"])?;

    Ok(Json(controllers.get_users_controller().read().await.get_by_username(username).await?))
}

pub async fn delete_user(
    State(controllers): State<ControllersState>,
    Path(username): Path<String>,
    current_user: AccessTokenPayload,
) -> Result<Response, AppError> {
    current_user.check_permissions(vec!["users_write", "users_full"])?;

    if current_user.username.to_lowercase() == username.to_lowercase() {
        return Err(AppError::UserNotDeletable);
    }

    controllers.get_users_controller().write().await.delete(username).await?;

    Ok(StatusCode::NO_CONTENT.into_response())
}

pub async fn get_permissions_for_user(
    State(controllers): State<ControllersState>,
    Path(username): Path<String>,
    current_user: AccessTokenPayload,
) -> Result<Json<Vec<String>>, AppError> {
    current_user.check_permissions(vec!["users_read", "users_full"])?;

    Ok(Json(controllers.get_permissions_controller().read().await.get_permissions_for_user(username).await?))
}

pub async fn grant_permission(
    State(controllers): State<ControllersState>,
    Path((username, permission_name)): Path<(String, String)>,
    current_user: AccessTokenPayload,
) -> Result<Response, AppError> {
    current_user.check_permissions(vec!["grant_permission"])?;

    controllers.get_permissions_controller().write().await.grant_permission(username, permission_name).await?;

    Ok(StatusCode::NO_CONTENT.into_response())
}

pub async fn revoke_permission(
    State(controllers): State<ControllersState>,
    Path((username, permission_name)): Path<(String, String)>,
    current_user: AccessTokenPayload,
) -> Result<Response, AppError> {
    current_user.check_permissions(vec!["revoke_permission"])?;

    if current_user.username.to_lowercase() == username.to_lowercase() {
        return Err(AppError::CannotRevokeUserPermission);
    }

    controllers.get_permissions_controller().write().await.revoke_permission(username, permission_name).await?;

    Ok(StatusCode::NO_CONTENT.into_response())
}
