use tokio_tungstenite::tungstenite::Message as WsMessage;

pub enum OriginalMessage {
    Ws(WsMessage),
    WebHook(serde_json::Value),
}
