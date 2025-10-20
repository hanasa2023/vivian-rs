//! 提供了处理各类请求（如好友请求、加群请求）的API接口功能。
//!
//! 这通常涉及到接受或拒绝由平台事件（如 [`FriendRequestData`](crate::types::event::FriendRequestData)
//! 或 [`GroupJoinRequestData`](crate::types::event::GroupJoinRequestData)）通知的请求。
//! 所有功能均通过 [`MilkyClient`] 的方法暴露。

use crate::client::MilkyClient;
use crate::error::Result;
use serde::Serialize;

/// 同意（接受）一个请求的参数。
#[derive(Serialize)]
pub struct AcceptRequestParams {
    /// 要同意的请求的唯一ID。
    /// 这个ID通常从相应的事件数据中获取，例如好友请求事件或加群请求事件。
    pub request_id: String,
}

/// 拒绝一个请求的参数。
#[derive(Serialize)]
pub struct RejectRequestParams {
    /// 要拒绝的请求的唯一ID。
    /// 这个ID通常从相应的事件数据中获取。
    pub request_id: String,
    /// 拒绝请求的理由（可选）。
    /// 如果提供，这个理由可能会展示给发起请求的用户。
    pub reason: Option<String>,
}

impl MilkyClient {
    /// 同意（接受）一个指定的请求。
    ///
    /// 这可以用于例如同意好友请求、同意用户加入群组的请求等。
    ///
    /// # 参数
    /// * `request_id`: 要接受的请求的唯一ID。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn accept_request(&self, request_id: &str) -> Result<()> {
        let params = AcceptRequestParams {
            request_id: request_id.to_string(),
        };
        self.send_request("accept_request", params).await
    }

    /// 拒绝一个指定的请求。
    ///
    /// 这可以用于例如拒绝好友请求、拒绝用户加入群组的请求等。
    ///
    /// # 参数
    /// * `request_id`: 要拒绝的请求的唯一ID。
    /// * `reason`: 可选的拒绝理由。如果为 `None`，则不提供具体理由。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn reject_request(&self, request_id: &str, reason: Option<&str>) -> Result<()> {
        // 将 Option<&str> 转换为 Option<String>
        let reason = reason.map(|s| s.to_string());
        let params = RejectRequestParams {
            request_id: request_id.to_string(),
            reason,
        };
        self.send_request("reject_request", params).await
    }
}
