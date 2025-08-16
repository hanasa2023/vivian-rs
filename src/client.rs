//! 定义了 `MilkyClient`，这是与后端服务进行通信的核心客户端。
//!
//! `MilkyClient` 负责处理HTTP API请求以及通过WebSocket接收事件。
//! 它管理连接状态、认证信息，并提供了一系列方法来调用具体的API端点
//! 和处理从服务器推送的事件。

use crate::error::{MilkyError, Result};
use crate::types::common::ApiResponse;
use crate::types::communication::Communication;
use crate::types::event::Event;
use crate::types::message::OriginalMessage;

use axum::routing::post;
use axum::{Json, Router};
use futures_util::{StreamExt, lock::Mutex};
use log::{debug, error, info, warn};
use reqwest::StatusCode;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message as WsMessage,
};
use url::Url;

/// `MilkyClient` 是与后端服务交互的主要结构体。
///
/// 它封装了HTTP客户端、API基础URL、事件WebSocket URL、访问令牌（可选）、
/// WebSocket流的共享引用以及用于向上层传递事件的发送通道。
pub struct MilkyClient {
    /// 用于发送HTTP API请求的 `reqwest` 客户端实例。
    http_client: reqwest::Client,
    /// 与服务端的通信方式
    comm_type: Communication,
    /// API请求的基础URL，例如 `http://127.0.0.1:8080/api/`。
    api_base_url: Url,
    /// WebHook接收事件的URL
    event_wh_url: String,
    /// 事件WebSocket连接的URL，例如 `ws://127.0.0.1:8080/event`。
    event_ws_url: Option<Url>,
    /// 可选的访问令牌，用于API请求和WebSocket连接的认证。
    access_token: Option<String>,
    /// WebSocket流的可选共享引用。
    /// 使用 `Arc<Mutex<...>>` 来允许多个任务安全地访问和修改WebSocket流。
    /// `Option` 表示连接可能尚未建立或已关闭。
    ws_stream: Arc<Mutex<Option<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    // 用于发送关闭 WebSocket 的信号
    ws_shutdown_signal_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
    /// 用于将从WebSocket接收到的事件发送到上层处理逻辑的mpsc通道发送端。
    event_sender: mpsc::Sender<Event>,
}

impl MilkyClient {
    /// 创建一个新的 `MilkyClient` 实例。
    ///
    /// # 参数
    /// * `comm`: 与服务端的通信方式
    /// * `event_sender`: 一个mpsc通道的发送端，用于将接收到的事件传递出去。
    ///
    /// # 返回
    /// 成功则返回 `Result<Self>`，其中 `Self` 是新创建的 `MilkyClient` 实例。
    /// 如果URL解析失败或协议不受支持，则返回错误。
    pub fn new(comm: Communication, event_sender: mpsc::Sender<Event>) -> Result<Self> {
        let _comm = comm.clone();
        match comm {
            Communication::WebSocket(config) => {
                // 解析基础URL
                let ws_url = Url::parse(&config.ws_endpoint)?;

                // 构建API基础URL
                let scheme = match ws_url.scheme() {
                    "ws" => "http",
                    "wss" => "https",
                    _ => return Err(MilkyError::UnsupportedScheme(ws_url.scheme().to_string())),
                };
                let mut api_base_url = ws_url.clone();
                api_base_url
                    .set_scheme(scheme)
                    .map_err(|_| MilkyError::UrlParse(url::ParseError::InvalidPort))?;
                api_base_url.set_path("api/");

                // 构建事件WebSocket URL
                let mut event_ws_url = ws_url.clone();
                event_ws_url.set_path("event");
                if let Some(token) = &config.access_token {
                    // 如果有访问令牌，则添加到查询参数中
                    event_ws_url
                        .query_pairs_mut()
                        .append_pair("access_token", token);
                }

                Ok(Self {
                    http_client: reqwest::Client::new(),
                    api_base_url,
                    comm_type: _comm,
                    event_wh_url: String::new(),
                    event_ws_url: Some(event_ws_url),
                    access_token: config.access_token,
                    ws_stream: Arc::new(Mutex::new(None)),
                    ws_shutdown_signal_tx: Arc::new(Mutex::new(None)),
                    event_sender,
                })
            }
            Communication::WebHook(config) => {
                // 构建Event基础URL
                let event_base_url = format!("{}:{}", config.host, config.port);

                // 构建Event基础URL
                let mut api_base_url = Url::parse(&config.http_endpoint)?;
                api_base_url.set_path("api");

                Ok(Self {
                    http_client: reqwest::Client::new(),
                    comm_type: _comm,
                    api_base_url,
                    event_wh_url: event_base_url,
                    event_ws_url: None,
                    access_token: config.access_token,
                    ws_stream: Arc::new(Mutex::new(None)),
                    ws_shutdown_signal_tx: Arc::new(Mutex::new(None)),
                    event_sender,
                })
            }
        }
    }

    /// 尝试接收服务端发送的事件。
    ///
    /// # 返回
    /// 成功建立连接并启动事件读取循环则返回 `Ok(())`，否则返回错误。
    pub async fn connect_events(&self) -> Result<()> {
        match self.comm_type {
            Communication::WebSocket(_) => {
                let event_ws_url = self
                    .event_ws_url
                    .as_ref()
                    .ok_or_else(|| {
                        error!("WebSocket endpoint为空");
                        MilkyError::Internal("WebSocket URL未配置".to_string())
                    })?
                    .to_string();
                info!("正在连接 WebSocket 以接收事件: {event_ws_url}");
                // 异步连接WebSocket
                let (ws_stream_internal, response) = connect_async(event_ws_url.clone())
                    .await
                    .map_err(|e| MilkyError::WebSocket(Box::new(e)))?;
                info!("事件 WebSocket 握手成功完成！");
                debug!("响应的 HTTP 代码: {}", response.status());

                *self.ws_stream.lock().await = Some(ws_stream_internal);

                let (shutdown_tx, mut shutdown_rx) = oneshot::channel::<()>();
                *self.ws_shutdown_signal_tx.lock().await = Some(shutdown_tx);

                let ws_stream_clone = Arc::clone(&self.ws_stream);
                let event_sender_clone = self.event_sender.clone();
                let ws_shutdown_signal_tx_clone_for_loop = Arc::clone(&self.ws_shutdown_signal_tx);

                tokio::spawn(async move {
                    info!("WebSocket 事件读取循环已启动。");
                    loop {
                        tokio::select! {
                            biased;

                            _ = &mut shutdown_rx => {
                                info!("WebSocket 事件读取循环收到关闭信号。");
                                if let Some(mut stream_to_close) = ws_stream_clone.lock().await.take() {
                                    info!("正在发送 WebSocket Close 帧...");
                                    if let Err(e) = stream_to_close.close(None).await {
                                        error!("发送 WebSocket Close 帧时出错: {e:?}");
                                    } else {
                                        info!("WebSocket Close 帧已发送，连接已关闭。");
                                    }
                                }
                                break;
                            }

                            message_result = async {
                                let mut guard = ws_stream_clone.lock().await;
                                if let Some(stream) = guard.as_mut() {
                                    stream.next().await
                                } else {
                                    None
                                }
                            } => {
                                match message_result {
                                    Some(Ok(message)) => {
                                        if let Err(e) = Self::handle_event_message(
                                            OriginalMessage::Ws(message),
                                            event_sender_clone.clone(),
                                        )
                                        .await
                                        {
                                            warn!("处理WebSocket事件消息时出错: {e:?}");
                                        }
                                    }
                                    Some(Err(e)) => {
                                        error!("接收WebSocket事件消息时出错: {e:?}");
                                        ws_stream_clone.lock().await.take(); // 移除错误的流
                                        break; // 退出循环
                                    }
                                    None => { // 服务器关闭连接或流在读取前变为None
                                        info!("服务器关闭了事件 WebSocket 连接或流已不存在。");
                                        ws_stream_clone.lock().await.take(); // 确保流被移除
                                        break; // 退出循环
                                    }
                                }
                            }
                        }
                    }
                    info!("WebSocket 事件读取循环已结束。");
                    ws_shutdown_signal_tx_clone_for_loop.lock().await.take();
                });
            }
            Communication::WebHook(_) => {
                info!("正在为 WebHook 配置事件接收路由...");
                let event_sender_for_webhook = self.event_sender.clone();
                let webhook_listen_address = self.event_wh_url.clone();

                let axum_webhook_handler = move |Json(payload): Json<Value>| {
                    let sender_clone_for_call = event_sender_for_webhook.clone();
                    async move {
                        debug!("WebHook 接收到 payload: {payload:?}");
                        if let Err(e) = Self::handle_event_message(
                            OriginalMessage::WebHook(payload),
                            sender_clone_for_call,
                        )
                        .await
                        {
                            warn!("处理 WebHook 事件消息时出错: {e:?}");
                            // 返回一个错误响应给调用方
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("Failed to process webhook: {e:?}"),
                            )
                        } else {
                            // 返回成功响应
                            (StatusCode::OK, "Webhook received successfully".to_string())
                        }
                    }
                };

                let app = Router::new().route("/webhook", post(axum_webhook_handler));

                tokio::spawn(async move {
                    info!("尝试在 {webhook_listen_address} 上启动 WebHook 事件接收服务器。",);

                    let listener =
                        match tokio::net::TcpListener::bind(&webhook_listen_address).await {
                            Ok(l) => l,
                            Err(e) => {
                                error!("无法将 WebHook 监听器绑定到 {webhook_listen_address}: {e}");
                                return;
                            }
                        };
                    let actual_listen_addr = listener
                        .local_addr()
                        .map_or_else(|_| webhook_listen_address.clone(), |addr| addr.to_string());
                    info!("WebHook 事件接收服务器正在监听: http://{actual_listen_addr}");

                    let shutdown_signal = async {
                        let ctrl_c = async {
                            tokio::signal::ctrl_c()
                                .await
                                .expect("安装 Ctrl+C 处理器失败");
                        };

                        #[cfg(unix)]
                        let terminate = async {
                            tokio::signal::unix::signal(
                                tokio::signal::unix::SignalKind::terminate(),
                            )
                            .expect("安装信号处理器失败")
                            .recv()
                            .await;
                        };

                        #[cfg(not(unix))]
                        let terminate = std::future::pending::<()>();

                        tokio::select! {
                            _ = ctrl_c => info!("Ctrl+C信号接收，开始关闭 WebHook 服务器..."),
                            _ = terminate => info!("SIGTERM信号接收，开始关闭 WebHook 服务器..."),
                        }
                        info!("WebHook 服务器关闭信号已触发。");
                    };

                    if let Err(e) = axum::serve(listener, app.into_make_service())
                        .with_graceful_shutdown(shutdown_signal)
                        .await
                    {
                        error!("WebHook 事件接收服务器遇到错误: {e:?}");
                    }
                    info!("WebHook 事件接收服务器已关闭。");
                });
                info!("WebHook 事件接收服务器已安排在后台运行。");
            }
        };

        Ok(())
    }

    /// 优雅地关闭与服务器的连接。
    ///
    /// 目前主要用于主动关闭 WebSocket 事件流连接。
    /// 它会向事件读取循环发送一个关闭信号。
    pub async fn shutdown(&self) {
        info!("正在请求关闭 MilkyClient...");
        if let Some(tx) = self.ws_shutdown_signal_tx.lock().await.take() {
            if tx.send(()).is_ok() {
                info!("已成功发送关闭信号到 WebSocket 事件读取循环。");
            } else {
                info!("无法发送关闭信号，WebSocket 事件读取循环可能已经关闭。");
            }
        } else {
            info!("没有活动的 WebSocket 关闭信号发送器，可能连接从未完全建立或已被关闭。");
        }
    }

    /// 处理接收到的单个事件消息。
    ///
    /// 此方法会解析消息内容，如果消息是文本类型并且可以成功反序列化为 [`Event`]，
    /// 则通过 `event_sender` 将事件发送出去。
    ///
    /// # 参数
    /// * `msg`: 接收到的原始 [`OriginalMessage`]。
    /// * `event_sender`: 用于发送解析后事件的mpsc通道发送端。
    ///
    /// # 返回
    /// 成功处理则返回 `Ok(())`，否则返回错误（主要是在发送事件到通道失败时）。
    async fn handle_event_message(
        msg: OriginalMessage,
        event_sender: mpsc::Sender<Event>,
    ) -> Result<()> {
        match msg {
            OriginalMessage::Ws(ws_msg) => match ws_msg {
                WsMessage::Text(text) => {
                    debug!("接收到事件文本: {text}",);
                    match serde_json::from_str::<Event>(&text) {
                        Ok(event) => {
                            if event_sender.send(event).await.is_err() {
                                error!("事件接收端已关闭，无法发送事件。");
                            }
                        }
                        Err(e) => {
                            warn!("无法将消息解析为已知的 Event 类型: {e}。原始文本: {text}");
                        }
                    }
                }
                WsMessage::Binary(_) => {
                    info!("在事件流上接收到二进制数据 (未处理)。");
                }
                WsMessage::Ping(_) => {
                    debug!("在事件流上接收到 Ping 帧。");
                }
                WsMessage::Pong(_) => {
                    debug!("在事件流上接收到 Pong 帧。");
                }
                WsMessage::Close(close_frame) => {
                    info!("在事件流上接收到 Close 帧: {close_frame:?}",);
                }
                WsMessage::Frame(_) => {
                    debug!("在事件流上接收到原始 Frame (未处理)。");
                }
            },
            OriginalMessage::WebHook(wh_msg) => {
                let msg = wh_msg.clone();
                match serde_json::from_value::<Event>(wh_msg) {
                    Ok(event) => {
                        if event_sender.send(event).await.is_err() {
                            error!("事件接收端已关闭，无法发送事件。");
                        }
                    }
                    Err(e) => {
                        warn!("无法将消息解析为已知的 Event 类型: {e}。原始文本: {msg:?}");
                    }
                }
            }
        }
        Ok(())
    }

    /// 发送一个API请求到后端服务。
    ///
    /// 这是一个泛型方法，用于发送POST请求到指定的API `action` 端点。
    /// 请求参数 `params` 会被序列化为JSON。
    /// 响应会被尝试反序列化为类型 `R`。
    ///
    /// # 类型参数
    /// * `P`: 请求参数的类型，必须实现 `serde::Serialize`。
    /// * `R`: 期望的响应数据类型，必须实现 `serde::de::DeserializeOwned`。
    ///
    /// # 参数
    /// * `action`: API操作的名称，例如 "send_private_msg"。
    /// * `params`: 要发送的请求参数。
    ///
    /// # 返回
    /// 成功则返回 `Result<R>`，其中 `R` 是反序列化后的响应数据。
    /// 如果请求失败、服务器返回错误或反序列化失败，则返回错误。
    pub async fn send_request<P: Serialize, R: DeserializeOwned>(
        &self,
        action: &str,
        params: P,
    ) -> Result<R> {
        // 构建完整的API URL
        let full_api_url = self.api_base_url.join(action)?;
        debug!("正在发送 API 请求至: {full_api_url}",);

        // 构建HTTP POST请求
        let mut request_builder = self.http_client.post(full_api_url);
        if let Some(token) = &self.access_token {
            // 如果有访问令牌，则添加Bearer Token认证头
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
                        .unwrap_or_else(|| "未知的 API 错误".to_string()),
                    retcode: Some(api_resp.retcode),
                })
            }
        } else {
            let error_message = http_response
                .text()
                .await
                .unwrap_or("未知的 HTTP 错误".to_string());
            Err(MilkyError::HttpApiError {
                status,
                message: error_message,
            })
        }
    }
}
