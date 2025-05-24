//! 提供了与消息处理相关的API接口功能。
//!
//! 这包括发送私聊和群聊消息、获取特定消息、获取历史消息、
//! 获取消息中的资源（如图片、语音）的临时下载链接、获取合并转发消息内容以及撤回消息等操作。
//! 所有功能均通过 [`MilkyClient`] 的方法暴露。

use crate::client::MilkyClient;
use crate::error::Result;
use crate::types::message::in_coming::IncomingMessage;
use crate::types::message::out_going::OutgoingSegment;
use serde::{Deserialize, Serialize};

/// 发送私聊消息的请求参数。
#[derive(Serialize)]
pub struct SendPrivateMsgParams {
    /// 接收消息的好友的QQ号。
    pub user_id: i64,
    /// 要发送的消息内容，由一个或多个 [`OutgoingSegment`] 组成。
    pub message: Vec<OutgoingSegment>,
}

/// 发送私聊消息的响应数据。
#[derive(Deserialize, Debug)]
#[allow(dead_code)] // 允许未使用代码，因为此结构体主要用于反序列化API响应
pub struct SendPrivateMsgResponse {
    /// 服务器生成的消息序列号 (`message_seq`)。
    pub message_seq: i64,
    /// 消息在服务器的发送时间（Unix时间戳，秒）。
    pub time: i64,
    /// 客户端生成的消息序列号 (`client_seq`)，用于去重或追踪。
    pub client_seq: i64,
}

/// 发送群聊消息的请求参数。
#[derive(Serialize)]
pub struct SendGroupMsgParams {
    /// 接收消息的群组的群号。
    pub group_id: i64,
    /// 要发送的消息内容，由一个或多个 [`OutgoingSegment`] 组成。
    pub message: Vec<OutgoingSegment>,
}

/// 发送群聊消息的响应数据。
#[derive(Deserialize, Debug)]
#[allow(dead_code)] // 允许未使用代码
pub struct SendGroupMsgResponse {
    /// 服务器生成的消息序列号 (`message_seq`)。
    pub message_seq: i64,
    /// 消息在服务器的发送时间（Unix时间戳，秒）。
    pub time: i64,
}

/// 获取单条消息的请求参数。
#[derive(Serialize)]
pub struct GetMsgParams {
    /// 消息所属的场景。
    /// 可能的值包括: "friend" (好友), "group" (群组), "temp" (临时会话)。
    pub message_scene: String,
    /// 消息所属的好友QQ号或群号。
    pub peer_id: i64,
    /// 要获取的消息的序列号 (`message_seq`)。
    pub message_seq: i64,
}

/// 获取单条消息的响应数据。
#[derive(Deserialize, Debug)]
#[allow(dead_code)] // 允许未使用代码
pub struct GetMsgResponse {
    /// 获取到的消息内容。
    pub message: IncomingMessage,
}

/// 获取历史消息记录的请求参数。
#[derive(Serialize)]
pub struct GetHistoryMsgParams {
    /// 消息所属的场景 (例如: "friend", "group", "temp")。
    pub message_scene: String,
    /// 消息所属的好友QQ号或群号。
    pub peer_id: i64,
    /// 起始消息的序列号 (`message_seq`)。
    /// 如果不提供此参数，则通常从最新的消息开始获取。可选。
    pub start_message_seq: Option<i64>,
    /// 消息获取方向。
    /// 可能的值包括: "newer" (获取比 `start_message_seq` 更新的消息),
    /// "older" (获取比 `start_message_seq` 更旧的消息)。
    pub direction: String,
    /// 获取的最大消息数量。
    /// 默认值为 `20`。
    pub limit: i32,
}

/// 获取历史消息记录的响应数据。
#[derive(Deserialize, Debug)]
#[allow(dead_code)] // 允许未使用代码
pub struct GetHistoryMsgResponse {
    /// 获取到的消息列表。
    /// 注意：列表中的某些消息可能由于已被撤回等原因而不存在实际内容。
    pub messages: Vec<IncomingMessage>,
}

/// 获取消息中资源（如图片、语音、文件）的临时下载链接的请求参数。
#[derive(Serialize)]
pub struct GetResourceTempUrlParams {
    /// 资源的ID。这个ID通常从接收到的消息段（如 [`ImageData`](crate::types::message::in_coming::ImageData)）中获取。
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
    /// 合并转发消息的ID。这个ID通常从接收到的 [`ForwardData`](crate::types::message::in_coming::ForwardData) 消息段中获取。
    pub forward_id: String,
}

/// 获取合并转发消息内容的响应数据。
#[derive(Deserialize, Debug)]
#[allow(dead_code)] // 允许未使用代码
pub struct GetForwardedMessagesResponse {
    /// 合并转发消息中包含的原始消息列表。
    pub messages: Vec<IncomingMessage>,
}

/// 撤回私聊消息的请求参数。
#[derive(Serialize)]
pub struct RecallPrivateMessageParams {
    /// 消息所属好友的QQ号。
    pub user_id: i64,
    /// 要撤回的消息的序列号 (`message_seq`)。
    pub message_seq: i64,
    /// 要撤回消息的客户端序列号 (`client_seq`)。对于私聊消息撤回，此字段通常是必需的。
    pub client_seq: i64,
}

/// 撤回群聊消息的请求参数。
#[derive(Serialize)]
pub struct RecallGroupMessageParams {
    /// 消息所属群组的群号。
    pub group_id: i64,
    /// 要撤回的消息的序列号 (`message_seq`)。
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
    /// 成功则返回包含消息回执信息（如 `message_seq`）的 [`SendPrivateMsgResponse`]。
    pub async fn send_private_msg(
        &self,
        user_id: i64,
        message: Vec<OutgoingSegment>,
    ) -> Result<SendPrivateMsgResponse> {
        let params = SendPrivateMsgParams { user_id, message };
        self.send_request("send_private_message", params).await
    }

    /// 发送群聊消息到指定群组。
    ///
    /// # 参数
    /// * `group_id`: 接收消息的群组的群号。
    /// * `message`: 由一个或多个 [`OutgoingSegment`] 组成的消息内容。
    ///
    /// # 返回
    /// 成功则返回包含消息回执信息（如 `message_seq`）的 [`SendGroupMsgResponse`]。
    pub async fn send_group_msg(
        &self,
        group_id: i64,
        message: Vec<OutgoingSegment>,
    ) -> Result<SendGroupMsgResponse> {
        let params = SendGroupMsgParams { group_id, message };
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
    /// 成功则返回包含消息内容的 [`GetMsgResponse`]。
    pub async fn get_msg(
        &self,
        message_scene: &str,
        peer_id: i64,
        message_seq: i64,
    ) -> Result<GetMsgResponse> {
        let params = GetMsgParams {
            message_scene: message_scene.to_string(),
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
    /// * `direction`: 获取方向 ("newer" 或 "older")。
    /// * `limit`: 可选的获取消息数量上限。若为 `None`，则默认为20条。
    ///
    /// # 返回
    /// 成功则返回包含历史消息列表的 [`GetHistoryMsgResponse`]。
    pub async fn get_history_messages(
        &self,
        message_scene: &str,
        peer_id: i64,
        start_message_seq: Option<i64>,
        direction: &str,
        limit: Option<i32>,
    ) -> Result<GetHistoryMsgResponse> {
        let limit = limit.unwrap_or(20); // 默认获取20条
        let params = GetHistoryMsgParams {
            message_scene: message_scene.to_string(),
            peer_id,
            start_message_seq,
            direction: direction.to_string(),
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
    /// * `forward_id`: 合并转发消息的ID，通常从 `ForwardData` 消息段中获得。
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
    /// * `client_seq`: 要撤回消息的客户端序列号 (对于私聊消息通常是必需的)。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn recall_private_message(
        &self,
        user_id: i64,
        message_seq: i64,
        client_seq: i64,
    ) -> Result<()> {
        let params = RecallPrivateMessageParams {
            user_id,
            message_seq,
            client_seq,
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
}
