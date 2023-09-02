use axum::{extract::FromRequestParts, async_trait, http::request::Parts, TypedHeader, headers::{Authorization, authorization::Bearer}, RequestPartsExt};
use chrono::{Utc, Duration};
use jsonwebtoken::{Validation, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::internal::apperror::AppError;

#[derive(Deserialize, Serialize)]
pub struct AccessTokenPayload {
    pub sub: String,
    exp: usize,
    pub username: String,
    pub permissions: Vec<String>,
}

impl AccessTokenPayload {
    pub fn new(user_id: i64, username: &str, permissions: Vec<String>) -> AccessTokenPayload {
        let exp = (Utc::now() + Duration::hours(1)).timestamp() as usize;

        AccessTokenPayload {
            sub: user_id.to_string(),
            exp,
            username: username.to_string(),
            permissions,
        }
    }

    pub fn check_permissions(&self, permissions: Vec<&str>) -> Result<(), AppError> {
        if !permissions.is_empty() && self.permissions.is_empty() {
            return Err(AppError::UnAuthorized);
        }

        let mut can = false;
        for permission in permissions {
            if self.permissions.contains(&permission.to_string()) {
                can = true;
                break;
            }
        }

        if !can {
            return Err(AppError::UnAuthorized);
        }

        Ok(())
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AccessTokenPayload
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts.extract::<TypedHeader<Authorization<Bearer>>>().await?;

        let token_data: jsonwebtoken::TokenData<AccessTokenPayload> = jsonwebtoken::decode::<AccessTokenPayload>(
            bearer.token(),
            &DecodingKey::from_secret("Abecedario".as_bytes()),
            &Validation::default(),
        )?;
        
        Ok(token_data.claims)
    }
}

#[derive(Clone)]
pub struct JwtEncryption;

impl JwtEncryption {
    pub fn new() -> JwtEncryption {
        JwtEncryption {}
    }

    pub fn generate_access_token(&self, payload: AccessTokenPayload) -> Result<String, AppError> {
        Ok(jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret("Abecedario".as_bytes()),
        )?)
    }
}
