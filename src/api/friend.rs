//! 提供了与好友互动相关的API接口功能，例如发送戳一戳和点赞。
//!
//! 这些功能通过 [`MilkyClient`] 的方法暴露出来。

use crate::error::Result;
use crate::{client::MilkyClient, types::friend::FriendRequest};
use serde::{Deserialize, Serialize};

/// 发送好友戳一戳的请求参数
#[derive(Serialize)]
pub struct SendFriendNudgeParams {
    /// 要戳一戳的好友的QQ号
    pub user_id: i64,
    /// 是否戳自己
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

/// 获取好友请求列表的请求参数。
#[derive(Serialize)]
pub struct GetFriendRequestsParams {
    /// 获取的最大请求数量，默认`20`
    pub limit: i32,
    /// `true` 表示只获取被过滤（由风险账号发起）的通知，`false` 表示只获取未被过滤的通知
    pub is_filtered: bool,
}

/// 获取好友请求列表的响应数据。
#[derive(Deserialize, Debug)]
pub struct GetFriendRequestsResponse {
    /// 好友请求列表
    pub requests: Vec<FriendRequest>,
}

/// 接受好友请求的请求参数。
#[derive(Serialize)]
pub struct AcceptFriendRequestParams {
    /// 请求发起者的UID
    pub initiator_uid: i64,
    /// 是否是被过滤的请求
    pub is_filtered: bool,
}

/// 拒绝好友请求的请求参数。
#[derive(Serialize)]
pub struct RejectFriendRequestParams {
    /// 请求发起者的UID
    pub initiator_uid: i64,
    /// 是否是被过滤的请求
    pub is_filtered: bool,
    /// 拒绝理由
    pub reason: String,
}

impl MilkyClient {
    /// 发送好友戳一戳（Nudge）。
    ///
    /// # 参数
    /// * `user_id`: 要戳一戳的好友的QQ号。
    /// * `is_self`: 可选参数，是否戳自己。如果为 `Some(true)`，则戳自己；如果为 `Some(false)` 或 `None`，则戳好友 `user_id`。默认为 `false`。
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
    /// * `count`: 可选参数，点赞的数量。如果为 `None`，则默认为 `1`。请注意平台可能对点赞次数有限制。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn send_profile_like(&self, user_id: i64, count: Option<i32>) -> Result<()> {
        // 如果 count 是 None，则使用默认值 1
        let count = count.unwrap_or(1);
        let params = SendProfileLikeParams { user_id, count };
        self.send_request("send_profile_like", params).await
    }

    /// 获取好友请求列表。
    ///
    /// # 参数
    /// * `limit`: 可选参数，获取的最大请求数量。默认为 `20`。
    /// * `is_filtered`: 可选参数，`true` 表示只获取被过滤的通知，`false` 表示只获取未被过滤的通知。默认为 `false`。
    /// # 返回
    /// 成功则返回 `Ok(GetFriendRequestsResponse)`，包含好友请求列表。
    pub async fn get_friend_requests(
        &self,
        limit: Option<i32>,
        is_filtered: Option<bool>,
    ) -> Result<GetFriendRequestsResponse> {
        // 如果 limit 是 None，则使用默认值 20
        let limit = limit.unwrap_or(20);
        // 如果 is_filtered 是 None，则使用默认值 false
        let is_filtered = is_filtered.unwrap_or(false);
        let params = GetFriendRequestsParams { limit, is_filtered };
        self.send_request("get_friend_requests", params).await
    }

    /// 接受好友请求。
    ///
    /// # 参数
    /// * `initiator_uid`: 请求发起者的UID。
    /// * `is_filtered`: 是否是被过滤的请求。
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn accept_friend_request(&self, initiator_uid: i64, is_filtered: bool) -> Result<()> {
        let params = AcceptFriendRequestParams {
            initiator_uid,
            is_filtered,
        };
        self.send_request("accept_friend_request", params).await
    }

    /// 拒绝好友请求。
    ///
    /// # 参数
    /// * `initiator_uid`: 请求发起者的UID。
    /// * `is_filtered`: 是否是被过滤的请求。
    /// * `reason`: 拒绝理由。
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn reject_friend_request(
        &self,
        initiator_uid: i64,
        is_filtered: bool,
        reason: String,
    ) -> Result<()> {
        let params = RejectFriendRequestParams {
            initiator_uid,
            is_filtered,
            reason,
        };
        self.send_request("reject_friend_request", params).await
    }
}
