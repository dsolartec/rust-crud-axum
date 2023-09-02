use axum::{extract::{FromRequest, rejection::JsonRejection}, Json, async_trait};
use hyper::Request;
use serde::de::DeserializeOwned;

use crate::internal::apperror::AppError;

pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedJson<T>
where
    T: DeserializeOwned + validator::Validate,
    S: Send + Sync,
    B: Send + 'static,
    Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(Self(value))
    }
}
