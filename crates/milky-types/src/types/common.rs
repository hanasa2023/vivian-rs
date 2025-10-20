//! 定义了与API通信时通用的请求和响应数据结构。

use serde::{Deserialize, Serialize};

/// 通用的API响应结构体。
///
/// 这是一个泛型结构体，用于封装从API接收到的标准响应格式。
/// `T` 代表响应数据（`data`字段）的具体类型。
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    /// 响应状态。通常为 "ok" 表示成功，或 "failed" 表示失败。
    pub status: String,
    /// 返回码。一个整数值，用于表示API调用的具体结果状态。
    /// 通常，`0` 代表成功，非零值代表不同类型的错误。
    pub retcode: i64,
    /// 实际的响应数据。这是一个可选字段，因为并非所有API调用都会返回数据，
    /// 或者在发生错误时可能没有数据。`T` 是数据的具体类型。
    pub data: Option<T>,
    /// 响应消息。这是一个可选字段，通常在API调用失败时提供错误信息，
    /// 或者在成功时提供一些附加的提示信息。
    pub message: Option<String>,
}

/// 通用的API请求结构体。
///
/// 这是一个泛型结构体，用于构建发送到API的请求。
/// `P` 代表请求参数（`params`字段）的具体类型。
#[derive(Serialize, Debug, Clone)]
pub struct ApiRequest<P> {
    /// 要执行的API操作或动作的名称。
    pub action: String,
    /// 针对指定 `action` 的具体参数。`P` 是参数的数据结构类型。
    pub params: P,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Platform {
    Windows,
    Linux,
    MacOS,
    AndroidPad,
    AndroidPhone,
    Ipad,
    IPhone,
    Harmony,
    Watch,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum Sex {
    Male,
    Female,
    #[default]
    Unknown,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum MessageScene {
    /// 好友消息场景
    Friend,
    /// 群组消息场景
    Group,
    /// 临时会话消息场景
    #[default]
    Temp,
}
