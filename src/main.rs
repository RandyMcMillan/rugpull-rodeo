use std::net::SocketAddr;
use temp_st::*;
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = load_app_state().await;
    let addr = state
        .bind_addr
        .parse::<SocketAddr>()
        .expect("Invalid address format");

    info!("171:main:test");
    start_cleanup_job(state.clone());
    start_trust_network_refresh_job(state.clone());

    let app = create_app(state).await;

    info!("🎧 blossom server listening on {}", addr);

    // Create a shutdown signal handler
    let shutdown = signal::ctrl_c();

    // Start the server with graceful shutdown
    let server = axum_server::bind(addr).serve(app.into_make_service());

    // Wait for either the server to complete or a shutdown signal
    tokio::select! {
        _ = server => {
            info!("Server completed");
        }
        _ = shutdown => {
            info!("Shutting down gracefully...");
        }
    }
}
