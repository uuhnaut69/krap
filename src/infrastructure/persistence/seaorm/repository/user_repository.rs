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
use crate::domain::user::User;
use crate::infrastructure::persistence::seaorm::entity::users;
use sea_orm::ColumnTrait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, Set};

pub struct SeaOrmUserRepository {
    pub db: DatabaseConnection,
}

#[async_trait::async_trait]
impl UserRepository for SeaOrmUserRepository {
    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<User>> {
        let found_user = users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .one(&self.db)
            .await?
            .map(|user| User {
                id: user.id,
                email: user.email,
                password: user.password,
                created_at: user.created_at,
                updated_at: user.updated_at,
            });
        Ok(found_user)
    }

    async fn save(&self, user: User) -> anyhow::Result<User> {
        let model = users::ActiveModel {
            id: Set(user.id),
            email: Set(user.email),
            password: Set(user.password),
            created_at: Set(user.created_at),
            updated_at: Set(user.updated_at),
        };

        let saved_user = users::Entity::insert(model)
            .exec_with_returning(&self.db)
            .await?;

        Ok(User {
            id: saved_user.id,
            email: saved_user.email,
            password: saved_user.password,
            created_at: saved_user.created_at,
            updated_at: saved_user.updated_at,
        })
    }
}
