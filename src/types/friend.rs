//! 定义与好友及其相关信息（如好友分组）的数据结构。

use serde::{Deserialize, Serialize};

use crate::types::common::Sex;

/// 代表好友请求的状态。
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FriendRequestState {
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

// TODO: Option?
/// 代表一个好友的基本信息。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Friend {
    /// 好友的QQ号。
    pub user_id: i64,
    /// 好友的昵称。
    pub nickname: String,
    /// 性别，可能值 "male", "female", "unknown"
    pub sex: Sex,
    /// 好友的QID（一种可选的唯一标识符）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qid: Option<String>,
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

/// 好友请求实体
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FriendRequest {
    /// 请求发起时的 Unix 时间戳（秒）
    pub time: i64,
    /// 请求发起者 QQ 号
    pub initiator_id: i64,
    /// 请求发起者 UID
    pub initiator_uid: String,
    /// 目标用户 QQ 号
    pub target_user_id: i64,
    /// 目标用户 UID
    pub target_user_uid: String,
    /// 请求状态
    pub state: FriendRequestState,
    /// 申请附加信息
    pub comment: String,
    /// 申请来源
    pub via: String,
    /// 请求是否被过滤（发起自风险账户）
    pub is_filtered: bool,
}
