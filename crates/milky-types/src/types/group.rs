//! 定义与群组及其相关信息（如成员、公告、文件等）的数据结构

use serde::{Deserialize, Serialize};

use crate::{
    common::RequestState,
    types::{common::Sex, message::in_coming::IncomingSegment},
};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum GroupRole {
    Owner,
    Admin,
    #[default]
    Member,
}

/// 代表一个群组的基本信息
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Group {
    /// 群号
    pub group_id: i64,
    /// 群组的名称
    pub group_name: String,
    /// 当前群组的成员数量
    pub member_count: i32,
    /// 群组的最大成员容量
    pub max_member_count: i32,
}

/// 代表一个群组成员的详细信息
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GroupMember {
    /// 用户QQ号
    pub user_id: i64,
    /// 用户昵称
    pub nickname: String,
    /// 用户性别
    /// 可能的值包括: "male" (男), "female" (女), "unknown" (未知)
    pub sex: Sex,
    /// 群号
    pub group_id: i64,
    /// 成员备注
    pub card: String,
    /// 专属头衔
    pub title: String,
    /// 群等级（注意与QQ等级区分）
    pub level: i32,
    /// 权限等级
    /// 可能的值包括: "owner" (群主), "admin" (管理员), "member" (普通成员)
    pub role: GroupRole,
    /// 加入群组时间，表示为Unix时间戳（秒）
    pub join_time: i64,
    /// 最后发言时间，表示为Unix时间戳（秒）
    pub last_sent_time: i64,
    /// 禁言结束时间，表示为Unix时间戳（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shut_up_end_time: Option<i64>,
}

/// 群精华消息
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GroupEssenceMessage {
    /// 群号
    pub group_id: i64,
    /// 消息序列号
    pub message_seq: i64,
    /// 消息发送时的 Unix 时间戳（秒）
    pub message_time: i64,
    /// 发送者 QQ 号
    pub sender_id: i64,
    /// 发送者名称
    pub sender_name: String,
    /// 设置精华的操作者 QQ 号
    pub operator_id: i64,
    /// 设置精华的操作者名称
    pub operator_name: String,
    /// 消息被设置精华时的 Unix 时间戳（秒）
    pub operation_time: i64,
    /// 消息段列表
    pub segments: Vec<IncomingSegment>,
}

/// 代表一条群公告的信息
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GroupAnnouncement {
    /// 群号
    pub group_id: i64,
    /// 公告的唯一ID
    pub announcement_id: String,
    /// 发布公告的成员的QQ号
    pub user_id: i64,
    /// 公告发布的时间，表示为Unix时间戳（秒）
    pub time: i64,
    /// 公告的文本内容
    pub content: String,
    /// 公告中附带的图片URL（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

/// 代表群文件系统中的一个文件
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GroupFile {
    /// 该文件所属群组的唯一标识符（群号）
    pub group_id: i64,
    /// 文件的唯一ID
    pub file_id: String,
    /// 文件的名称
    pub file_name: String,
    /// 父文件夹ID
    pub parent_folder_id: String,
    /// 文件的大小（字节）
    pub file_size: i64,
    /// 文件上传的时间，表示为Unix时间戳（秒）
    pub uploaded_time: i64,
    /// 文件过期的时间，表示为Unix时间戳（秒）
    pub expire_time: i64,
    /// 文件上传者的QQ号
    pub uploader_id: i64,
    /// 文件的下载次数
    pub downloaded_times: i32,
}

/// 代表群文件系统中的一个文件夹
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GroupFolder {
    /// 该文件夹所属群组的唯一标识符（群号）
    pub group_id: i64,
    /// 文件夹的唯一ID
    pub folder_id: String,
    /// 父文件夹ID
    pub parent_folder_id: String,
    /// 文件夹的名称
    pub folder_name: String,
    /// 文件夹创建的时间，表示为Unix时间戳（秒）
    pub created_time: i64,
    /// 文件夹最后修改的时间，表示为Unix时间戳（秒）
    pub last_modified_time: i64,
    /// 文件夹创建者的QQ号
    pub creator_id: i64,
    /// 文件夹中包含的文件数量
    pub file_count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupNotification {
    /// 群号
    pub group_id: i64,
    /// 操作者QQ号
    pub operator_id: i64,
    /// 通知序列号
    pub notification_seq: i64,
    /// 通知类型及相关数据
    #[serde(flatten)]
    pub notification_kind: GroupNotificationKind,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case", content = "")]
pub enum GroupNotificationKind {
    JoinRequest {
        is_filtered: bool,
        initiator_id: i64,
        state: RequestState,
        comment: String,
    },

    AdminChange {
        target_user_id: i64,
        is_set: bool,
    },

    Kick {
        target_user_id: i64,
    },

    Quit {
        target_user_id: i64,
    },

    InvitedJoinRequest {
        initiator_id: i64,
        target_user_id: i64,
        state: RequestState,
    },
}
