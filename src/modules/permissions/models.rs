use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Deserialize, Serialize)]
pub struct Permission {
    pub id: i64,
    #[serde(rename = "permission_name")]
    pub name: String,
    pub description: String,
    #[serde(skip)]
    pub deletable: bool,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct CreatePermissionBody {
    #[validate(length(message = "El nombre del permiso debe contener entre 4 y 50 caracteres", min = 4, max = 50))]
    pub name: String,

    #[validate(length(message = "La descripción sólo puede contener hasta 100 caracteres", min = 1, max = 100))]
    pub description: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UserPermission {
    pub user_id: i64,
    pub permission_id: i64,
}
