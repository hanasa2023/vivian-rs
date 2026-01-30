use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use serde_json::Value;
use std::sync::Arc;

pub async fn api_handler(
    Path(api): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> String {
    println!("Received API call for: {}", api);
    let broadcast_message = payload.to_string();

    let clients = state.clients.lock().unwrap();
    for tx in clients.values().cloned() {
        let broadcast_message = broadcast_message.clone();
        tokio::spawn(async move {
            if tx.send(broadcast_message.clone()).await.is_err() {
                // This client has disconnected, its removal is handled in handle_socket
            }
        });
    }

    format!("Message broadcasted to {} clients.", clients.len())
}
