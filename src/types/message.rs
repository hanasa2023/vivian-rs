pub mod in_coming;
pub mod out_going;

use crate::types::message::in_coming::IncomingSegment;
use tokio_tungstenite::tungstenite::Message as WsMessage;

pub enum OriginalMessage {
    Ws(WsMessage),
    WebHook(serde_json::Value),
}

/// 从消息段列表中提取所有文本内容并拼接成一个字符串。
///
/// # 参数
/// * `segments`: 一个包含 `IncomingSegment` 的向量引用。
///
/// # 返回
/// 一个 `String`，其中包含所有 `TextData` 段落拼接后的文本内容。
/// 如果没有文本段落，则返回空字符串。
pub fn get_plain_text_from_segments(segments: &Vec<IncomingSegment>) -> String {
    let mut combined_text = String::new();

    for segment in segments {
        if let IncomingSegment::Text(text_data) = segment {
            combined_text.push_str(&text_data.text);
        }
    }

    combined_text
}
