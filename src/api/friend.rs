use crate::client::MilkyClient;
use crate::error::Result;
use serde::Serialize;

#[derive(Serialize)]
pub struct SendFriendNudgeParams {
    /// 好友QQ号
    pub user_id: i64,
    /// 是否戳自己（默认值：false）
    #[serde(default)]
    pub is_self: bool,
}

#[derive(Serialize)]
pub struct SendProfileLikeParams {
    /// 好友 QQ 号
    pub user_id: i64,
    /// 点赞数量（默认值：1）
    pub count: i32,
}

impl MilkyClient {
    pub async fn send_friend_nudge(&self, user_id: i64, is_self: Option<bool>) -> Result<()> {
        let is_self = is_self.unwrap_or(false);
        let params = SendFriendNudgeParams { user_id, is_self };
        self.send_request("send_friend_nudge", params).await
    }

    pub async fn send_profile_like(&self, user_id: i64, count: Option<i32>) -> Result<()> {
        let count = count.unwrap_or(1);
        let params = SendProfileLikeParams { user_id, count };
        self.send_request("send_profile_like", params).await
    }
}
