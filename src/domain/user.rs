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
use crate::domain::common::{DateTimeUtc, DomainError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

impl User {
    pub fn create_new_user(email: &str, password: &str) -> Result<User, DomainError> {
        let hash_password = Self::hash_password(password)?;
        let user = User {
            id: uuid::Uuid::now_v7().to_string(),
            email: email.to_lowercase(),
            password: hash_password,
            created_at: DateTimeUtc::from(chrono::Utc::now()),
            updated_at: DateTimeUtc::from(chrono::Utc::now()),
        };
        Ok(user)
    }

    pub fn hash_password(password: &str) -> Result<String, DomainError> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|_| DomainError::InternalError)
    }

    pub fn is_password_match(&self, password: &str) -> Result<(), DomainError> {
        match bcrypt::verify(password, &self.password) {
            Ok(true) => Ok(()),
            Ok(false) => Err(DomainError::PasswordNotMatchError),
            Err(_) => Err(DomainError::InternalError),
        }
    }

    pub fn change_password(&mut self, new_password: &str) -> Result<(), DomainError> {
        if self.is_password_match(new_password).is_ok() {
            return Err(DomainError::SamePasswordError);
        }

        let hashed_password = Self::hash_password(new_password)?;
        self.password = hashed_password;
        self.updated_at = DateTimeUtc::from(chrono::Utc::now());
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct UserProfile {
    pub id: String,
    pub email: String,
}

impl From<User> for UserProfile {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
        }
    }
}
