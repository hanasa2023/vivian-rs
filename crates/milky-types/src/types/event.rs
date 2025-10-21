//! 定义了从通信平台接收到的事件结构

use crate::types::{common::MessageScene, message::in_coming::IncomingMessage};
use serde::{Deserialize, Serialize};

/// 代表从平台接收到的通用事件
///
/// 每个事件都有一个时间戳、接收该事件的机器人实例的ID，
/// 以及一个详细说明事件性质的特定 [`EventKind`]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case", tag = "event_type", content = "data")]
pub enum EventKind {
    /// 机器人离线事件
    BotOffline {
        /// 下线原因
        reason: String,
    },

    /// 当接收到消息时触发的事件
    MessageReceive {
        #[serde(flatten)]
        message: IncomingMessage,
    },

    /// 当消息被撤回时触发的事件
    MessageRecall {
        /// 消息被撤回的场景（例如："friend", "group", "temp"）
        message_scene: MessageScene,
        /// 消息被撤回的好友QQ号或群号
        peer_id: i64,
        /// 被撤回消息的序列号
        message_seq: i64,
        /// 被撤回消息的发送者QQ号
        sender_id: i64,
        /// 执行撤回操作的用户QQ号
        operator_id: i64,
        /// 撤回提示的后缀文本
        display_suffix: String,
    },

    /// 当接收到好友请求时触发的事件
    FriendRequest {
        /// 申请好友的用户 QQ 号
        initiator_id: String,
        /// 用户 UID
        initiator_uid: i64,
        /// 申请附加信息
        comment: String,
        /// 申请来源
        via: String,
    },

    /// 当用户请求加入群组时触发的事件
    GroupJoinRequest {
        /// 群号
        group_id: i64,
        /// 请求对应的通知序列号
        notification_seq: i64,
        /// 请求是否被过滤（发起自风险账户）
        is_filtered: bool,
        /// 申请入群的用户 QQ 号
        initiator_id: i64,
        /// 申请附加信息
        comment: String,
    },

    /// 当用户被群成员邀请加入群组时触发的事件
    GroupInvitedJoinRequest {
        /// 群号
        group_id: i64,
        /// 请求对应的通知序列号
        notification_seq: i64,
        /// 邀请者 QQ 号
        initiator_id: i64,
        /// 被邀请者 QQ 号
        target_user_id: i64,
    },

    /// 当机器人被邀请加入群组时触发的事件
    GroupInvitationRequest {
        /// 群号
        group_id: i64,
        /// 邀请序列号
        invitation_seq: i64,
        /// 邀请者 QQ 号
        initiator_id: i64,
    },

    /// 当好友发送“戳一戳”互动时触发的事件
    FriendNudge {
        /// 好友 QQ 号
        user_id: i64,
        /// 是否是自己发送的戳一戳
        is_self_send: bool,
        /// 是否是自己接收的戳一戳
        is_self_receive: bool,
        /// 戳一戳提示的动作文本
        display_action: String,
        /// 戳一戳提示的后缀文本
        display_suffix: String,
        /// 戳一戳提示的动作图片 URL，用于取代动作提示文本
        display_action_img_url: String,
    },

    /// 当在好友聊天中上传文件时触发的事件
    FriendFileUpload {
        /// 上传文件或从机器人接收文件的好友的QQ号
        user_id: i64,
        /// 上传文件的唯一ID
        file_id: String,
        /// 上传文件的名称
        file_name: String,
        /// 上传文件的大小（字节）
        file_size: i64,
        /// 文件的 TriSHA1 哈希值
        file_hash: String,
        /// 如果是机器人上传的文件，则为true；如果是好友上传的，则为false
        is_self: bool,
    },

    /// 当群管理员状态变更（被提升或降级）时触发的事件
    GroupAdminChange {
        /// 管理员状态发生变更的群组ID
        group_id: i64,
        /// 管理员状态发生变更的用户的QQ号
        user_id: i64,
        /// 如果用户被设置为管理员，则为true；如果其管理员状态被撤销，则为false
        is_set: bool,
    },

    /// 当群消息被标记或取消标记为“精华”消息时触发的事件
    GroupEssenceMessageChange {
        /// 精华消息状态发生变更的群组ID
        group_id: i64,
        /// 精华消息状态发生变更的消息序列号
        message_seq: i64,
        /// 如果消息被设置为精华消息，则为true；如果被取消设置，则为false
        is_set: bool,
    },

    /// 当新成员加入群组时触发的事件
    GroupMemberIncrease {
        /// 成员加入的群组ID
        group_id: i64,
        /// 加入群组的用户的QQ号
        user_id: i64,
        /// 如果适用（例如，用户申请并且管理员批准），则为批准加入的管理员的QQ号
        #[serde(skip_serializing_if = "Option::is_none")]
        operator_id: Option<i64>,
        /// 如果适用（例如，用户被邀请并加入），则为邀请该用户的成员的QQ号
        #[serde(skip_serializing_if = "Option::is_none")]
        invitor_id: Option<i64>,
    },

    /// 当成员离开或被移出群组时触发的事件
    GroupMemberDecrease {
        /// 成员离开或被移除的群组ID
        group_id: i64,
        /// 离开或被移除的用户的QQ号
        user_id: i64,
        /// 如果适用（例如，成员被踢出），则为移除该成员的管理员的QQ号
        #[serde(skip_serializing_if = "Option::is_none")]
        operator_id: Option<i64>,
    },

    /// 当群组名称更改时触发的事件
    GroupNameChange {
        /// 名称被更改的群组ID
        group_id: i64,
        /// 群组的新名称
        group_new_name: String,
        /// 更改群名称的用户的QQ号
        operator_id: i64,
    },

    /// 当群消息的表态（reaction）被添加或移除时触发的事件
    GroupMessageReaction {
        /// 发生消息表态的群组ID
        group_id: i64,
        /// 添加或移除表态的用户的QQ号
        user_id: i64,
        /// 被表态的消息的序列号
        message_seq: i64,
        /// 表态表情/face的ID
        face_id: String,
        /// 是否为添加，`false` 表示取消回应
        is_add: bool,
    },

    /// 当群成员被禁言或解除禁言时触发的事件
    GroupMute {
        /// 成员被禁言/解除禁言的群组ID
        group_id: i64,
        /// 被禁言/解除禁言的用户的QQ号
        user_id: i64,
        /// 操作者QQ号
        operator_id: i64,
        /// 禁言时长（秒）值为0通常表示用户被解除禁言。
        duration: i32,
    },

    /// 当整个群组被禁言或解除禁言时触发的事件
    GroupWholeMute {
        /// 被禁言/解除禁言的群组ID
        group_id: i64,
        /// 执行全群禁言/解除禁言操作的管理员的QQ号
        operator_id: i64,
        /// 如果群组被禁言，则为true；如果被解除禁言，则为false
        is_mute: bool,
    },

    /// 当在群组中发生“戳一戳”互动时触发的事件
    GroupNudge {
        /// 发生戳一戳的群组ID
        group_id: i64,
        /// 发送戳一戳的用户的QQ号
        sender_id: i64,
        /// 接收戳一戳的用户的QQ号
        receiver_id: i64,
        /// 戳一戳提示的动作文本
        display_action: String,
        /// 戳一戳提示的后缀文本
        display_suffix: String,
        /// 戳一戳提示的动作图片 URL，用于取代动作提示文本
        display_action_img_url: String,
    },

    /// 当在群聊中上传文件时触发的事件
    GroupFileUpload {
        /// 文件上传的群组ID
        group_id: i64,
        /// 上传文件的用户的QQ号
        user_id: i64,
        /// 上传文件的唯一ID
        file_id: String,
        /// 上传文件的名称
        file_name: String,
        /// 上传文件的大小（字节）
        file_size: i64,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{Token, assert_tokens};

    #[test]
    fn test_serialize_message_receive_event() {
        let event = Event {
            self_id: 1234567890,
            time: 1630483200,
            kind: EventKind::MessageReceive {
                message: IncomingMessage {
                    peer_id: 987654321,
                    message_seq: 12345,
                    sender_id: 987654321,
                    time: 1630483200,
                    segments: vec![],
                    message_scene: MessageScene::Friend,
                },
            },
        };

        assert_tokens(
            &event,
            &[
                Token::Map { len: None },
                Token::Str("time"),
                Token::I64(1630483200),
                Token::Str("self_id"),
                Token::I64(1234567890),
                Token::Str("event_type"),
                Token::UnitVariant {
                    name: "EventKind",
                    variant: "message_receive",
                },
                Token::Str("data"),
                Token::Map { len: None },
                Token::Str("peer_id"),
                Token::I64(987654321),
                Token::Str("message_seq"),
                Token::I64(12345),
                Token::Str("sender_id"),
                Token::I64(987654321),
                Token::Str("time"),
                Token::I64(1630483200),
                Token::Str("segments"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("message_scene"),
                Token::UnitVariant {
                    name: "MessageScene",
                    variant: "friend",
                },
                Token::MapEnd,
                Token::MapEnd,
            ],
        );
    }
}
