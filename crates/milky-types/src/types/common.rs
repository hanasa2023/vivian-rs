//! 定义了与API通信时通用的请求和响应数据结构

use serde::{Deserialize, Serialize};

/// 通用的API响应结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    /// 响应状态通常为 "ok" 表示成功，或 "failed" 表示失败。
    pub status: String,
    /// 返回码一个整数值，用于表示API调用的具体结果状态。
    /// `0` 代表成功，非零值代表不同类型的错误
    pub retcode: i64,
    /// 实际的响应数据
    pub data: Option<T>,
    /// 响应消息
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
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
#[serde(rename_all = "lowercase")]
pub enum Sex {
    Male,
    Female,
    #[default]
    Unknown,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum MessageScene {
    /// 好友消息场景
    Friend,
    /// 群组消息场景
    Group,
    /// 临时会话消息场景
    #[default]
    Temp,
}

/// 请求状态
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum RequestState {
    /// 等待处理
    #[default]
    Pending,
    /// 已接受
    Accepted,
    /// 已拒绝
    Rejected,
    /// 已忽略
    Ignored,
}

/// 图片类型枚举
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageSubType {
    /// 普通图片
    #[default]
    Normal,
    /// 贴图表情
    Sticker,
}