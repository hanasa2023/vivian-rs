use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Friend {
    /// 好友 QQ 号
    pub user_id: i64,
    /// 好友 QID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qid: Option<String>,
    /// 好友昵称
    pub nickname: String,
    /// 好友备注
    pub remark: String,
    /// 好友分组（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<FriendCategory>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FriendCategory {
    /// 好友分组 ID
    pub category_id: i32,
    /// 好友分组名称
    pub category_name: String,
}
