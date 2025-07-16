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
use sea_orm::ConnectOptions;

pub async fn establish_connection() -> anyhow::Result<sea_orm::DatabaseConnection> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| anyhow::anyhow!("DATABASE_URL environment variable must be set"))?;
    let mut opt = ConnectOptions::new(&database_url);
    opt.max_connections(20);
    opt.min_connections(5);
    opt.sqlx_logging(true);
    sea_orm::Database::connect(opt)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to the database: {}", e))
}
