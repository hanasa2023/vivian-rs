use serde::{Deserialize, Serialize};

use crate::{Friend, Group, GroupMember};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct IncomingMessage {
    /// 好友 QQ 号或群号
    pub peer_id: i64,
    /// 消息序列号
    pub message_seq: i64,
    /// 发送者 QQ 号
    pub sender_id: i64,
    /// 消息 Unix 时间戳（秒）
    pub time: i64,
    /// 消息段列表
    pub segments: Vec<IncomingSegment>,
    /// 类型标识符
    pub mesage_scene: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FriendMessage {
    /// 好友 QQ 号或群号
    pub peer_id: i64,
    /// 消息序列号
    pub message_seq: i64,
    /// 发送者 QQ 号
    pub sender_id: i64,
    /// 消息 Unix 时间戳（秒）
    pub time: i64,
    /// 消息段列表
    pub segments: Vec<IncomingSegment>,
    /// 类型标识符
    pub mesage_scene: String,
    /// 好友信息
    pub friend: Friend,
    /// 客户端序列号
    pub client_seq: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GroupMessage {
    /// 好友 QQ 号或群号
    pub peer_id: i64,
    /// 消息序列号
    pub message_seq: i64,
    /// 发送者 QQ 号
    pub sender_id: i64,
    /// 消息 Unix 时间戳（秒）
    pub time: i64,
    /// 消息段列表
    pub segments: Vec<IncomingSegment>,
    /// 类型标识符
    pub mesage_scene: String,
    /// 群信息
    pub group: Group,
    /// 群成员信息
    pub group_member: GroupMember,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TempMessage {
    /// 好友 QQ 号或群号
    pub peer_id: i64,
    /// 消息序列号
    pub message_seq: i64,
    /// 发送者 QQ 号
    pub sender_id: i64,
    /// 消息 Unix 时间戳（秒）
    pub time: i64,
    /// 消息段列表
    pub segments: Vec<IncomingSegment>,
    /// 类型标识符
    pub mesage_scene: String,
    /// 临时会话发送者的所在的群信息（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

// TODO: 纠正结构体名
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct IncomingForwardMessage {
    /// 发送者 QQ 号
    pub user_id: i64,
    /// 发送者名称
    pub name: String,
    /// 消息段列表
    pub segments: Vec<IncomingSegment>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
pub enum IncomingSegment {
    #[serde(rename = "text")]
    Text(TextData),

    #[serde(rename = "mention")]
    Mention(MentionData),

    #[serde(rename = "mention_all")]
    MentionAll(MentionAllData),

    #[serde(rename = "face")]
    Face(FaceData),

    #[serde(rename = "reply")]
    Reply(ReplyData),

    #[serde(rename = "image")]
    Image(ImageData),

    #[serde(rename = "record")]
    Record(RecordData),

    #[serde(rename = "video")]
    Video(VideoData),

    #[serde(rename = "forward")]
    Forward(ForwardData),

    #[serde(rename = "market_face")]
    MarketFace(MarketFaceData),

    #[serde(rename = "light_app")]
    LightApp(LightAppData),

    #[serde(rename = "xml")]
    XML(XMLData),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TextData {
    /// 文本内容
    text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MentionData {
    /// 提及的 QQ 号
    user_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MentionAllData;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FaceData {
    /// 表情ID
    face_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReplyData {
    /// 被引用的消息序列号
    pub message_seq: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ImageData {
    /// 资源 ID
    pub resource_id: String,
    /// 图片预览文本（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// 图片类型（可能值：`normal`, `sticker`）
    pub sub_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RecordData {
    /// 资源 ID
    pub resource_id: String,
    /// 语音时长（秒）
    pub duration: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct VideoData {
    /// 资源 ID
    resource_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ForwardData {
    /// 合并转发 ID
    pub forward_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MarketFaceData {
    /// 市场表情 URL
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LightAppData {
    /// 小程序名称
    pub app_name: String,
    /// 小程序 JSON 数据
    pub json_payload: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct XMLData {
    /// 服务 ID
    pub service_id: i32,
    /// XML 数据
    pub xml_payload: String,
}
