/*
 * Copyright 2025 uuhnaut69
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::infrastructure::http::error_handler::{ApiError, ErrorKind};
use axum::Json;
use axum::body::Body;
use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRequest, Request};
use serde::de::DeserializeOwned;
use std::future::Future;
use validator::Validate;

#[derive(Debug)]
pub struct ValidatedJson<T>(pub T);

#[async_trait::async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ApiError;

    fn from_request(
        req: Request<Body>,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let Json(value) = Json::<T>::from_request(req, state)
                .await
                .map_err(|rejection| {
                    tracing::debug!("JSON parsing error: {:?}", rejection);
                    ApiError::new("invalid_json_format".to_string(), ErrorKind::BadRequest)
                })?;

            value
                .validate()
                .map_err(|validation_errors| ApiError::from(validation_errors))?;
            Ok(ValidatedJson(value))
        }
    }
}

pub trait ValidateExt {
    fn validate_and_map_error(&self) -> Result<(), ApiError>;
}

impl<T> ValidateExt for T
where
    T: Validate,
{
    fn validate_and_map_error(&self) -> Result<(), ApiError> {
        self.validate().map_err(ApiError::from)
    }
}
