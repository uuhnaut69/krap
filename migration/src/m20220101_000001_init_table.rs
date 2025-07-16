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

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let sql = r#"
        CREATE TABLE IF NOT EXISTS "users"
        (
            id         VARCHAR(36) PRIMARY KEY NOT NULL,
            email      VARCHAR(255) UNIQUE     NOT NULL,
            password   VARCHAR(255)            NOT NULL,
            created_at TIMESTAMPTZ             NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ             NOT NULL DEFAULT NOW()
        )
        "#;
        db.execute_unprepared(&sql).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let sql = r#"
        DROP TABLE IF EXISTS "users"
        "#;
        db.execute_unprepared(&sql).await?;
        Ok(())
    }
}
