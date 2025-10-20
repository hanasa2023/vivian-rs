//! 定义了 `MilkyClient` 及其相关操作中可能发生的各种错误类型。
//!
//! 本模块主要包含一个 [`MilkyError`] 枚举，它整合了来自不同库（如 `tungstenite`、`url`、`serde_json`、`reqwest`）的错误，
//! 以及特定于本应用程序逻辑的自定义错误。
//! 同时，提供了一个统一的 [`Result<T>`] 类型别名，以便在整个库中方便地使用。

use reqwest;
use thiserror::Error;
use tokio_tungstenite::tungstenite;

/// `MilkyClient` 操作中可能发生的错误枚举。
///
/// 使用 `thiserror::Error` 宏来自动派生 `std::error::Error` trait 的实现，
/// 并为每个错误变体提供用户友好的描述信息。
#[derive(Error, Debug)]
pub enum MilkyError {
    /// WebSocket 通信过程中发生的错误。
    /// 通常由底层的 `tokio-tungstenite` 库引发。
    #[error("WebSocket 错误: {0}")]
    WebSocket(#[from] Box<tungstenite::Error>),

    /// URL 解析失败时发生的错误。
    /// 例如，当提供的服务器地址或API端点格式不正确时。
    #[error("URL 解析错误: {0}")]
    UrlParse(#[from] url::ParseError),

    /// 当 URL 使用了不支持的协议方案（scheme）时发生的错误。
    /// 例如，客户端可能只支持 "http" 和 "https" (以及对应的 "ws", "wss")。
    #[error("不支持的协议方案: {0}")]
    UnsupportedScheme(String),

    /// JSON 序列化或反序列化过程中发生的错误。
    /// 这可能发生在构造API请求或解析API响应时。
    #[error("JSON 序列化/反序列化错误: {0}")]
    Json(#[from] serde_json::Error),

    /// 标准输入/输出 (I/O) 操作发生的错误。
    /// 例如，在读取配置文件或写入日志时可能发生。
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    /// API 请求失败，通常表示服务器成功处理了请求但返回了一个业务逻辑上的错误。
    /// 例如，权限不足、参数错误等。
    #[error("API 请求失败: {message}")]
    ApiError {
        /// 来自服务器的错误描述信息。
        message: String,
        /// 来自服务器的特定返回码（retcode），有助于定位具体错误原因。
        retcode: Option<i64>,
    },

    /// HTTP API 请求返回了非成功状态码（例如 4xx, 5xx）。
    /// 这表示 HTTP 请求本身可能已发送，但服务器响应了一个 HTTP 错误。
    #[error("HTTP API 错误: {message}")]
    HttpApiError {
        /// HTTP 响应的状态码。
        status: reqwest::StatusCode,
        /// 从服务器响应体中获取的错误信息，或者一个通用的错误描述。
        message: String,
    },

    /// 表示 WebSocket 连接尚未建立或已经丢失。
    /// 尝试在未连接状态下进行需要连接的操作时可能发生。
    #[error("连接未建立或已丢失")]
    NotConnected,

    /// 操作（如API请求）等待响应超时。
    #[error("响应超时")]
    Timeout,

    /// 收到了非预期的响应类型。
    /// 例如，期望一个特定的JSON结构但收到了其他格式。
    #[error("收到了非预期的响应类型")]
    UnexpectedResponse,

    /// 当请求中的 `echo` 字段与响应中的 `echo` 字段不匹配时发生。
    /// 用于验证异步请求和响应的对应关系。
    #[error("操作的 echo 字段不匹配")]
    EchoMismatch,

    /// 底层 HTTP 请求库 (`reqwest`) 发生的错误。
    /// 例如，网络连接问题、DNS解析失败等。
    #[error("HTTP 请求错误: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("内部错误: {0}")]
    Internal(String),
}

/// 一个统一的 `Result` 类型别名，用于 `MilkyClient` 的所有操作。
///
/// 它简化了函数签名，其中 `T` 是成功情况下的返回值类型，
/// 错误类型固定为 [`MilkyError`]。
pub type Result<T> = std::result::Result<T, MilkyError>;
