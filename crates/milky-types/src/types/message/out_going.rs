//! 定义了用于发送消息的各类数据结构，包括消息段和特定的消息格式（如合并转发）

use serde::{Deserialize, Serialize};

/// 代表一条待发送的合并转发消息中的单条消息内容
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OutgoingForwardMessage {
    /// 发送者QQ号
    pub user_id: i64,
    /// 发送者名称
    pub sender_name: String,
    /// 组成该条转发消息内容的实际数据段列表
    pub segments: Vec<OutgoingSegment>,
}

/// 枚举构成待发送消息内容的各种可能的消息段类型
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
pub enum OutgoingSegment {
    /// 文本消息段
    #[serde(rename = "text")]
    Text(TextData),

    /// 提及（@）某人的消息段
    #[serde(rename = "mention")]
    Mention(MentionData),

    /// 提及（@）全体成员的消息段
    #[serde(rename = "mention_all")]
    MentionAll(MentionAllData),

    /// QQ表情消息段
    #[serde(rename = "face")]
    Face(FaceData),

    /// 回复消息段，用于引用之前的某条消息进行回复
    #[serde(rename = "reply")]
    Reply(ReplyData),

    /// 图片消息段
    #[serde(rename = "image")]
    Image(ImageData),

    /// 语音消息段
    #[serde(rename = "record")]
    Record(RecordData),

    /// 视频消息段
    #[serde(rename = "video")]
    Video(VideoData),

    /// 合并转发消息段
    #[serde(rename = "forward")]
    Forward(ForwardData),
}

/// 待发送的文本消息段的具体数据
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TextData {
    /// 要发送的实际文本内容
    pub text: String,
}

/// 待发送的提及（@）某人的消息段的具体数据
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MentionData {
    /// 要提及的用户的QQ号
    pub user_id: i64,
}

/// 待发送的提及（@）全体成员的消息段的具体数据
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MentionAllData;

/// 待发送的QQ表情消息段的具体数据
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FaceData {
    /// QQ表情的内置ID
    pub face_id: String,
}

/// 待发送的回复消息段的具体数据
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReplyData {
    /// 要回复（引用）的消息的序列号 (`message_seq`)
    pub message_seq: i64,
}

/// 待发送的图片消息段的具体数据
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ImageData {
    /// 图片文件的统一资源标识符 (URI)
    /// 支持三种格式:
    /// - `file:///path/to/image.jpg` (本地文件路径)
    /// - `http://example.com/image.png` 或 `https://example.com/image.png` (网络URL)
    /// - `base64://<BASE64编码的图片数据>` (Base64编码的图片内容)
    pub uri: String,
    /// 图片的预览文本（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// 图片类型
    /// 可能的值包括: "normal" (普通图片), "sticker" (贴图表情) 等
    pub sub_type: String,
}

/// 待发送的语音消息段的具体数据
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RecordData {
    /// 语音文件的统一资源标识符 (URI)
    /// 支持三种格式:
    /// - `file:///path/to/image.jpg` (本地文件路径)
    /// - `http://example.com/image.png` 或 `https://example.com/image.png` (网络URL)
    /// - `base64://<BASE64编码的图片数据>` (Base64编码的图片内容)
    pub uri: String,
}

/// 待发送的视频消息段的具体数据
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct VideoData {
    /// 视频文件的统一资源标识符 (URI)
    /// 支持三种格式:
    /// - `file:///path/to/image.jpg` (本地文件路径)
    /// - `http://example.com/image.png` 或 `https://example.com/image.png` (网络URL)
    /// - `base64://<BASE64编码的图片数据>` (Base64编码的图片内容)
    pub uri: String,
    /// 视频封面图片的URI（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_uri: Option<String>,
}

/// 待发送的（已存在的）合并转发消息段的具体数据
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ForwardData {
    /// 合并转发消息段
    pub messages: Vec<OutgoingForwardMessage>,
}
