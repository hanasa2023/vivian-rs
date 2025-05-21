use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OutgoingForwardMessage {
    /// 发送者 QQ 号
    pub user_id: i64,
    /// 发送者名称
    pub name: String,
    /// 消息段列表
    pub segments: Vec<OutgoingSegment>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
pub enum OutgoingSegment {
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
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TextData {
    /// 文本内容
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MentionData {
    /// 提及的 QQ 号
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MentionAllData;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FaceData {
    /// 表情ID
    pub face_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReplyData {
    /// 被引用的消息序列号
    pub message_seq: i64,
    /// 被引用的消息的客户端序列号，在回复私聊消息时必须提供（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_seq: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ImageData {
    /// 文件 URI，支持 `file://` `http(s)://` `base64://` 三种格式
    pub uri: String,
    /// 图片预览文本（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// 图片类型（可能值：`normal`, `sticker`）
    pub sub_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RecordData {
    /// 文件 URI，支持 `file://` `http(s)://` `base64://` 三种格式
    pub uri: String,
    /// 语音时长（秒）
    pub duration: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct VideoData {
    /// 文件 URI，支持 `file://` `http(s)://` `base64://` 三种格式
    pub uri: String,
    /// 封面图片 URI（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_uri: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ForwardData {
    /// 合并转发 ID
    pub forward_id: String,
}
