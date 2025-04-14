mod app_state;
mod config;
mod constants;
mod controllers;
mod core;
mod dtos;
mod extensions;
mod models;
mod router;
mod service;

use std::sync::Arc;

use anyhow::Context;
use app_state::AppState;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or("debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&config::CONFIG.db_url)
        .await
        .context("Failed to connect to database")
        .unwrap();

    let app = router::api_router().with_state(Arc::new(AppState::new(db)));

    let listener = TcpListener::bind("localhost:3001").await.unwrap();

    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
