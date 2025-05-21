use serde::{Deserialize, Serialize};

/// 通用响应结构
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    pub status: String, // "ok" or "failed"
    pub retcode: i64,
    pub data: Option<T>,
    pub message: Option<String>,
}

/// 通用请求结构
#[derive(Serialize, Debug, Clone)]
pub struct ApiRequest<P> {
    pub action: String,
    pub params: P,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<String>,
}
