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
use crate::application::user::spi::user_repository::UserRepository;
use crate::domain::common::DomainError;
use crate::domain::user::User;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait UserService: Send + Sync + 'static {
    async fn create_user_if_not_exists(
        &self,
        email: &str,
        password: &str,
    ) -> Result<User, DomainError>;

    async fn find_by_email(&self, email: &str) -> Result<User, DomainError>;

    async fn change_password(
        &self,
        user_id: &str,
        current_password: &str,
        new_password: &str,
    ) -> Result<User, DomainError>;
}

pub struct DefaultUserService {
    pub user_repository: Arc<dyn UserRepository>,
}

#[async_trait::async_trait]
impl UserService for DefaultUserService {
    async fn create_user_if_not_exists(
        &self,
        email: &str,
        password: &str,
    ) -> Result<User, DomainError> {
        let user = User::create_new_user(email, password)?;

        match self.user_repository.find_by_email(email).await {
            Ok(Some(_)) => {
                return Err(DomainError::ConflictError(
                    "user_already_exists_error".to_string(),
                ));
            }
            Ok(None) => {}
            Err(e) => {
                tracing::error!("Error checking for existing user: {:?}", e);
                return Err(DomainError::InternalError);
            }
        }

        let saved_user = self
            .user_repository
            .save(user)
            .await
            .map_err(|_| DomainError::InternalError)?;
        Ok(saved_user)
    }

    async fn find_by_email(&self, email: &str) -> Result<User, DomainError> {
        match self.user_repository.find_by_email(email).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(DomainError::NotFoundError),
            Err(e) => {
                tracing::error!("Error finding user by email: {:?}", e);
                Err(DomainError::InternalError)
            }
        }
    }

    async fn change_password(
        &self,
        user_id: &str,
        current_password: &str,
        new_password: &str,
    ) -> Result<User, DomainError> {
        let mut user = match self.user_repository.find_by_id(user_id).await {
            Ok(Some(user)) => user,
            Ok(None) => return Err(DomainError::NotFoundError),
            Err(e) => {
                tracing::error!("Error finding user by id: {:?}", e);
                return Err(DomainError::InternalError);
            }
        };

        user.is_password_match(current_password)?;
        user.change_password(new_password)?;

        let updated_user = self
            .user_repository
            .update(user)
            .await
            .map_err(|_| DomainError::InternalError)?;

        Ok(updated_user)
    }
}
