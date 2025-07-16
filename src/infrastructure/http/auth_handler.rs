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
use crate::infrastructure::app_state::AppState;
use crate::infrastructure::http::common::auth::{AuthenticatedUser, SESSION_USER_KEY};
use crate::infrastructure::http::common::validator::ValidatedJson;
use crate::infrastructure::http::error_handler::{ApiError, ApiResult, ErrorCategory};
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_sessions::Session;
use utoipa::ToSchema;

const AUTH_TAG: &str = "Auth";

#[derive(Serialize, Debug, ToSchema)]
pub struct AuthResponse {
    pub id: String,
    pub email: String,
}

#[derive(Deserialize, Debug, ToSchema, validator::Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "invalid_email_format"))]
    pub email: String,
    #[validate(length(min = 8, message = "password_must_be_at_least_8_characters"))]
    pub password: String,
}

#[utoipa::path(
    tag = AUTH_TAG,
    post,
    path = "/auth/register",
    description = "Register a new user account with email and password. Creates a new user session upon successful registration.",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Validation error", body = ApiError),
        (status = 409, description = "User already exists", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
pub async fn register(
    State(app_state): State<Arc<AppState>>,
    session: Session,
    ValidatedJson(request): ValidatedJson<RegisterRequest>,
) -> ApiResult<AuthResponse> {
    let user = app_state
        .auth_service
        .register(&request.email, &request.password)
        .await?;

    let current_user = UserProfile::from(user.clone());

    session
        .insert(SESSION_USER_KEY, &current_user)
        .await
        .map_err(|_| {
            ApiError::new(
                "failed_to_create_session_error".to_string(),
                ErrorCategory::InternalServerError,
            )
        })?;

    Ok(Json(AuthResponse {
        id: user.id.to_string(),
        email: user.email,
    }))
}

#[derive(Deserialize, Debug, ToSchema, validator::Validate)]
pub struct LoginRequest {
    #[validate(email(message = "invalid_email_format"))]
    pub email: String,
    #[validate(length(min = 1, message = "password_required"))]
    pub password: String,
}

#[utoipa::path(
    tag = AUTH_TAG,
    post,
    path = "/auth/login",
    description = "Authenticate user with email and password credentials. Creates a new user session upon successful authentication.",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successfully", body = AuthResponse),
        (status = 401, description = "Invalid credentials", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
pub async fn login(
    State(app_state): State<Arc<AppState>>,
    session: Session,
    ValidatedJson(request): ValidatedJson<LoginRequest>,
) -> ApiResult<AuthResponse> {
    let user = app_state
        .auth_service
        .login(&request.email, &request.password)
        .await?;

    let current_user = UserProfile::from(user.clone());

    session
        .insert(SESSION_USER_KEY, &current_user)
        .await
        .map_err(|_| {
            ApiError::new(
                "failed_to_create_session_error".to_string(),
                ErrorCategory::InternalServerError,
            )
        })?;

    Ok(Json(AuthResponse {
        id: user.id.to_string(),
        email: user.email,
    }))
}

#[utoipa::path(
    tag = AUTH_TAG,
    post,
    path = "/auth/logout",
    description = "Logout the current user by terminating their session. This will invalidate the user's session and require re-authentication.",
    responses(
        (status = 200, description = "Logout successfully"),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
pub async fn logout(session: Session) -> ApiResult<()> {
    session.flush().await.map_err(|_| {
        ApiError::new(
            "failed_to_logout_error".to_string(),
            ErrorCategory::InternalServerError,
        )
    })?;

    Ok(Json(()))
}

#[utoipa::path(
    tag = AUTH_TAG,
    get,
    path = "/auth/profile",
    description = "Retrieve the current authenticated user's profile information. Requires a valid user session.",
    responses(
        (status = 200, description = "User profile information", body = AuthResponse),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
pub async fn profile(
    AuthenticatedUser(current_user): AuthenticatedUser,
) -> ApiResult<AuthResponse> {
    Ok(Json(AuthResponse {
        id: current_user.id,
        email: current_user.email,
    }))
}
