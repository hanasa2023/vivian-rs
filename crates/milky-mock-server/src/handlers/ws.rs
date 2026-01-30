use crate::state::{AppState, NEXT_CLIENT_ID};
use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use futures_util::stream::StreamExt;
use std::sync::{Arc, atomic::Ordering};
use tokio::sync::mpsc;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let my_id = NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed);
    println!("WebSocket client {} connected", my_id);

    let (tx, mut _rx) = mpsc::channel(100);

    state.clients.lock().unwrap().insert(my_id, tx);

    let (mut _sender, mut receiver) = socket.split();

    loop {
        tokio::select! {
            // 从广播通道接收到的消息
            // Some(msg_to_send) = rx.recv() => {
            //     if sender.send(Message::Text(msg_to_send)).await.is_err() {
            //         // Client disconnected
            //         break;
            //     }
            // }
            // 从 websocket客户端收到的消息
            Some(Ok(msg)) = receiver.next() => {
                if let Message::Text(text) = msg {
                    println!("Received message from client {}: {}", my_id, text);
                }
            }
            else => {
                // Both channels are closed
                break;
            }
        }
    }

    // Client disconnected, remove it from the shared state
    println!("WebSocket client {} disconnected", my_id);
    state.clients.lock().unwrap().remove(&my_id);
}
