use crate::error::{MilkyError, Result};
use crate::types::common::ApiResponse;
use crate::types::event::Event;

use futures_util::{StreamExt, lock::Mutex};
use log::{debug, error, info, warn};
use reqwest::StatusCode;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message as WsMessage,
};
use url::Url;

pub struct MilkyClient {
    http_client: reqwest::Client,
    api_base_url: Url,
    event_ws_url: Url,
    access_token: Option<String>,
    ws_stream: Arc<Mutex<Option<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    event_sender: mpsc::Sender<Event>,
}

impl MilkyClient {
    pub fn new(
        host_address: &str, // e.g., "http://127.0.0.1:8080"
        access_token: Option<String>,
        event_sender: mpsc::Sender<Event>,
    ) -> Result<Self> {
        let base_url = Url::parse(host_address)?;

        let mut api_base_url = base_url.to_string();

        if !api_base_url.ends_with('/') {
            api_base_url.push('/');
        }

        let api_base_url = Url::parse(&api_base_url)?.join("api/")?;

        let mut event_ws_url = base_url.clone();
        let scheme = match base_url.scheme() {
            "http" => "ws",
            "https" => "wss",
            _ => return Err(MilkyError::UnsupportedScheme(base_url.scheme().to_string())),
        };
        event_ws_url
            .set_scheme(scheme)
            .map_err(|_| MilkyError::UrlParse(url::ParseError::InvalidPort))?; // Simplified error
        event_ws_url.set_path("event");
        if let Some(token) = &access_token {
            event_ws_url
                .query_pairs_mut()
                .append_pair("access_token", token);
        }

        Ok(Self {
            http_client: reqwest::Client::new(),
            api_base_url,
            event_ws_url,
            access_token,
            ws_stream: Arc::new(Mutex::new(None)),
            event_sender,
        })
    }

    pub async fn connect_events(&self) -> Result<()> {
        info!("Connecting to WebSocket for events: {}", self.event_ws_url);
        let (ws_stream_internal, response) = connect_async(self.event_ws_url.clone())
            .await
            .map_err(MilkyError::WebSocket)?;
        info!("WebSocket handshake for events successfully completed!");
        debug!("Response HTTP Code: {}", response.status());

        *self.ws_stream.lock().await = Some(ws_stream_internal);

        let ws_stream_clone = Arc::clone(&self.ws_stream);
        let event_sender_clone = self.event_sender.clone();

        tokio::spawn(async move {
            info!("WebSocket event read loop started.");
            loop {
                let mut guard = ws_stream_clone.lock().await;
                let stream = match guard.as_mut() {
                    Some(s) => s,
                    None => {
                        info!("WebSocket stream is None, event read loop will exit.");
                        break;
                    }
                };

                match stream.next().await {
                    Some(Ok(message)) => {
                        if let Err(e) =
                            Self::handle_event_message(message, event_sender_clone.clone()).await
                        {
                            warn!("Error handling event message: {:?}", e);
                        }
                    }
                    Some(Err(e)) => {
                        error!("Error receiving event message: {:?}", e);
                        *guard = None;
                        break;
                    }
                    None => {
                        info!("WebSocket connection for events closed by server.");
                        *guard = None;
                        break;
                    }
                }
            }
            info!("WebSocket event read loop ended.");
        });

        Ok(())
    }

    async fn handle_event_message(msg: WsMessage, event_sender: mpsc::Sender<Event>) -> Result<()> {
        match msg {
            WsMessage::Text(text) => {
                debug!("Received event text: {}", text);
                match serde_json::from_str::<Event>(&text) {
                    Ok(event) => {
                        if event_sender.send(event).await.is_err() {
                            error!("Event receiver dropped, cannot send event.");
                        }
                    }
                    Err(e) => {
                        warn!(
                            "Failed to parse message as a known Event: {:?}. Raw: {}",
                            e, text
                        );
                    }
                }
            }
            WsMessage::Binary(_) => {
                info!("Received binary data on event stream (unhandled).");
            }
            WsMessage::Ping(_) => {
                debug!("Received Ping on event stream.");
            }
            WsMessage::Pong(_) => {
                debug!("Received Pong on event stream.");
            }
            WsMessage::Close(close_frame) => {
                info!("Received Close frame on event stream: {:?}", close_frame);
            }
            WsMessage::Frame(_) => {
                debug!("Received raw Frame on event stream (unhandled).");
            }
        }
        Ok(())
    }

    pub async fn send_request<P: Serialize, R: DeserializeOwned>(
        &self,
        action: &str,
        params: P,
    ) -> Result<R> {
        let full_api_url = self.api_base_url.join(action)?;
        debug!("Sending API request to: {}", full_api_url);

        let mut request_builder = self.http_client.post(full_api_url);
        if let Some(token) = &self.access_token {
            request_builder = request_builder.bearer_auth(token);
        }
        request_builder = request_builder.header(reqwest::header::CONTENT_TYPE, "application/json");

        let http_response = request_builder.json(&params).send().await?;

        let status = http_response.status();
        if status == StatusCode::OK {
            let api_resp = http_response.json::<ApiResponse<Value>>().await?;
            if api_resp.status == "ok" && api_resp.retcode == 0 {
                let data = api_resp.data.unwrap_or(Value::Null);
                serde_json::from_value(data).map_err(MilkyError::Json)
            } else {
                Err(MilkyError::ApiError {
                    message: api_resp
                        .message
                        .unwrap_or_else(|| "Unknown API error".to_string()),
                    retcode: Some(api_resp.retcode),
                })
            }
        } else {
            let error_message = http_response
                .text()
                .await
                .unwrap_or("Unknow error".to_string());
            Err(MilkyError::HttpApiError {
                status,
                message: error_message,
            })
        }
    }

    pub async fn close_event_stream(&self) -> Result<()> {
        info!("Closing WebSocket event stream.");
        let mut guard = self.ws_stream.lock().await;
        if let Some(mut stream) = guard.take() {
            if let Err(e) = stream.close(None).await {
                warn!("Error closing WebSocket event stream: {:?}", e);
            }
        }
        info!("WebSocket event stream closed.");
        Ok(())
    }
}
