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
use crate::infrastructure::app_state::AppState;
use crate::infrastructure::http::*;
use crate::infrastructure::openapi::BaseOpenApi;
use axum::Router;
use std::env;
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::time::Duration;
use time::Duration as SessionDuration;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::decompression::RequestDecompressionLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tower_http::CompressionLevel;
use tower_sessions::cookie::SameSite;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_redis_store::{fred::prelude::*, RedisStore};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry;
use tracing_subscriber::util::SubscriberInitExt;
use utoipa::openapi::OpenApi;
use utoipa_axum::routes;
use utoipa_scalar::{Scalar, Servable};
use utoipa_swagger_ui::SwaggerUi;

pub async fn initialize_server() -> anyhow::Result<()> {
    init_observability();
    let port = get_server_port()?;
    let app_state = Arc::new(AppState::initialize_app_state().await?);
    let session_layer = initialize_session_layer().await?;

    let router = setup_router(app_state.clone(), session_layer);
    start_server(router, port).await?;
    Ok(())
}

fn get_server_port() -> anyhow::Result<u16> {
    env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .map_err(|e| anyhow::anyhow!("Invalid PORT environment variable: {}", e))
}

fn setup_router(
    app_state: Arc<AppState>,
    session_layer: SessionManagerLayer<RedisStore<Pool>>,
) -> Router {
    let (router, api) = setup_routes_and_openapi();
    let documentation_router = setup_documentation(api);

    router
        .merge(documentation_router)
        .with_state(app_state)
        .layer(session_layer)
        .layer(
            ServiceBuilder::new()
                .layer(RequestDecompressionLayer::new())
                .layer(CompressionLayer::new().quality(CompressionLevel::Fastest)),
        )
        .layer(CorsLayer::permissive())
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(10)),
        ))
}

fn setup_routes_and_openapi() -> (Router<Arc<AppState>>, OpenApi) {
    BaseOpenApi::router::<Arc<AppState>>()
        .routes(routes!(health_handler::health_check))
        .routes(routes!(auth_handler::register))
        .routes(routes!(auth_handler::login))
        .routes(routes!(auth_handler::logout))
        .routes(routes!(auth_handler::get_profile))
        .routes(routes!(auth_handler::change_password))
        .split_for_parts()
}

fn setup_documentation(api: OpenApi) -> Router<Arc<AppState>> {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .merge(Scalar::with_url("/scalar", api))
}

async fn start_server(router: Router, port: u16) -> anyhow::Result<()> {
    let address = format!("{}:{}", Ipv4Addr::UNSPECIFIED, port);
    let listener = TcpListener::bind(&address)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to bind to address {}: {}", address, e))?;

    tracing::info!("🚀 Server listening on {}", &address);

    axum::serve(listener, router)
        .await
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;
    Ok(())
}

async fn initialize_session_layer() -> anyhow::Result<SessionManagerLayer<RedisStore<Pool>>> {
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    let config = Config::from_url(&redis_url)?;
    let pool = Pool::new(config, None, None, None, 6)?;
    pool.connect();
    pool.wait_for_connect().await?;
    let session_store = RedisStore::new(pool);
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(SessionDuration::days(1)));
    Ok(session_layer)
}

fn init_observability() {
    registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
