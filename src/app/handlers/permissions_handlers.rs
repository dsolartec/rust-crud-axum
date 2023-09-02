use axum::{extract::{State, Path}, Json, response::{Response, IntoResponse}, debug_handler};
use hyper::StatusCode;

use crate::{app::state::ControllersState, internal::{platform::{encryption::AccessTokenPayload, validator::ValidatedJson}, apperror::AppError}, modules::permissions::models::{Permission, CreatePermissionBody}};

#[debug_handler]
pub async fn create_permission(
    State(controllers): State<ControllersState>,
    current_user: AccessTokenPayload,
    ValidatedJson(data): ValidatedJson<CreatePermissionBody>,
) -> Result<Json<Permission>, AppError> {
    current_user.check_permissions(vec!["permissions_write", "permissions_full"])?;

    Ok(Json(controllers.get_permissions_controller().write().await.create(data).await?))
}

pub async fn get_permissions(
    State(controllers): State<ControllersState>,
    current_user: AccessTokenPayload,
) -> Result<Json<Vec<Permission>>, AppError> {
    current_user.check_permissions(vec!["permissions_read", "permissions_full"])?;

    Ok(Json(controllers.get_permissions_controller().read().await.get_all().await))
}

pub async fn get_permission_by_id(
    State(controllers): State<ControllersState>,
    Path(id): Path<i64>,
    current_user: AccessTokenPayload,
) -> Result<Json<Permission>, AppError> {
    current_user.check_permissions(vec!["permissions_read", "permissions_full"])?;

    Ok(Json(controllers.get_permissions_controller().read().await.get_by_id(id).await?))
}

pub async fn get_permission_by_name(
    State(controllers): State<ControllersState>,
    Path(permission_name): Path<String>,
    current_user: AccessTokenPayload,
) -> Result<Json<Permission>, AppError> {
    current_user.check_permissions(vec!["permissions_read", "permissions_full"])?;

    Ok(Json(controllers.get_permissions_controller().read().await.get_by_name(permission_name).await?))
}

pub async fn delete_permission(
    State(controllers): State<ControllersState>,
    Path(permission_name): Path<String>,
    current_user: AccessTokenPayload,
) -> Result<Response, AppError> {
    current_user.check_permissions(vec!["permissions_write", "permissions_full"])?;

    controllers.get_permissions_controller().write().await.delete(permission_name).await?;

    Ok(StatusCode::NO_CONTENT.into_response())
}
