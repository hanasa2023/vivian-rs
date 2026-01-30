//! 定义了从通信平台接收到的事件结构

use crate::types::{
    common::MessageScene,
    message::in_coming::{FriendMessage, GroupMessage, IncomingMessage, TempMessage},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
        message: MessageEvent,
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
    GroupInvitation {
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
        /// 发生变更的用户QQ号
        user_id: i64,
        /// 操作者 QQ 号
        operator_id: i64,
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

/// 消息事件的包装类型，根据 message_scene 字段自动反序列化为具体的消息类型
///
/// 使用自定义序列化/反序列化逻辑根据 message_scene 字段选择具体的消息结构，
/// 保留额外的元信息
#[derive(Debug, Clone, PartialEq)]
pub enum MessageEvent {
    /// 好友消息，包含好友的详细信息
    Friend(FriendMessage),
    /// 群消息，包含群和群成员的详细信息
    Group(GroupMessage),
    /// 临时会话消息，可能包含来源群组信息
    Temp(TempMessage),
}

impl MessageEvent {
    /// 获取消息场景
    pub fn message_scene(&self) -> MessageScene {
        match self {
            MessageEvent::Friend(msg) => msg.message.message_scene,
            MessageEvent::Group(msg) => msg.message.message_scene,
            MessageEvent::Temp(msg) => msg.message.message_scene,
        }
    }

    /// 获取基础消息信息
    pub fn base_message(&self) -> &IncomingMessage {
        match self {
            MessageEvent::Friend(msg) => &msg.message,
            MessageEvent::Group(msg) => &msg.message,
            MessageEvent::Temp(msg) => &msg.message,
        }
    }
}

impl Serialize for MessageEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MessageEvent::Friend(msg) => msg.serialize(serializer),
            MessageEvent::Group(msg) => msg.serialize(serializer),
            MessageEvent::Temp(msg) => msg.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for MessageEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let value = serde_json::Value::deserialize(deserializer)?;

        let message_scene = value
            .get("message_scene")
            .and_then(|v| v.as_str())
            .unwrap_or_default();

        match message_scene {
            "friend" => {
                let msg: FriendMessage = serde_json::from_value(value)
                    .map_err(|_| D::Error::custom("无法反序列化为好友消息"))?;
                Ok(MessageEvent::Friend(msg))
            }
            "group" => {
                let msg: GroupMessage = serde_json::from_value(value)
                    .map_err(|_| D::Error::custom("无法反序列化为群消息"))?;
                Ok(MessageEvent::Group(msg))
            }
            "temp" => {
                let msg: TempMessage = serde_json::from_value(value)
                    .map_err(|_| D::Error::custom("无法反序列化为临时消息"))?;
                Ok(MessageEvent::Temp(msg))
            }
            scene => Err(D::Error::custom(format!("未知的消息场景: {}", scene))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        common::Sex,
        friend::Friend,
        group::{Group, GroupMember, GroupRole},
    };

    #[test]
    fn test_serialize_and_deserialize_friend_message() {
        let event = Event {
            self_id: 1234567890,
            time: 1630483200,
            kind: EventKind::MessageReceive {
                message: MessageEvent::Friend(FriendMessage {
                    message: IncomingMessage {
                        peer_id: 987654321,
                        message_seq: 12345,
                        sender_id: 987654321,
                        time: 1630483200,
                        segments: vec![],
                        message_scene: MessageScene::Friend,
                    },
                    friend: Friend {
                        user_id: 987654321,
                        nickname: "测试好友".to_string(),
                        sex: Sex::Male,
                        qid: "".to_string(),
                        remark: "".to_string(),
                        category: None,
                    },
                }),
            },
        };

        // 验证序列化
        let json = serde_json::to_string_pretty(&event).unwrap();
        println!("好友消息 JSON:\n{}", json);

        // 验证反序列化
        let deserialized: Event = serde_json::from_str(&json).unwrap();
        match &deserialized.kind {
            EventKind::MessageReceive {
                message: MessageEvent::Friend(msg),
            } => {
                assert_eq!(msg.message.message_scene, MessageScene::Friend);
                assert_eq!(msg.friend.nickname, "测试好友");
            }
            _ => panic!("反序列化结果应该是好友消息"),
        }
    }

    #[test]
    fn test_serialize_and_deserialize_group_message() {
        let event = Event {
            self_id: 1234567890,
            time: 1630483200,
            kind: EventKind::MessageReceive {
                message: MessageEvent::Group(GroupMessage {
                    message: IncomingMessage {
                        peer_id: 123456,
                        message_seq: 12345,
                        sender_id: 987654321,
                        time: 1630483200,
                        segments: vec![],
                        message_scene: MessageScene::Group,
                    },
                    group: Group {
                        group_id: 123456,
                        group_name: "测试群".to_string(),
                        member_count: 100,
                        max_member_count: 500,
                    },
                    group_member: GroupMember {
                        user_id: 987654321,
                        nickname: "测试成员".to_string(),
                        sex: Sex::Female,
                        group_id: 123456,
                        card: "".to_string(),
                        title: "".to_string(),
                        level: 1,
                        role: GroupRole::Member,
                        join_time: 1630000000,
                        last_sent_time: 1630483200,
                        shut_up_end_time: None,
                    },
                }),
            },
        };

        // 验证序列化
        let json = serde_json::to_string_pretty(&event).unwrap();
        println!("群消息 JSON:\n{}", json);

        // 验证反序列化
        let deserialized: Event = serde_json::from_str(&json).unwrap();
        match &deserialized.kind {
            EventKind::MessageReceive {
                message: MessageEvent::Group(msg),
            } => {
                assert_eq!(msg.message.message_scene, MessageScene::Group);
                assert_eq!(msg.group.group_name, "测试群");
                assert_eq!(msg.group_member.nickname, "测试成员");
            }
            _ => panic!("反序列化结果应该是群消息"),
        }
    }

    #[test]
    fn test_deserialize_message_with_meta() {
        // 测试带有完整元信息的群消息 JSON
        let json = r#"{
            "time": 1630483200,
            "self_id": 1234567890,
            "event_type": "message_receive",
            "data": {
                "peer_id": 123456,
                "message_seq": 12345,
                "sender_id": 987654321,
                "time": 1630483200,
                "segments": [],
                "message_scene": "group",
                "group": {
                    "group_id": 123456,
                    "group_name": "测试群",
                    "member_count": 100,
                    "max_member_count": 500
                },
                "group_member": {
                    "user_id": 987654321,
                    "nickname": "测试成员",
                    "sex": "female",
                    "group_id": 123456,
                    "card": "",
                    "title": "",
                    "level": 1,
                    "role": "member",
                    "join_time": 1630000000,
                    "last_sent_time": 1630483200
                }
            }
        }"#;

        let event: Event = serde_json::from_str(json).unwrap();
        match &event.kind {
            EventKind::MessageReceive {
                message: MessageEvent::Group(msg),
            } => {
                assert_eq!(msg.message.message_scene, MessageScene::Group);
                assert_eq!(msg.group.group_name, "测试群");
                assert_eq!(msg.group_member.nickname, "测试成员");
            }
            _ => panic!("反序列化结果应该是群消息"),
        }
    }

    #[test]
    fn test_deserialize_temp_message() {
        // 测试临时会话消息 JSON
        let json = r#"{
            "time": 1630483200,
            "self_id": 1234567890,
            "event_type": "message_receive",
            "data": {
                "peer_id": 987654321,
                "message_seq": 12345,
                "sender_id": 987654321,
                "time": 1630483200,
                "segments": [],
                "message_scene": "temp",
                "group": {
                    "group_id": 123456,
                    "group_name": "来源群",
                    "member_count": 50,
                    "max_member_count": 200
                }
            }
        }"#;

        let event: Event = serde_json::from_str(json).unwrap();
        match &event.kind {
            EventKind::MessageReceive {
                message: MessageEvent::Temp(msg),
            } => {
                assert_eq!(msg.message.message_scene, MessageScene::Temp);
                assert!(msg.group.is_some());
                assert_eq!(msg.group.as_ref().unwrap().group_name, "来源群");
            }
            _ => panic!("反序列化结果应该是临时消息"),
        }
    }

    #[test]
    fn test_message_event_helper_methods() {
        let friend_msg = MessageEvent::Friend(FriendMessage {
            message: IncomingMessage {
                peer_id: 123,
                message_seq: 1,
                sender_id: 123,
                time: 0,
                segments: vec![],
                message_scene: MessageScene::Friend,
            },
            friend: Friend::default(),
        });

        assert_eq!(friend_msg.message_scene(), MessageScene::Friend);
        assert_eq!(friend_msg.base_message().peer_id, 123);

        let group_msg = MessageEvent::Group(GroupMessage {
            message: IncomingMessage {
                peer_id: 456,
                message_seq: 2,
                sender_id: 789,
                time: 0,
                segments: vec![],
                message_scene: MessageScene::Group,
            },
            group: Group::default(),
            group_member: GroupMember::default(),
        });

        assert_eq!(group_msg.message_scene(), MessageScene::Group);
        assert_eq!(group_msg.base_message().sender_id, 789);
    }
}
