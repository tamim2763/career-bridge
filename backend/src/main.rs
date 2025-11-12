use std::net::SocketAddr;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

mod errors;
mod handlers;
mod models;
mod security;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global tracing subscriber");

    let app = handlers::create_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Server listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
