use axum::{
    Router,
    routing::{get, post},
};
use std::net::SocketAddr;

mod errors;
mod handlers;
mod state;

use eyre::{Result, WrapErr};
use handlers::{api::api_handler, ws::websocket_handler};
use state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    errors::install_hooks()?;

    let state = AppState::new();

    let app = Router::new()
        .route("/api/{api}", post(api_handler))
        .route("/event", get(websocket_handler))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3002));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .wrap_err("无法创建监听器")?;
    axum::serve(listener, app)
        .await
        .wrap_err("服务器运行失败")?;

    Ok(())
}
