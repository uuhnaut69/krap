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
use crate::application::auth::api::auth_service::{AuthService, DefaultAuthService};
use crate::application::health::api::health_service::{HealthService, HealthServiceImpl};
use crate::application::user::api::user_service::DefaultUserService;
use crate::infrastructure::application_health::ApplicationHealth;
use crate::infrastructure::persistence::seaorm::db::establish_connection;
use crate::infrastructure::persistence::seaorm::repository::user_repository::SeaOrmUserRepository;
use anyhow;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub health_service: Arc<dyn HealthService>,
    pub auth_service: Arc<dyn AuthService>,
}

impl AppState {
    pub async fn initialize_app_state() -> anyhow::Result<Self> {
        // Health module
        let application_health = Arc::new(ApplicationHealth::default());
        let health_service = Arc::new(HealthServiceImpl {
            health_repository: application_health,
        });

        let db_connection = establish_connection().await?;

        // Auth module
        let user_repository = Arc::new(SeaOrmUserRepository {
            db: db_connection.clone(),
        });
        let user_service = Arc::new(DefaultUserService {
            user_repository: user_repository.clone(),
        });
        let auth_service = Arc::new(DefaultAuthService { user_service });

        Ok(AppState {
            health_service,
            auth_service,
        })
    }
}
