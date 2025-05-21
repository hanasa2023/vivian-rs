use crate::client::MilkyClient;
use crate::error::Result;
use crate::types::message::in_coming::IncomingMessage;
use crate::types::message::out_going::OutgoingSegment;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct SendPrivateMsgParams {
    /// 好友QQ号
    pub user_id: i64,
    /// 消息内容
    pub message: Vec<OutgoingSegment>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SendPrivateMsgResponse {
    /// 消息序列号
    pub message_seq: i64,
    /// 消息发送时间
    pub time: i64,
    /// 消息的客户端序列号
    pub client_seq: i64,
}

#[derive(Serialize)]
pub struct SendGroupMsgParams {
    /// 群号
    pub group_id: i64,
    /// 消息内容
    pub message: Vec<OutgoingSegment>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SendGroupMsgResponse {
    /// 消息序列号
    pub message_seq: i64,
    /// 消息发送时间
    pub time: i64,
}

#[derive(Serialize)]
pub struct GetMsgParams {
    /// 消息场景（可能值：`friend`, `group`, `temp`）
    pub message_scene: String,
    /// 好友 QQ 号或群号
    pub peer_id: i64,
    /// 消息序列号
    pub message_seq: i64,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct GetMsgResponse {
    /// 消息内容
    pub message: IncomingMessage,
}

#[derive(Serialize)]
pub struct GetHistoryMsgParams {
    /// 消息场景（可能值：`friend`, `group`, `temp`）
    pub message_scene: String,
    /// 好友 QQ 号或群号
    pub peer_id: i64,
    /// 起始消息序列号，不提供则从最新消息开始（可选）
    pub start_message_seq: Option<i64>,
    /// 消息获取方向（可能值：`newer`, `older`）
    pub direction: String,
    /// 获取的最大消息数量（默认值：`20`）
    pub limit: i32,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct GetHistoryMsgResponse {
    /// 获取到的消息，部分消息可能不存在，如撤回的消息
    pub messages: Vec<IncomingMessage>,
}

#[derive(Serialize)]
pub struct GetResourceTempUrlParams {
    /// 资源 ID
    pub resource_id: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct GetResourceTempUrlResponse {
    /// 临时资源链接
    pub url: String,
}

#[derive(Serialize)]
pub struct GetForwardedMessagesParams {
    /// 转发消息 ID
    pub forward_id: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct GetForwardedMessagesResponse {
    /// 转发消息内容
    pub messages: Vec<IncomingMessage>,
}

#[derive(Serialize)]
pub struct RecallPrivateMessageParams {
    /// 好友 QQ 号
    pub user_id: i64,
    /// 消息序列号
    pub message_seq: i64,
    /// 客户端序列号
    pub client_seq: i64,
}

#[derive(Serialize)]
pub struct RecallGroupMessageParams {
    /// 群号
    pub group_id: i64,
    /// 消息序列号
    pub message_seq: i64,
}

impl MilkyClient {
    pub async fn send_private_msg(
        &self,
        user_id: i64,
        message: Vec<OutgoingSegment>,
    ) -> Result<SendPrivateMsgResponse> {
        let params = SendPrivateMsgParams { user_id, message };
        self.send_request("send_private_msg", params).await
    }

    pub async fn send_group_msg(
        &self,
        group_id: i64,
        message: Vec<OutgoingSegment>,
    ) -> Result<SendGroupMsgResponse> {
        let params = SendGroupMsgParams { group_id, message };
        self.send_request("send_group_msg", params).await
    }

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
        self.send_request("get_msg", params).await
    }

    pub async fn get_history_messages(
        &self,
        message_scene: &str,
        peer_id: i64,
        start_message_seq: Option<i64>,
        direction: &str,
        limit: Option<i32>,
    ) -> Result<GetHistoryMsgResponse> {
        let limit = limit.unwrap_or(20);
        let params = GetHistoryMsgParams {
            message_scene: message_scene.to_string(),
            peer_id,
            start_message_seq,
            direction: direction.to_string(),
            limit,
        };
        self.send_request("get_history_messages", params).await
    }

    pub async fn get_resource_temp_url(
        &self,
        resource_id: &str,
    ) -> Result<GetResourceTempUrlResponse> {
        let params = GetResourceTempUrlParams {
            resource_id: resource_id.to_string(),
        };
        self.send_request("get_resource_temp_url", params).await
    }

    pub async fn get_forwarded_messages(
        &self,
        forward_id: &str,
    ) -> Result<GetForwardedMessagesResponse> {
        let params = GetForwardedMessagesParams {
            forward_id: forward_id.to_string(),
        };
        self.send_request("get_forwarded_messages", params).await
    }

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

    pub async fn recall_group_message(&self, group_id: i64, message_seq: i64) -> Result<()> {
        let params = RecallGroupMessageParams {
            group_id,
            message_seq,
        };
        self.send_request("recall_group_message", params).await
    }
}
