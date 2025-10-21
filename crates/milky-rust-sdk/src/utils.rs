use milky_types::message::in_coming::IncomingSegment;

/// 从消息段列表中提取所有文本内容并拼接成一个字符串
///
/// # 参数
/// * `segments`: 一个包含 `IncomingSegment` 的向量引用
///
/// # 返回
/// 一个 `String`，其中包含所有 `TextData` 段落拼接后的文本内容
/// 如果没有文本段落，则返回空字符串
pub fn get_plain_text_from_segments(segments: &[IncomingSegment]) -> String {
    segments
        .iter()
        .filter_map(|segment| {
            if let IncomingSegment::Text { text } = segment {
                Some(text.as_str())
            } else {
                None
            }
        })
        .collect()
}
