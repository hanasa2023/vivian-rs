use reqwest;
use thiserror::Error;
use tokio_tungstenite::tungstenite;

#[derive(Error, Debug)]
pub enum MilkyError {
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tungstenite::Error),
    #[error("URL parsing error: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("Unsupported schema: {0}")]
    UnsupportedScheme(String),
    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("API request failed: {message}")]
    ApiError {
        message: String,
        retcode: Option<i64>,
    },
    #[error("Http API errorL {message}")]
    HttpApiError {
        status: reqwest::StatusCode,
        message: String,
    },
    #[error("Connection not established or lost")]
    NotConnected,
    #[error("Response timeout")]
    Timeout,
    #[error("Received unexpected response type")]
    UnexpectedResponse,
    #[error("Action echo mismatch")]
    EchoMismatch,
    #[error("HTTP request error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, MilkyError>;
