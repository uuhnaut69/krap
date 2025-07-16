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

use crate::domain::user::User;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<User>>;

    async fn find_by_id(&self, id: &str) -> anyhow::Result<Option<User>>;

    async fn save(&self, user: User) -> anyhow::Result<User>;

    async fn update(&self, user: User) -> anyhow::Result<User>;
}
