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
use crate::application::user::api::user_service::UserService;
use crate::domain::common::DomainError;
use crate::domain::user::User;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait AuthService: Send + Sync + 'static {
    async fn register(&self, email: &str, password: &str) -> Result<User, DomainError>;
    async fn login(&self, email: &str, password: &str) -> Result<User, DomainError>;
    async fn change_password(
        &self,
        user_id: &str,
        current_password: &str,
        new_password: &str,
    ) -> Result<User, DomainError>;
}

pub struct DefaultAuthService {
    pub user_service: Arc<dyn UserService>,
}

#[async_trait::async_trait]
impl AuthService for DefaultAuthService {
    async fn register(&self, email: &str, password: &str) -> Result<User, DomainError> {
        self.user_service
            .create_user_if_not_exists(email, password)
            .await
    }

    async fn login(&self, email: &str, password: &str) -> Result<User, DomainError> {
        let user = self.user_service.find_by_email(&email).await?;
        user.is_password_match(&password)?;
        Ok(user)
    }

    async fn change_password(
        &self,
        user_id: &str,
        current_password: &str,
        new_password: &str,
    ) -> Result<User, DomainError> {
        self.user_service
            .change_password(user_id, current_password, new_password)
            .await
    }
}
