//! 定义了从通信平台接收到的事件结构

use crate::types::{common::MessageScene, message::in_coming::IncomingMessage};
use serde::{Deserialize, Serialize};

/// 代表从平台接收到的通用事件
///
/// 每个事件都有一个时间戳、接收该事件的机器人实例的ID，
/// 以及一个详细说明事件性质的特定 [`EventKind`]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    /// 事件发生的Unix时间戳（秒）
    pub time: i64,
    /// 机器人自身的 QQ 号
    pub self_id: i64,
    /// 事件的具体种类及其关联数据
    #[serde(flatten)]
    pub kind: EventKind,
}

/// 枚举可以接收到的不同类型的事件
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "event_type", content = "data")]
pub enum EventKind {
    /// 机器人离线事件
    #[serde(rename = "bot_offline")]
    BotOffline(BotOfflineData),

    /// 当接收到消息时触发的事件
    #[serde(rename = "message_receive")]
    MessageReceive(IncomingMessage),

    /// 当消息被撤回时触发的事件
    #[serde(rename = "message_recall")]
    MessageRecall(MessageRecallData),

    /// 当接收到好友请求时触发的事件
    #[serde(rename = "friend_request")]
    FriendRequest(FriendRequestData),

    /// 当用户请求加入群组时触发的事件
    #[serde(rename = "group_join_request")]
    GroupJoinRequest(GroupJoinRequestData),

    /// 当用户被群成员邀请加入群组时触发的事件
    /// 通常需要管理员批准
    #[serde(rename = "group_invited_join_request")]
    GroupInvitedJoinRequest(GroupInvitedJoinRequestData),

    /// 当机器人被邀请加入群组时触发的事件
    #[serde(rename = "group_invitation_request")]
    GroupInvitationRequest(GroupInvitationData),

    /// 当好友发送“戳一戳”互动时触发的事件
    #[serde(rename = "friend_nudge")]
    FriendNudge(FriendNudgeData),

    /// 当在好友聊天中上传文件时触发的事件
    #[serde(rename = "friend_file_upload")]
    FriendFileUpload(FriendFileUploadData),

    /// 当群管理员状态变更（被提升或降级）时触发的事件
    #[serde(rename = "group_admin_change")]
    GroupAdminChange(GroupAdminChangeData),

    /// 当群消息被标记或取消标记为“精华”消息时触发的事件
    #[serde(rename = "group_essence_message_change")]
    GroupEssenceMessageChange(GroupEssenceMessageChangeData),

    /// 当新成员加入群组时触发的事件
    #[serde(rename = "group_member_increase")]
    GroupMemberIncrease(GroupMemberIncreaseData),

    /// 当成员离开或被移出群组时触发的事件
    #[serde(rename = "group_member_decrease")]
    GroupMemberDecrease(GroupMemberDecreaseData),

    /// 当群组名称更改时触发的事件
    #[serde(rename = "group_name_change")]
    GroupNameChange(GroupNameChangeData),

    /// 当群消息的表态（reaction）被添加或移除时触发的事件
    #[serde(rename = "group_message_reaction")]
    GroupMessageReaction(GroupMessageReactionData),

    /// 当群成员被禁言或解除禁言时触发的事件
    #[serde(rename = "group_mute")]
    GroupMute(GroupMuteData),

    /// 当整个群组被禁言或解除禁言时触发的事件
    #[serde(rename = "group_whole_mute")]
    GroupWholeMute(GroupWholeMuteData),

    /// 当在群组中发生“戳一戳”互动时触发的事件
    #[serde(rename = "group_nudge")]
    GroupNudge(GroupNudgeData),

    /// 当在群聊中上传文件时触发的事件
    #[serde(rename = "group_file_upload")]
    GroupFileUpload(GroupFileUploadData),
}

/// 与 `BotOffline` 事件关联的数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotOfflineData {
    /// 下线原因
    pub reason: String,
}

/// 与 `MessageRecall` 事件关联的数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageRecallData {
    /// 消息被撤回的场景（例如："friend", "group", "temp"）
    pub message_scene: MessageScene,
    /// 消息被撤回的好友QQ号或群号
    pub peer_id: i64,
    /// 被撤回消息的序列号
    pub message_seq: i64,
    /// 被撤回消息的发送者QQ号
    pub sender_id: i64,
    /// 执行撤回操作的用户QQ号
    pub operator_id: i64,
    /// 撤回提示的后缀文本
    pub display_suffix: String,
}

/// 与 `FriendRequest` 事件关联的数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriendRequestData {
    /// 申请好友的用户 QQ 号
    pub initiator_id: String,
    /// 用户 UID
    pub initiator_uid: i64,
    /// 申请附加信息
    pub comment: String,
    /// 申请来源
    pub via: String,
}

/// 与 `GroupJoinRequest` 事件关联的数据
/// 这是指用户主动申请加入群组的情况
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupJoinRequestData {
    /// 群号
    pub group_id: i64,
    /// 请求对应的通知序列号
    pub notification_seq: i64,
    /// 请求是否被过滤（发起自风险账户）
    pub is_filtered: bool,
    /// 申请入群的用户 QQ 号
    pub initiator_id: i64,
    /// 申请附加信息
    pub comment: String,
}

/// 与 `GroupInvitedJoinRequest` 事件关联的数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupInvitedJoinRequestData {
    /// 群号
    pub group_id: i64,
    /// 请求对应的通知序列号
    pub notification_seq: i64,
    /// 邀请者 QQ 号
    pub initiator_id: i64,
    /// 被邀请者 QQ 号
    pub target_user_id: i64,
}

/// 与 `GroupInvitationRequest` 事件关联的数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupInvitationData {
    /// 群号
    pub group_id: i64,
    /// 邀请序列号
    pub invitation_seq: i64,
    /// 邀请者 QQ 号
    pub initiator_id: i64,
}

/// 与 `FriendNudge`（好友戳一戳）事件关联的数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriendNudgeData {
    /// 好友 QQ 号
    pub user_id: i64,
    /// 是否是自己发送的戳一戳
    pub is_self_send: bool,
    /// 是否是自己接收的戳一戳
    pub is_self_receive: bool,
    /// 戳一戳提示的动作文本
    pub display_action: String,
    /// 戳一戳提示的后缀文本
    pub display_suffix: String,
    /// 戳一戳提示的动作图片 URL，用于取代动作提示文本
    pub display_action_img_url: String,
}

/// 与 `FriendFileUpload`（好友文件上传）事件关联的数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriendFileUploadData {
    /// 上传文件或从机器人接收文件的好友的QQ号
    pub user_id: i64,
    /// 上传文件的唯一ID
    pub file_id: String,
    /// 上传文件的名称
    pub file_name: String,
    /// 上传文件的大小（字节）
    pub file_size: i64,
    /// 文件的 TriSHA1 哈希值
    pub file_hash: String,
    /// 如果是机器人上传的文件，则为true；如果是好友上传的，则为false
    pub is_self: bool,
}

/// 与 `GroupAdminChange`（群管理员变更）事件关联的数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupAdminChangeData {
    /// 管理员状态发生变更的群组ID
    pub group_id: i64,
    /// 管理员状态发生变更的用户的QQ号
    pub user_id: i64,
    /// 如果用户被设置为管理员，则为true；如果其管理员状态被撤销，则为false
    pub is_set: bool,
}

/// 与 `GroupEssenceMessageChange`（群精华消息变更）事件关联的数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupEssenceMessageChangeData {
    /// 精华消息状态发生变更的群组ID
    pub group_id: i64,
    /// 精华消息状态发生变更的消息序列号
    pub message_seq: i64,
    /// 如果消息被设置为精华消息，则为true；如果被取消设置，则为false
    pub is_set: bool,
}

/// 与 `GroupMemberIncrease`（群成员增加）事件关联的数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMemberIncreaseData {
    /// 成员加入的群组ID
    pub group_id: i64,
    /// 加入群组的用户的QQ号
    pub user_id: i64,
    /// 如果适用（例如，用户申请并且管理员批准），则为批准加入的管理员的QQ号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<i64>,
    /// 如果适用（例如，用户被邀请并加入），则为邀请该用户的成员的QQ号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitor_id: Option<i64>,
}

/// 与 `GroupMemberDecrease`（群成员减少）事件关联的数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMemberDecreaseData {
    /// 成员离开或被移除的群组ID
    pub group_id: i64,
    /// 离开或被移除的用户的QQ号
    pub user_id: i64,
    /// 如果适用（例如，成员被踢出），则为移除该成员的管理员的QQ号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<i64>,
}

/// 与 `GroupNameChange`（群名称变更）事件关联的数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupNameChangeData {
    /// 名称被更改的群组ID
    pub group_id: i64,
    /// 群组的新名称
    pub group_new_name: String,
    /// 更改群名称的用户的QQ号
    pub operator_id: i64,
}

/// 与 `GroupMessageReaction`（群消息表态）事件关联的数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMessageReactionData {
    /// 发生消息表态的群组ID
    pub group_id: i64,
    /// 添加或移除表态的用户的QQ号
    pub user_id: i64,
    /// 被表态的消息的序列号
    pub message_seq: i64,
    /// 表态表情/face的ID
    pub face_id: String,
    /// 是否为添加，`false` 表示取消回应
    pub is_add: bool,
}

/// 与 `GroupMute`（群禁言特定成员）事件关联的数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMuteData {
    /// 成员被禁言/解除禁言的群组ID
    pub group_id: i64,
    /// 被禁言/解除禁言的用户的QQ号
    pub user_id: i64,
    /// 操作者QQ号
    pub operator_id: i64,
    /// 禁言时长（秒）值为0通常表示用户被解除禁言。
    pub duration: i32,
}

/// 与 `GroupWholeMute`（全群禁言）事件关联的数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupWholeMuteData {
    /// 被禁言/解除禁言的群组ID
    pub group_id: i64,
    /// 执行全群禁言/解除禁言操作的管理员的QQ号
    pub operator_id: i64,
    /// 如果群组被禁言，则为true；如果被解除禁言，则为false
    pub is_mute: bool,
}

/// 与 `GroupNudge`（群戳一戳）事件关联的数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupNudgeData {
    /// 发生戳一戳的群组ID
    pub group_id: i64,
    /// 发送戳一戳的用户的QQ号
    pub sender_id: i64,
    /// 接收戳一戳的用户的QQ号
    pub receiver_id: i64,
    /// 戳一戳提示的动作文本
    pub display_action: String,
    /// 戳一戳提示的后缀文本
    pub display_suffix: String,
    /// 戳一戳提示的动作图片 URL，用于取代动作提示文本
    pub display_action_img_url: String,
}

/// 与 `GroupFileUpload`（群文件上传）事件关联的数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupFileUploadData {
    /// 文件上传的群组ID
    pub group_id: i64,
    /// 上传文件的用户的QQ号
    pub user_id: i64,
    /// 上传文件的唯一ID
    pub file_id: String,
    /// 上传文件的名称
    pub file_name: String,
    /// 上传文件的大小（字节）
    pub file_size: i64,
}
