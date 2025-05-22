//! 定义与好友及其相关信息（如好友分组）的数据结构。

use serde::{Deserialize, Serialize};

/// 代表一个好友的基本信息。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Friend {
    /// 好友的QQ号。
    pub user_id: i64,
    /// 好友的QID（一种可选的唯一标识符）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qid: Option<String>,
    /// 好友的昵称。
    pub nickname: String,
    /// 您为好友设置的备注名称。
    pub remark: String,
    /// 好友所属的分组信息（可选）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<FriendCategory>,
}

/// 代表一个好友分组的信息。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FriendCategory {
    /// 好友分组的唯一ID。
    pub category_id: i32,
    /// 好友分组的名称。
    pub category_name: String,
}
