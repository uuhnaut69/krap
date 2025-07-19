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
use crate::domain::user::UserProfile;
use crate::infrastructure::http::error_handler::{ApiError, ErrorKind};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use tower_sessions::Session;

pub const SESSION_USER_KEY: &str = "user";

pub struct AuthenticatedUser(pub UserProfile);

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl Future<Output=Result<Self, Self::Rejection>> + Send {
        async move {
            let session = parts.extract::<Session>().await.map_err(|_| {
                ApiError::new(
                    "internal_server_error".to_string(),
                    ErrorKind::InternalServerError,
                )
            })?;

            let current_user: UserProfile = session
                .get(SESSION_USER_KEY)
                .await
                .map_err(|_| {
                    ApiError::new(
                        "internal_server_error".to_string(),
                        ErrorKind::InternalServerError,
                    )
                })?
                .ok_or_else(|| {
                    ApiError::new(
                        "unauthenticated_error".to_string(),
                        ErrorKind::Unauthorized,
                    )
                })?;

            Ok(AuthenticatedUser(current_user))
        }
    }
}
