//! 提供了与消息处理相关的API接口功能。
//!
//! 这包括发送私聊和群聊消息、获取特定消息、获取历史消息、
//! 获取消息中的资源（如图片、语音）的临时下载链接、获取合并转发消息内容以及撤回消息等操作。
//! 所有功能均通过 [`MilkyClient`] 的方法暴露。

use crate::error::Result;
use crate::types::common::MessageScene;
use crate::types::message::in_coming::IncomingMessage;
use crate::types::message::out_going::OutgoingSegment;
use crate::{client::MilkyClient, in_coming::IncomingForwardMessage};
use serde::{Deserialize, Serialize};

/// 发送私聊消息的请求参数。
#[derive(Serialize)]
pub struct SendPrivateMessageParams {
    /// 接收消息的好友的QQ号。
    pub user_id: i64,
    /// 要发送的消息内容，由一个或多个 [`OutgoingSegment`] 组成。
    pub message: Vec<OutgoingSegment>,
}

/// 发送私聊消息的响应数据。
#[derive(Deserialize, Debug)]
#[allow(dead_code)] // 允许未使用代码，因为此结构体主要用于反序列化API响应
pub struct SendPrivateMessageResponse {
    /// 消息序列号
    pub message_seq: i64,
    /// 消息的发送时间（Unix时间戳，秒）
    pub time: i64,
}

/// 发送群聊消息的请求参数。
#[derive(Serialize)]
pub struct SendGroupMessageParams {
    /// 接收消息的群组的群号。
    pub group_id: i64,
    /// 要发送的消息内容，由一个或多个 [`OutgoingSegment`] 组成。
    pub message: Vec<OutgoingSegment>,
}

/// 发送群聊消息的响应数据。
#[derive(Deserialize, Debug)]
pub struct SendGroupMessageResponse {
    /// 消息序列号
    pub message_seq: i64,
    /// 消息发送时间（Unix时间戳，秒）
    pub time: i64,
}

/// 获取单条消息的请求参数。
#[derive(Serialize)]
pub struct GetMessageParams {
    /// 消息场景
    pub message_scene: MessageScene,
    /// 消息所属的好友QQ号或群号。
    pub peer_id: i64,
    /// 要获取的消息的序列号。
    pub message_seq: i64,
}

/// 获取单条消息的响应数据。
#[derive(Deserialize, Debug)]
#[allow(dead_code)] // 允许未使用代码
pub struct GetMessageResponse {
    /// 获取到的消息内容。
    pub message: IncomingMessage,
}

/// 获取历史消息记录的请求参数
#[derive(Serialize)]
pub struct GetHistoryMessageParams {
    /// 消息所属的场景
    pub message_scene: MessageScene,
    /// 消息所属的好友QQ号或群号
    pub peer_id: i64,
    /// 起始消息的序列号 (`message_seq`)。
    /// 如果不提供此参数，则通常从最新的消息开始获取。可选。
    pub start_message_seq: Option<i64>,
    /// 获取的最大消息数量。
    /// 默认值为 `20`, 最多30条。
    pub limit: i32,
}

/// 获取历史消息记录的响应数据。
#[derive(Deserialize, Debug)]
#[allow(dead_code)] // 允许未使用代码
pub struct GetHistoryMessageResponse {
    /// 获取到的消息列表。
    /// 注意：列表中的某些消息可能由于已被撤回等原因而不存在实际内容。
    pub messages: Vec<IncomingMessage>,
    /// 下一页起始消息序列号
    pub next_message_seq: Option<i64>,
}

/// 获取消息中资源（如图片、语音、文件）的临时下载链接的请求参数。
#[derive(Serialize)]
pub struct GetResourceTempUrlParams {
    /// 资源ID
    pub resource_id: String,
}

/// 获取消息中资源的临时下载链接的响应数据。
#[derive(Deserialize, Debug)]
#[allow(dead_code)] // 允许未使用代码
pub struct GetResourceTempUrlResponse {
    /// 获取到的临时下载链接。此链接通常有有效期。
    pub url: String,
}

/// 获取合并转发消息内容的请求参数。
#[derive(Serialize)]
pub struct GetForwardedMessagesParams {
    /// 转发消息ID
    pub forward_id: String,
}

/// 获取合并转发消息内容的响应数据。
#[derive(Deserialize, Debug)]
#[allow(dead_code)] // 允许未使用代码
pub struct GetForwardedMessagesResponse {
    /// 合并转发消息中包含的原始消息列表。
    pub messages: Vec<IncomingForwardMessage>,
}

/// 撤回私聊消息的请求参数。
#[derive(Serialize)]
pub struct RecallPrivateMessageParams {
    /// 消息所属好友的QQ号。
    pub user_id: i64,
    /// 要撤回的消息的序列号
    pub message_seq: i64,
}

/// 撤回群聊消息的请求参数。
#[derive(Serialize)]
pub struct RecallGroupMessageParams {
    /// 消息所属群组的群号。
    pub group_id: i64,
    /// 要撤回的消息的序列号 (`message_seq`)。
    pub message_seq: i64,
}

/// 标记消息为已读的请求参数。
#[derive(Serialize)]
pub struct MarkMessageAsReadParams {
    /// 消息所属的场景
    pub message_scene: MessageScene,
    /// 好友QQ号或群号。
    pub peer_id: i64,
    /// 标为已读的消息序列号，该消息及更早的消息将被标记为已读
    pub message_seq: i64,
}

impl MilkyClient {
    /// 发送私聊消息给指定好友。
    ///
    /// # 参数
    /// * `user_id`: 接收消息的好友QQ号。
    /// * `message`: 由一个或多个 [`OutgoingSegment`] 组成的消息内容。
    ///
    /// # 返回
    /// 成功则返回包含消息回执信息（如 `message_seq`）的 [`SendPrivateMessageResponse`]。
    pub async fn send_private_message(
        &self,
        user_id: i64,
        message: Vec<OutgoingSegment>,
    ) -> Result<SendPrivateMessageResponse> {
        let params = SendPrivateMessageParams { user_id, message };
        self.send_request("send_private_message", params).await
    }

    /// 发送群聊消息到指定群组。
    ///
    /// # 参数
    /// * `group_id`: 接收消息的群组的群号。
    /// * `message`: 由一个或多个 [`OutgoingSegment`] 组成的消息内容。
    ///
    /// # 返回
    /// 成功则返回包含消息回执信息（如 `message_seq`）的 [`SendGroupMessageResponse`]。
    pub async fn send_group_message(
        &self,
        group_id: i64,
        message: Vec<OutgoingSegment>,
    ) -> Result<SendGroupMessageResponse> {
        let params = SendGroupMessageParams { group_id, message };
        self.send_request("send_group_message", params).await
    }

    /// 获取指定场景下的单条消息内容。
    ///
    /// # 参数
    /// * `message_scene`: 消息场景 (例如: "friend", "group")。
    /// * `peer_id`: 好友QQ号或群号。
    /// * `message_seq`: 要获取的消息的序列号。
    ///
    /// # 返回
    /// 成功则返回包含消息内容的 [`GetMessageResponse`]。
    pub async fn get_message(
        &self,
        message_scene: MessageScene,
        peer_id: i64,
        message_seq: i64,
    ) -> Result<GetMessageResponse> {
        let params = GetMessageParams {
            message_scene,
            peer_id,
            message_seq,
        };
        self.send_request("get_message", params).await
    }

    /// 获取指定场景下的历史消息记录。
    ///
    /// # 参数
    /// * `message_scene`: 消息场景。
    /// * `peer_id`: 好友QQ号或群号。
    /// * `start_message_seq`: 可选的起始消息序列号。若为 `None`，则从最新消息开始。
    /// * `limit`: 可选的获取消息数量上限。若为 `None`，则默认为20条。
    ///
    /// # 返回
    /// 成功则返回包含历史消息列表的 [`GetHistoryMessageResponse`]。
    pub async fn get_history_messages(
        &self,
        message_scene: MessageScene,
        peer_id: i64,
        start_message_seq: Option<i64>,
        limit: Option<i32>,
    ) -> Result<GetHistoryMessageResponse> {
        let limit = limit.unwrap_or(20); // 默认获取20条
        let params = GetHistoryMessageParams {
            message_scene,
            peer_id,
            start_message_seq,
            limit,
        };
        self.send_request("get_history_messages", params).await
    }

    /// 获取消息中特定资源（如图片、语音）的临时下载URL。
    ///
    /// # 参数
    /// * `resource_id`: 资源的唯一标识符，通常从消息段中获得。
    ///
    /// # 返回
    /// 成功则返回包含临时URL的 [`GetResourceTempUrlResponse`]。
    pub async fn get_resource_temp_url(
        &self,
        resource_id: &str,
    ) -> Result<GetResourceTempUrlResponse> {
        let params = GetResourceTempUrlParams {
            resource_id: resource_id.to_string(),
        };
        self.send_request("get_resource_temp_url", params).await
    }

    /// 获取合并转发消息的具体内容。
    ///
    /// # 参数
    /// * `forward_id`: 合并转发消息的ID
    ///
    /// # 返回
    /// 成功则返回包含转发消息列表的 [`GetForwardedMessagesResponse`]。
    pub async fn get_forwarded_messages(
        &self,
        forward_id: &str,
    ) -> Result<GetForwardedMessagesResponse> {
        let params = GetForwardedMessagesParams {
            forward_id: forward_id.to_string(),
        };
        self.send_request("get_forwarded_messages", params).await
    }

    /// 撤回指定的私聊消息。
    ///
    /// # 参数
    /// * `user_id`: 消息所属好友的QQ号。
    /// * `message_seq`: 要撤回消息的服务器序列号。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn recall_private_message(&self, user_id: i64, message_seq: i64) -> Result<()> {
        let params = RecallPrivateMessageParams {
            user_id,
            message_seq,
        };
        self.send_request("recall_private_message", params).await
    }

    /// 撤回指定的群聊消息。
    ///
    /// # 参数
    /// * `group_id`: 消息所属群组的群号。
    /// * `message_seq`: 要撤回消息的服务器序列号。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn recall_group_message(&self, group_id: i64, message_seq: i64) -> Result<()> {
        let params = RecallGroupMessageParams {
            group_id,
            message_seq,
        };
        self.send_request("recall_group_message", params).await
    }

    /// 将指定消息标记为已读。
    ///
    /// # 参数
    /// * `message_scene`: 消息所属的场景（好友、群组等）。
    /// * `peer_id`: 好友QQ号或群号。
    /// * `message_seq`: 要标记为已读的消息序列号。该消息及更早的消息将被标记为已读。
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn mark_message_as_read(
        &self,
        message_scene: MessageScene,
        peer_id: i64,
        message_seq: i64,
    ) -> Result<()> {
        let params = MarkMessageAsReadParams {
            message_scene,
            peer_id,
            message_seq,
        };
        self.send_request("mark_message_as_read", params).await
    }
}
