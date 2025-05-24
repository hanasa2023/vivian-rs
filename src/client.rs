//! 定义了 `MilkyClient`，这是与后端服务进行通信的核心客户端。
//!
//! `MilkyClient` 负责处理HTTP API请求以及通过WebSocket接收事件。
//! 它管理连接状态、认证信息，并提供了一系列方法来调用具体的API端点
//! 和处理从服务器推送的事件。

use crate::error::{MilkyError, Result};
use crate::types::common::ApiResponse;
use crate::types::event::Event;

use futures_util::{StreamExt, lock::Mutex};
use log::{debug, error, info, warn}; // 日志记录宏
use reqwest::StatusCode; // HTTP状态码
use serde::{Serialize, de::DeserializeOwned}; // 序列化与反序列化相关
use serde_json::Value;
use std::sync::Arc; // 原子引用计数，用于多线程共享数据
use tokio::net::TcpStream; // Tokio提供的异步TCP流
use tokio::sync::mpsc; // Tokio提供的多生产者单消费者异步通道
use tokio_tungstenite::tungstenite::protocol::CloseFrame; // 用于处理动态JSON数据
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message as WsMessage,
}; // WebSocket相关
use url::Url; // URL处理

/// `MilkyClient` 是与后端服务交互的主要结构体。
///
/// 它封装了HTTP客户端、API基础URL、事件WebSocket URL、访问令牌（可选）、
/// WebSocket流的共享引用以及用于向上层传递事件的发送通道。
pub struct MilkyClient {
    /// 用于发送HTTP API请求的 `reqwest` 客户端实例。
    http_client: reqwest::Client,
    /// API请求的基础URL，例如 `http://127.0.0.1:8080/api/`。
    api_base_url: Url,
    /// 事件WebSocket连接的URL，例如 `ws://127.0.0.1:8080/event`。
    event_ws_url: Url,
    /// 可选的访问令牌，用于API请求和WebSocket连接的认证。
    access_token: Option<String>,
    /// WebSocket流的可选共享引用。
    /// 使用 `Arc<Mutex<...>>` 来允许多个任务安全地访问和修改WebSocket流。
    /// `Option` 表示连接可能尚未建立或已关闭。
    ws_stream: Arc<Mutex<Option<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    /// 用于将从WebSocket接收到的事件发送到上层处理逻辑的mpsc通道发送端。
    event_sender: mpsc::Sender<Event>,
}

impl MilkyClient {
    /// 创建一个新的 `MilkyClient` 实例。
    ///
    /// # 参数
    /// * `host_address`: 后端服务的主机地址，例如 `"http://127.0.0.1:8080"`。
    /// * `access_token`: 可选的访问令牌，用于认证。
    /// * `event_sender`: 一个mpsc通道的发送端，用于将接收到的事件传递出去。
    ///
    /// # 返回
    /// 成功则返回 `Result<Self>`，其中 `Self` 是新创建的 `MilkyClient` 实例。
    /// 如果URL解析失败或协议不受支持，则返回错误。
    pub fn new(
        host_address: &str, // 例如: "http://127.0.0.1:8080"
        access_token: Option<String>,
        event_sender: mpsc::Sender<Event>,
    ) -> Result<Self> {
        // 解析基础URL
        let base_url = Url::parse(host_address)?;

        // 构建API基础URL
        let mut api_base_url_str = base_url.to_string();
        if !api_base_url_str.ends_with('/') {
            api_base_url_str.push('/');
        }
        let api_base_url = Url::parse(&api_base_url_str)?.join("api/")?;

        // 构建事件WebSocket URL
        let mut event_ws_url = base_url.clone();
        let scheme = match base_url.scheme() {
            "http" => "ws",
            "https" => "wss",
            _ => return Err(MilkyError::UnsupportedScheme(base_url.scheme().to_string())),
        };
        event_ws_url
            .set_scheme(scheme)
            .map_err(|_| MilkyError::UrlParse(url::ParseError::InvalidPort))?;
        event_ws_url.set_path("event");
        if let Some(token) = &access_token {
            // 如果有访问令牌，则添加到查询参数中
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

    /// 连接到WebSocket以接收事件。
    ///
    /// 此方法会尝试建立到 `event_ws_url` 的WebSocket连接。
    /// 连接成功后，会启动一个新的异步任务来持续读取和处理来自服务器的事件消息。
    ///
    /// # 返回
    /// 成功建立连接并启动事件读取循环则返回 `Ok(())`，否则返回错误。
    pub async fn connect_events(&self) -> Result<()> {
        info!("正在连接 WebSocket 以接收事件: {}", self.event_ws_url);
        // 异步连接WebSocket
        let (ws_stream_internal, response) = connect_async(self.event_ws_url.clone())
            .await
            .map_err(MilkyError::WebSocket)?;
        info!("事件 WebSocket 握手成功完成！");
        debug!("响应的 HTTP 代码: {}", response.status());

        *self.ws_stream.lock().await = Some(ws_stream_internal);

        let ws_stream_clone = Arc::clone(&self.ws_stream);
        let event_sender_clone = self.event_sender.clone();

        tokio::spawn(async move {
            info!("WebSocket 事件读取循环已启动。");
            loop {
                let mut guard = ws_stream_clone.lock().await;
                let stream = match guard.as_mut() {
                    Some(s) => s,
                    None => {
                        info!("WebSocket 流为空，事件读取循环将退出。");
                        break;
                    }
                };

                match stream.next().await {
                    Some(Ok(message)) => {
                        if let Err(e) =
                            Self::handle_event_message(message, event_sender_clone.clone()).await
                        {
                            warn!("处理事件消息时出错: {:?}", e);
                        }
                    }
                    Some(Err(e)) => {
                        error!("接收事件消息时出错: {:?}", e);
                        *guard = None;
                        break;
                    }
                    None => {
                        info!("服务器关闭了事件 WebSocket 连接。");
                        *guard = None;
                        break;
                    }
                }
            }
            info!("WebSocket 事件读取循环已结束。");
        });

        Ok(())
    }

    /// 处理从WebSocket接收到的单个事件消息。
    ///
    /// 此方法会解析消息内容，如果消息是文本类型并且可以成功反序列化为 [`Event`]，
    /// 则通过 `event_sender` 将事件发送出去。
    ///
    /// # 参数
    /// * `msg`: 从WebSocket接收到的原始 [`WsMessage`]。
    /// * `event_sender`: 用于发送解析后事件的mpsc通道发送端。
    ///
    /// # 返回
    /// 成功处理则返回 `Ok(())`，否则返回错误（主要是在发送事件到通道失败时）。
    async fn handle_event_message(msg: WsMessage, event_sender: mpsc::Sender<Event>) -> Result<()> {
        match msg {
            WsMessage::Text(text) => {
                debug!("接收到事件文本: {}", text);
                match serde_json::from_str::<Event>(&text) {
                    Ok(event) => {
                        if event_sender.send(event).await.is_err() {
                            error!("事件接收端已关闭，无法发送事件。");
                        }
                    }
                    Err(e) => {
                        warn!(
                            "无法将消息解析为已知的 Event 类型: {:?}。原始文本: {}",
                            e, text
                        );
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
                info!("在事件流上接收到 Close 帧: {:?}", close_frame);
            }
            WsMessage::Frame(_) => {
                debug!("在事件流上接收到原始 Frame (未处理)。");
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
        debug!("正在发送 API 请求至: {}", full_api_url);

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

    /// 关闭当前的WebSocket事件流连接。
    ///
    /// 如果连接存在，则会尝试发送一个Close帧并等待关闭完成。
    ///
    /// # 返回
    /// 成功关闭或连接本就不存在则返回 `Ok(())`。如果在关闭过程中发生错误，则会记录警告。
    pub async fn close_event_stream(&self) -> Result<()> {
        info!("正在关闭 WebSocket 事件流。");
        let mut guard = self.ws_stream.lock().await;
        if let Some(mut stream) = guard.take() {
            if let Err(e) = stream
                .close(Some(CloseFrame {
                    code: CloseCode::Normal,
                    reason: "关闭事件流".into(),
                }))
                .await
            {
                warn!("关闭 WebSocket 事件流时出错: {:?}", e);
            }
        }
        info!("WebSocket 事件流已关闭。");
        Ok(())
    }
}
