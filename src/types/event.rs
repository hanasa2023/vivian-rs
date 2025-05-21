use crate::types::message::in_coming::IncomingMessage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    /// 事件 Unix 时间戳（秒）
    pub time: i64,
    /// 机器人 QQ 号
    pub self_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "event_type", content = "data")]
pub enum EventKind {
    #[serde(rename = "message_receive")]
    MessageReceive(IncomingMessage),
    #[serde(rename = "message_recall")]
    MessageRecall(MessageRecallData),
    #[serde(rename = "friend_request")]
    FriendRequest(FriendRequestData),
    #[serde(rename = "group_join_request")]
    GroupJoinRequest(GroupJoinRequestData),
    #[serde(rename = "group_invited_join_request")]
    GroupInvitedJoinRequest(GroupInvitedJoinRequestData),
    #[serde(rename = "group_invitation_request")]
    GroupInvitationRequest(GroupInvitationRequestData),
    #[serde(rename = "friend_nudge")]
    FriendNudge(FriendNudgeData),
    #[serde(rename = "friend_file_upload")]
    FriendFileUpload(FriendFileUploadData),
    #[serde(rename = "group_admin_change")]
    GroupAdminChange(GroupAdminChangeData),
    #[serde(rename = "group_essence_message_change")]
    GroupEssenceMessageChange(GroupEssenceMessageChangeData),
    #[serde(rename = "group_member_increase")]
    GroupMemberIncrease(GroupMemberIncreaseData),
    #[serde(rename = "group_member_decrease")]
    GroupMemberDecrease(GroupMemberDecreaseData),
    #[serde(rename = "group_name_change")]
    GroupNameChange(GroupNameChangeData),
    #[serde(rename = "group_message_reaction")]
    GroupMessageReaction(GroupMessageReactionData),
    #[serde(rename = "group_mute")]
    GroupMute(GroupMuteData),
    #[serde(rename = "group_whole_mute")]
    GroupWholeMute(GroupWholeMuteData),
    #[serde(rename = "group_nudge")]
    GroupNudge(GroupNudgeData),
    #[serde(rename = "group_file_upload")]
    GroupFileUpload(GroupFileUploadData),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageRecallData {
    /// 消息场景（可能值：`friend`, `group`, `temp`）
    pub message_scene: String,
    /// 好友 QQ 号或群号
    pub peer_id: i64,
    /// 消息序列号
    pub message_seq: i64,
    /// 被撤回的消息的发送者 QQ 号
    pub sender_id: i64,
    /// 操作者 QQ 号
    pub operator_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriendRequestData {
    /// 请求 ID，用于同意 / 拒绝请求
    pub request_id: String,
    /// 发起请求的用户 QQ 号
    pub operator_id: i64,
    /// 好友请求附加信息（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// 好友请求来源（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupJoinRequestData {
    /// 请求 ID，用于同意 / 拒绝请求
    pub request_id: String,
    /// 发起请求的用户 QQ 号
    pub operator_id: i64,
    /// 群号
    pub group_id: i64,
    /// 入群请求附加信息（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupInvitedJoinRequestData {
    /// 请求 ID，用于同意 / 拒绝请求
    pub request_id: String,
    /// 发起请求的用户 QQ 号
    pub operator_id: i64,
    /// 群号
    pub group_id: i64,
    /// 被邀请者 QQ 号
    pub invitee_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupInvitationRequestData {
    /// 请求 ID，用于同意 / 拒绝请求
    pub request_id: String,
    /// 发起请求的用户 QQ 号
    pub operator_id: i64,
    /// 群号
    pub group_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriendNudgeData {
    /// 好友 QQ 号
    pub user_id: i64,
    /// 是否是自己发送的戳一戳
    pub is_self_send: bool,
    /// 是否是自己接收的戳一戳
    pub is_self_receive: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriendFileUploadData {
    /// 好友 QQ 号
    pub user_id: i64,
    /// 文件 ID
    pub file_id: String,
    /// 文件名称
    pub file_name: String,
    /// 文件大小
    pub file_size: i64,
    /// 是否是自己发送的文件
    pub is_self: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupAdminChangeData {
    /// 群号
    pub group_id: i64,
    /// 发生变更的用户 QQ 号
    pub user_id: i64,
    /// 是否被设置为管理员，`false` 表示被取消管理员
    pub is_set: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupEssenceMessageChangeData {
    /// 群号
    pub group_id: i64,
    /// 发生变更的消息序列号
    pub message_seq: i64,
    /// 是否被设置为精华，`false` 表示被取消精华
    pub is_set: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMemberIncreaseData {
    /// 群号
    pub group_id: i64,
    /// 发生变更的用户 QQ 号
    pub user_id: i64,
    /// 管理员 QQ 号，如果是管理员同意入群（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<i64>,
    /// 邀请者 QQ 号，如果是邀请入群（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitor_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMemberDecreaseData {
    /// 群号
    pub group_id: i64,
    /// 发生变更的用户 QQ 号
    pub user_id: i64,
    /// 管理员 QQ 号，如果是管理员踢出（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupNameChangeData {
    /// 群号
    pub group_id: i64,
    /// 新的群名称
    pub name: String,
    /// 操作者 QQ 号
    pub operator_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMessageReactionData {
    /// 群号
    pub group_id: i64,
    /// 发送回应者 QQ 号
    pub user_id: i64,
    /// 消息序列号
    pub message_seq: i64,
    /// 表情 ID
    pub face_id: String,
    /// 是否为添加，`false` 表示取消回应（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_add: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMuteData {
    /// 群号
    pub group_id: i64,
    /// 发生变更的用户 QQ 号
    pub user_id: i64,
    /// 禁言时长（秒），为 0 表示取消禁言
    pub duration: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupWholeMuteData {
    /// 群号
    pub group_id: i64,
    /// 操作者 QQ 号
    pub operator_id: i64,
    /// 是否全员禁言，`false` 表示取消全员禁言
    pub is_mute: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupNudgeData {
    /// 群号
    pub group_id: i64,
    /// 发送者 QQ 号
    pub sender_id: i64,
    /// 接收者 QQ 号
    pub receiver_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupFileUploadData {
    /// 群号
    pub group_id: i64,
    /// 发送者 QQ 号
    pub user_id: i64,
    /// 文件 ID
    pub file_id: String,
    /// 文件名称
    pub file_name: String,
    /// 文件大小
    pub file_size: i64,
}
