mod config;
mod constants;
mod ctx;
mod models;
mod router;

use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or("debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = router::api_router();

    // .layer(ErrorLayer::new());

    let listener = TcpListener::bind("localhost:3001").await.unwrap();

    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
