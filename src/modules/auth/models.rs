use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize)]
pub struct LogInBody {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct SignUpBody {
    #[validate(length(message = "El nombre de usuario debe contener entre 4 y 15 caracteres", min = 4, max = 15))]
    pub username: String,

    #[validate(length(message = "La contraseña debe contener entre 8 y 40 caracteres", min = 8, max = 40))]
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
}
