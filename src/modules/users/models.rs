use serde::{Serialize, Deserialize};
use totp_rs::TOTP;

#[derive(Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[serde(skip)]
    pub password: String,
    pub otp_enabled: bool,
    #[serde(skip)]
    pub otp: TOTP,
}
