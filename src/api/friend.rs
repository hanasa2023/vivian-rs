//! 提供了与好友互动相关的API接口功能，例如发送戳一戳和点赞。
//!
//! 这些功能通过 [`MilkyClient`] 的方法暴露出来。

use crate::client::MilkyClient;
use crate::error::Result;
use serde::Serialize;

/// 发送好友戳一戳（Nudge）的请求参数。
#[derive(Serialize)]
pub struct SendFriendNudgeParams {
    /// 要戳一戳的好友的QQ号。
    pub user_id: i64,
    /// 是否戳自己。如果为 `true`，则表示机器人戳自己（在与 `user_id` 的聊天窗口中）。
    /// 默认为 `false`。
    /// `#[serde(default)]` 确保如果调用者未提供此字段，则使用类型的默认值（对于bool是false）。
    #[serde(default)]
    pub is_self: bool,
}

/// 发送资料卡点赞的请求参数。
#[derive(Serialize)]
pub struct SendProfileLikeParams {
    /// 要点赞的好友的QQ号。
    pub user_id: i64,
    /// 点赞的数量。通常平台对可点赞次数有限制。
    /// 默认为 `1`。
    pub count: i32,
}

impl MilkyClient {
    /// 发送好友戳一戳（Nudge）。
    ///
    /// # 参数
    /// * `user_id`: 要戳一戳的好友的QQ号。
    /// * `is_self`: 可选参数，是否戳自己。如果为 `Some(true)`，则戳自己；
    ///              如果为 `Some(false)` 或 `None`，则戳好友 `user_id`。默认为 `false`。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn send_friend_nudge(&self, user_id: i64, is_self: Option<bool>) -> Result<()> {
        // 如果 is_self 是 None，则使用默认值 false
        let is_self = is_self.unwrap_or(false);
        let params = SendFriendNudgeParams { user_id, is_self };
        self.send_request("send_friend_nudge", params).await
    }

    /// 为好友的资料卡点赞。
    ///
    /// # 参数
    /// * `user_id`: 要点赞的好友的QQ号。
    /// * `count`: 可选参数，点赞的数量。如果为 `None`，则默认为 `1`。
    ///            请注意平台可能对点赞次数有限制。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn send_profile_like(&self, user_id: i64, count: Option<i32>) -> Result<()> {
        // 如果 count 是 None，则使用默认值 1
        let count = count.unwrap_or(1);
        let params = SendProfileLikeParams { user_id, count };
        self.send_request("send_profile_like", params).await
    }
}
