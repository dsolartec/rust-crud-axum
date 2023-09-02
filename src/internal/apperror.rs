use std::collections::HashMap;

use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Internal server error")]
    InternalServerError,

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("Unauthorized")]
    UnAuthorized,

    // Users
    #[error("Wrong authentication")]
    UserWrongAuthentication,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("User not found")]
    UserNotFound,

    #[error("User not deletable")]
    UserNotDeletable,

    // Permissions
    #[error("Permission already exists")]
    PermissionAlreadyExists,

    #[error("Permission not found")]
    PermissionNotFound,

    #[error("Permission not deletable")]
    PermissionNotDeletable,

    // User permissions
    #[error("User already has permission")]
    UserAlreadyHasPermission,

    #[error("User permission not found")]
    UserPermissionNotFound,

    #[error("Cannot revoke user permission")]
    CannotRevokeUserPermission,
}

impl From<axum::extract::rejection::JsonRejection> for AppError {
    fn from(_value: axum::extract::rejection::JsonRejection) -> Self {
        AppError::InternalServerError
    }
}

impl From<axum::extract::rejection::TypedHeaderRejection> for AppError {
    fn from(_value: axum::extract::rejection::TypedHeaderRejection) -> Self {
        AppError::UnAuthorized
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(_value: jsonwebtoken::errors::Error) -> Self {
        AppError::UnAuthorized
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        if let AppError::ValidationError(errors) = self {
            let status_code = StatusCode::BAD_REQUEST;

            let mut details: HashMap<String, String> = HashMap::new();
            for (field, errors) in errors.field_errors() {
                for err in errors {
                    if let Some(message) = err.message.clone() {
                        details.insert(field.to_string(), message.to_string());
                    }
                }
            }

            let body = Json(json!({
                "code": "validation_error",
                "validation_details": details,
                "message": "Ha ocurrido un error validando la información",
                "status_code": status_code.as_u16(),
            }));

            return (status_code, body).into_response();
        }

        let (status_code, code, message) = match self {
            AppError::UnAuthorized => (StatusCode::UNAUTHORIZED, "unauthorized_user", "No tienes permitido consumir esta url"),

            // Users
            AppError::UserWrongAuthentication => (StatusCode::BAD_REQUEST, "wrong_authentication", "El usuario o la contraseña no son correctos"),
            AppError::UserAlreadyExists => (StatusCode::CONFLICT, "user_already_exists", "El nombre de usuario ya está en uso"),
            AppError::UserNotFound => (StatusCode::NOT_FOUND, "user_not_found", "El usuario no existe"),
            AppError::UserNotDeletable => (StatusCode::CONFLICT, "cannot_delete_user", "No puedes eliminar el usuario con el que estás autenticado"),

            // Permissions
            AppError::PermissionAlreadyExists => (StatusCode::CONFLICT, "permission_already_exists", "El nombre del permiso ya está en uso"),
            AppError::PermissionNotFound => (StatusCode::NOT_FOUND, "permission_not_found", "El permiso no existe"),
            AppError::PermissionNotDeletable => (StatusCode::CONFLICT, "permission_not_deletable", "No puedes eliminar este permiso"),

            // User permissions
            AppError::UserAlreadyHasPermission => (StatusCode::CONFLICT, "user_has_permission", "El usuario ya posee este permiso"),
            AppError::UserPermissionNotFound => (StatusCode::BAD_REQUEST, "user_not_has_permission", "El usuario no posee este permiso"),
            AppError::CannotRevokeUserPermission => (StatusCode::CONFLICT, "cannot_revoke_permission", "No puedes eliminarle un permiso al usuario con el que estás autenticado"),
        
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "internal_server_error", "Error interno"),
        };

        let body = Json(json!({
            "code": code,
            "message": message,
            "status_code": status_code.as_u16(),
        }));

        (status_code, body).into_response()
    }
}
