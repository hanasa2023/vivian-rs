//! 定义了接收到的各类消息（如私聊、群聊、临时会话消息）及其组成部分（消息段）的数据结构。

use serde::{Deserialize, Serialize};

use crate::{Friend, Group, GroupMember};

/// 代表一个通用的接收消息结构。
///
/// 这是许多具体消息类型（如 [`FriendMessage`], [`GroupMessage`]）的基础，
/// 包含了消息的共同属性。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct IncomingMessage {
    /// 消息的接收方ID，可以是好友QQ号或群号。
    pub peer_id: i64,
    /// 消息的序列号，用于唯一标识一条消息。
    pub message_seq: i64,
    /// 消息发送者的QQ号。
    pub sender_id: i64,
    /// 消息发送的Unix时间戳（单位：秒）。
    pub time: i64,
    /// 组成消息内容的实际数据段列表。
    pub segments: Vec<IncomingSegment>,
    /// 消息场景的类型标识符，例如 "friend", "group", "temp" 等。
    pub message_scene: String,
}

/// 代表接收到的好友消息。
///
/// 继承自 [`IncomingMessage`] 并额外包含了好友的详细信息。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FriendMessage {
    /// 好友的QQ号（即消息的接收方，通常与 `sender_id` 对应，除非是自己给自己发消息）。
    pub peer_id: i64,
    /// 消息的序列号。
    pub message_seq: i64,
    /// 消息发送者的QQ号。
    pub sender_id: i64,
    /// 消息发送的Unix时间戳（秒）。
    pub time: i64,
    /// 组成消息内容的实际数据段列表。
    #[serde(rename = "message")]
    pub segments: Vec<IncomingSegment>,
    /// 消息场景的类型标识符，对于好友消息通常是 "friend"。
    pub message_scene: String,
    /// 发送此消息的好友的详细信息。
    pub friend: Friend,
    /// 客户端生成的消息序列号，可用于去重等操作。
    pub client_seq: i64,
}

/// 代表接收到的群消息。
///
/// 继承自 [`IncomingMessage`] 并额外包含了群及发送成员的详细信息。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GroupMessage {
    /// 群号（即消息的接收群组）。
    pub peer_id: i64,
    /// 消息的序列号。
    pub message_seq: i64,
    /// 消息发送者的QQ号。
    pub sender_id: i64,
    /// 消息发送的Unix时间戳（秒）。
    pub time: i64,
    /// 组成消息内容的实际数据段列表。
    #[serde(rename = "message")]
    pub segments: Vec<IncomingSegment>,
    /// 消息场景的类型标识符，对于群消息通常是 "group"。
    pub message_scene: String,
    /// 消息所属群组的详细信息。
    pub group: Group,
    /// 发送此消息的群成员的详细信息。
    pub group_member: GroupMember,
}

/// 代表接收到的临时会话消息。
///
/// 继承自 [`IncomingMessage`] 并可能包含临时会话来源群组的信息。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TempMessage {
    /// 临时会话对方的QQ号。
    pub peer_id: i64,
    /// 消息的序列号。
    pub message_seq: i64,
    /// 消息发送者的QQ号。
    pub sender_id: i64,
    /// 消息发送的Unix时间戳（秒）。
    pub time: i64,
    /// 组成消息内容的实际数据段列表。
    #[serde(rename = "message")]
    pub segments: Vec<IncomingSegment>,
    /// 消息场景的类型标识符，对于临时会话消息通常是 "temp"。
    pub message_scene: String,
    /// 如果临时会话是通过某个群发起的，则此字段包含该群的详细信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

/// 代表接收到的合并转发消息中的单条消息内容。
///
/// 注意：此结构体名可能需要根据其在合并转发消息中的确切角色进行调整。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct IncomingForwardMessage {
    /// 该条转发消息的原始发送者QQ号。
    pub user_id: i64,
    /// 该条转发消息的原始发送者当时的昵称或名称。
    pub name: String,
    /// 组成该条转发消息内容的实际数据段列表。
    #[serde(rename = "message")]
    pub segments: Vec<IncomingSegment>,
}

/// 枚举了构成接收消息内容的各种可能的消息段类型。
///
/// 使用 `serde` 的 `tag` 和 `content` 属性进行反序列化：
/// - `tag = "type"`: JSON中用于区分消息段类型的字段名。
/// - `content = "data"`: JSON中包含该类型消息段具体数据的字段名。
///
/// 例如，一个文本消息段的JSON可能如下：
/// ```json
/// {
///   "type": "text",
///   "data": {
///     "text": "你好"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
pub enum IncomingSegment {
    /// 文本消息段。
    #[serde(rename = "text")]
    Text(TextData),

    /// 提及（@）某人的消息段。
    #[serde(rename = "mention")]
    Mention(MentionData),

    /// 提及（@）全体成员的消息段。
    #[serde(rename = "mention_all")]
    MentionAll(MentionAllData),

    /// QQ表情消息段。
    #[serde(rename = "face")]
    Face(FaceData),

    /// 回复消息段，用于引用之前的某条消息。
    #[serde(rename = "reply")]
    Reply(ReplyData),

    /// 图片消息段。
    #[serde(rename = "image")]
    Image(ImageData),

    /// 语音消息段。
    #[serde(rename = "record")]
    Record(RecordData),

    /// 视频消息段。
    #[serde(rename = "video")]
    Video(VideoData),

    /// 合并转发消息段。其内容通常是多条 [`IncomingForwardMessage`]。
    #[serde(rename = "forward")]
    Forward(ForwardData),

    /// 商城表情（大表情）消息段。
    #[serde(rename = "market_face")]
    MarketFace(MarketFaceData),

    /// 轻应用（小程序、小游戏卡片等）消息段。
    #[serde(rename = "light_app")]
    LightApp(LightAppData),

    /// XML 卡片消息段。
    #[serde(rename = "xml")]
    XML(XMLData),
}

/// 文本消息段的具体数据。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TextData {
    /// 实际的文本内容。
    pub text: String,
}

/// 提及（@）某人的消息段的具体数据。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MentionData {
    /// 被提及用户的QQ号。
    pub user_id: i64,
}

/// 提及（@）全体成员的消息段的具体数据。
/// 此结构体通常为空，仅作为类型标记。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MentionAllData;

/// QQ表情消息段的具体数据。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FaceData {
    /// QQ表情的内置ID。
    pub face_id: String,
}

/// 回复消息段的具体数据。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReplyData {
    /// 被回复（引用）的消息的序列号。
    pub message_seq: i64,
}

/// 图片消息段的具体数据。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ImageData {
    /// 图片的资源ID，可用于后续操作（如获取图片URL）。
    pub resource_id: String,
    /// 临时URL
    pub temp_url: String,
    /// 图片的预览文本或摘要（可选）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// 图片的子类型，例如 "normal" (普通图片), "sticker" (贴图表情) 等。
    pub sub_type: String,
}

/// 语音消息段的具体数据。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RecordData {
    /// 语音的资源ID，可用于后续操作。
    pub resource_id: String,
    /// 临时URL
    pub temp_url: String,
    /// 语音的时长（单位：秒）。
    pub duration: i32,
}

/// 视频消息段的具体数据。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct VideoData {
    /// 视频的资源ID，可用于后续操作。
    pub resource_id: String,
    /// 临时URL
    pub temp_url: String,
}

/// 合并转发消息段的具体数据。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ForwardData {
    /// 合并转发消息的ID，可用于获取转发消息的具体内容。
    pub forward_id: String,
}

/// 商城表情（大表情）消息段的具体数据。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MarketFaceData {
    /// 商城表情的图片URL。
    pub url: String,
}

/// 轻应用（小程序、小游戏卡片等）消息段的具体数据。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LightAppData {
    /// 轻应用的名称。
    pub app_name: String,
    /// 轻应用的JSON数据负载，具体结构由应用本身定义。
    pub json_payload: String,
}

/// XML 卡片消息段的具体数据。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct XMLData {
    /// XML消息的服务ID。
    pub service_id: i32,
    /// XML数据的字符串负载。
    pub xml_payload: String,
}
