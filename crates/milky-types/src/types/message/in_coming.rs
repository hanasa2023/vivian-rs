//! 定义了接收到的各类消息（如私聊、群聊、临时会话消息）及其组成部分（消息段）的数据结构

use serde::{Deserialize, Serialize};

use crate::types::{
    common::MessageScene,
    friend::Friend,
    group::{Group, GroupMember},
};

/// 代表一个通用的接收消息结构
///
/// 这是许多具体消息类型（如 [`FriendMessage`], [`GroupMessage`]）的基础，
/// 包含了消息的共同属性
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct IncomingMessage {
    /// 消息的接收方ID，可以是好友QQ号或群号
    pub peer_id: i64,
    /// 消息的序列号，用于唯一标识一条消息
    pub message_seq: i64,
    /// 消息发送者的QQ号
    pub sender_id: i64,
    /// 消息发送的Unix时间戳（单位：秒）
    pub time: i64,
    /// 组成消息内容的实际数据段列表
    pub segments: Vec<IncomingSegment>,
    /// 消息场景的类型标识符，例如 "friend", "group", "temp" 等
    pub message_scene: MessageScene,
}

/// 代表接收到的好友消息
///
/// 继承自 [`IncomingMessage`] 并额外包含了好友的详细信息
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FriendMessage {
    #[serde(flatten)]
    pub message: IncomingMessage,
    /// 发送此消息的好友的详细信息
    pub friend: Friend,
}

/// 代表接收到的群消息
///
/// 继承自 [`IncomingMessage`] 并额外包含了群及发送成员的详细信息
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GroupMessage {
    #[serde(flatten)]
    pub message: IncomingMessage,
    /// 消息所属群组的详细信息
    pub group: Group,
    /// 发送此消息的群成员的详细信息
    pub group_member: GroupMember,
}

/// 代表接收到的临时会话消息
///
/// 继承自 [`IncomingMessage`] 并可能包含临时会话来源群组的信息
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TempMessage {
    #[serde(flatten)]
    pub message: IncomingMessage,
    /// 如果临时会话是通过某个群发起的，则此字段包含该群的详细信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

/// 代表接收到的合并转发消息中的单条消息内容
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct IncomingForwardMessage {
    /// 发送者名称
    pub sender_name: String,
    /// 发送者头像URL
    pub avatar_url: String,
    /// 消息 Unix 时间戳（秒）
    pub time: i64,
    /// 组成该条转发消息内容的实际数据段列表
    pub segments: Vec<IncomingSegment>,
}

/// 枚举构成接收消息内容的各种可能的消息段类型
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type", content = "data")]
pub enum IncomingSegment {
    /// 文本消息段
    Text {
        /// 实际的文本内容
        text: String,
    },

    /// 提及（@）某人的消息段
    Mention {
        /// 被提及用户的QQ号
        user_id: i64,
    },

    /// 提及（@）全体成员的消息段
    MentionAll {},

    /// QQ表情消息段
    Face {
        /// QQ表情的内置ID
        face_id: String,
    },

    /// 回复消息段，用于引用之前的某条消息
    Reply {
        /// 被回复（引用）的消息的序列号
        message_seq: i64,
    },

    /// 图片消息段
    Image {
        /// 图片的资源ID，可用于后续操作（如获取图片URL）
        resource_id: String,
        /// 临时URL
        temp_url: String,
        /// 图片宽度
        width: i32,
        /// 图片高度
        height: i32,
        /// 图片的预览文本
        summary: String,
        /// 图片的子类型，例如 "normal" (普通图片), "sticker" (贴图表情) 等
        sub_type: String,
    },

    /// 语音消息段
    Record {
        /// 语音的资源ID，可用于后续操作
        resource_id: String,
        /// 临时URL
        temp_url: String,
        /// 语音的时长（单位：秒）
        duration: i32,
    },

    /// 视频消息段
    Video {
        /// 视频的资源ID，可用于后续操作
        resource_id: String,
        /// 临时URL
        temp_url: String,
        /// 视频宽度
        width: i32,
        /// 视频高度
        height: i32,
        /// 视频时长（单位：秒）
        duration: i32,
    },

    /// 文件消息段
    File {
        /// 文件 ID
        file_id: String,
        /// 文件名称
        file_name: String,
        /// 文件大小（字节）
        file_size: i64,
        /// 文件的 TriSHA1 哈希值，仅在私聊文件中存在
        #[serde(skip_serializing_if = "Option::is_none")]
        file_hash: Option<String>,
    },

    /// 合并转发消息段
    Forward {
        /// 合并转发消息的ID，可用于获取转发消息的具体内容
        forward_id: String,
    },

    /// 商城表情（大表情）消息段
    MarketFace {
        /// 商城表情的图片URL
        url: String,
    },

    /// 轻应用（小程序、小游戏卡片等）消息段
    LightApp {
        /// 小程序的名称
        app_name: String,
        /// 小程序的JSON数据负载，具体结构由应用本身定义
        json_payload: String,
    },

    /// XML 卡片消息段
    XML {
        /// XML消息的服务ID
        service_id: i32,
        /// XML数据的字符串负载
        xml_payload: String,
    },
}

#[cfg(test)]
mod test {
    use serde_test::{Token, assert_tokens};

    use super::*;

    #[test]
    fn test_serialize_mention_all() {
        let segment = IncomingSegment::MentionAll {};
        println!("{:?}", serde_json::to_string_pretty(&segment).unwrap());

        assert_tokens(
            &segment,
            &[
                Token::Struct {
                    name: "IncomingSegment",
                    len: 2,
                },
                Token::Str("type"),
                Token::UnitVariant {
                    name: "IncomingSegment",
                    variant: "mention_all",
                },
                Token::Str("data"),
                Token::Struct {
                    name: "mention_all",
                    len: 0,
                },
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
