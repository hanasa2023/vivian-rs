//! 定义与服务端的通信方式

/// 枚举了可以使用的通信方式。
#[derive(Clone)]
pub enum Communication {
    /// WebSocket
    WebSocket(WebSocketConfig),
    /// WebHook
    WebHook(WebHookConfig),
}

/// WebSocket的配置项
#[derive(Clone)]
pub struct WebSocketConfig {
    /// 服务端的WebSocket 接入点 e.g. `ws:127.0.0.1:3000`。
    pub ws_endpoint: String,
    /// 可选的访问令牌，用于认证。
    pub access_token: Option<String>,
}

impl WebSocketConfig {
    /// 创建一个新的 `WebSocketCofig` 实例。
    ///
    /// # 参数
    /// * `ws_endpoint`: 服务端的WebSocket 接入点 e.g. `ws:127.0.0.1:3000`。
    /// * `access_token`: 可选的访问令牌，用于认证。
    ///
    /// # 返回
    /// 成功则返回 `Result<Self>`，其中 `Self` 是新创建的 `WebSocket` 实例。
    /// 如果URL解析失败或协议不受支持，则返回错误。
    pub fn new(ws_endpoint: String, access_token: Option<String>) -> Self {
        Self {
            ws_endpoint,
            access_token,
        }
    }
}

/// WebHook的配置项
#[derive(Clone)]
pub struct WebHookConfig {
    /// http service主机地址，默认 `127.0.0.1`。
    pub host: String,
    /// 本机开放http service的端口。
    pub port: i32,
    /// 服务端的Http 接入点 e.g. `http://127.0.0.1:3000`。
    pub http_endpoint: String,
    /// 可选的访问令牌，用于认证。
    pub access_token: Option<String>,
}

impl WebHookConfig {
    /// 创建一个新的 `WebSocketCofig` 实例。
    ///
    /// # 参数
    /// * `host`: http service主机地址（可选），默认 `127.0.0.1`
    /// * `port`: 本机开放http service的端口。
    /// * `http_endpoint`: 服务端的Http服务 接入点 e.g. `https://127.0.0.1:3000`。
    /// * `access_token`: 可选的访问令牌，用于认证。
    ///
    /// # 返回
    /// 成功则返回 `Result<Self>`，其中 `Self` 是新创建的 `WebSocket` 实例。
    /// 如果URL解析失败或协议不受支持，则返回错误。
    pub fn new(
        host: Option<String>,
        port: i32,
        http_endpoint: String,
        access_token: Option<String>,
    ) -> Self {
        let host = match host {
            Some(_host) => _host,
            None => "127.0.0.1".to_string(),
        };
        Self {
            host,
            port,
            http_endpoint,
            access_token,
        }
    }
}
