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

use crate::domain::common::DomainError;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use std::collections::HashMap;
use utoipa::ToSchema;
use validator::ValidationErrors;

pub type ApiResult<T> = Result<Json<T>, ApiError>;
pub type ErrorDetail = HashMap<String, String>;

#[derive(Debug)]
pub enum ErrorKind {
    BadRequest,
    Unauthorized,
    NotFound,
    Conflict,
    InternalServerError,
}

impl ErrorKind {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Conflict => StatusCode::CONFLICT,
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Serialize, Debug, ToSchema)]
pub struct ApiError {
    pub message: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[serde(skip)]
    pub kind: ErrorKind,
}

impl ApiError {
    pub fn new(message: String, kind: ErrorKind) -> Self {
        Self {
            message,
            details: vec![],
            kind,
        }
    }

    fn with_details(message: String, details: Vec<ErrorDetail>, kind: ErrorKind) -> Self {
        Self {
            message,
            details,
            kind,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.kind.status_code(), Json(self)).into_response()
    }
}

impl From<DomainError> for ApiError {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::InternalError => {
                tracing::error!("Internal server error occurred");
                ApiError::new(
                    "internal_error".to_string(),
                    ErrorKind::InternalServerError,
                )
            }
            DomainError::ConflictError(message) => {
                tracing::warn!("Conflict error: {}", message);
                ApiError::new(message, ErrorKind::Conflict)
            }
            DomainError::NotFoundError => {
                ApiError::new("not_found_error".to_string(), ErrorKind::NotFound)
            }
            DomainError::PasswordNotMatchError
            | DomainError::AuthenticationFailed
            | DomainError::InvalidCredentials => {
                tracing::warn!("Authentication error: {}", error);
                ApiError::new(error.to_string(), ErrorKind::Unauthorized)
            }
            DomainError::SamePasswordError => {
                tracing::warn!("Same password validation error: {}", error);
                ApiError::new(error.to_string(), ErrorKind::BadRequest)
            }
        }
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(errors: ValidationErrors) -> Self {
        let details: Vec<ErrorDetail> = errors
            .field_errors()
            .iter()
            .map(|(field, field_errors)| {
                let error_messages: Vec<String> = field_errors
                    .iter()
                    .map(|error| {
                        error
                            .message
                            .as_ref()
                            .map(|msg| msg.to_string())
                            .unwrap_or_else(|| format!("invalid_{}", field))
                    })
                    .collect();

                let mut detail = HashMap::new();
                detail.insert(field.to_string(), error_messages.join(", "));
                detail
            })
            .collect();

        tracing::debug!("Validation errors: {:?}", details);
        ApiError::with_details(
            "validation_error".to_string(),
            details,
            ErrorKind::BadRequest,
        )
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(error: anyhow::Error) -> Self {
        tracing::error!("Unexpected error: {:?}", error);
        ApiError::new(
            "internal_error".to_string(),
            ErrorKind::InternalServerError,
        )
    }
}
