//! 提供了与群组管理和互动相关的API接口功能。
//!
//! 这包括设置群信息、管理群成员、处理群公告、以及发送群内互动（如戳一戳、表情回应）等操作。
//! 所有功能均通过 [`MilkyClient`] 的方法暴露。

use crate::client::MilkyClient;
use crate::error::Result;
use crate::types::group::GroupAnnouncement; // 假设 GroupAnnouncement 定义在 vivian/src/types/group.rs
use serde::{Deserialize, Serialize};

/// 设置群名称的请求参数。
#[derive(Serialize)]
pub struct SetGroupNameParams {
    /// 要操作的目标群组的群号。
    pub group_id: i64,
    /// 要设置的新的群名称。
    pub name: String,
}

/// 设置群头像的请求参数。
#[derive(Serialize)]
pub struct SetGroupAvatarParams {
    /// 要操作的目标群组的群号。
    pub group_id: i64,
    /// 图像文件的统一资源标识符 (URI)。
    /// 支持 `file:///path/to/image` (本地文件),
    /// `http(s)://example.com/image` (网络URL),
    /// 以及 `base64://<BASE64编码的图像数据>` (Base64编码内容)。
    pub image_uri: String,
}

/// 设置群成员名片（备注）的请求参数。
#[derive(Serialize)]
pub struct SetGroupMemberCardParams {
    /// 目标群组的群号。
    pub group_id: i64,
    /// 要设置名片的群成员的QQ号。
    pub user_id: i64,
    /// 要设置的新的群名片内容。如果设置为空字符串，通常表示删除名片。
    pub card: String,
}

/// 设置群成员专属头衔的请求参数。
#[derive(Serialize)]
pub struct SetGroupMemberSpecialTitleParams {
    /// 目标群组的群号。
    pub group_id: i64,
    /// 要设置专属头衔的群成员的QQ号。
    pub user_id: i64,
    /// 要设置的新的专属头衔。如果设置为空字符串，通常表示删除专属头衔。
    pub special_title: String,
}

/// 设置群成员管理员权限的请求参数。
#[derive(Serialize)]
pub struct SetGroupMemberAdminParams {
    /// 目标群组的群号。
    pub group_id: i64,
    /// 要操作的群成员的QQ号。
    pub user_id: i64,
    /// 是否设置为管理员。`true` 为设置，`false` 为取消管理员。
    /// 默认为 `true`。
    #[serde(default = "default_true")]
    pub is_set: bool,
}

/// 辅助函数，用于 `serde` 的 `default` 属性，返回 `true`。
#[allow(dead_code)] // 允许未使用代码，因为它仅由 serde 使用
fn default_true() -> bool {
    true
}

/// 设置群成员禁言的请求参数。
#[derive(Serialize)]
pub struct SetGroupMemberMuteParams {
    /// 目标群组的群号。
    pub group_id: i64,
    /// 要操作的群成员的QQ号。
    pub user_id: i64,
    /// 禁言的持续时间（单位：秒）。
    /// 设置为 `0` 表示取消禁言。默认为 `0`。
    #[serde(default)]
    pub duration: i64,
}

/// 设置全群禁言的请求参数。
#[derive(Serialize)]
pub struct SetGroupWholeMuteParams {
    /// 目标群组的群号。
    pub group_id: i64,
    /// 是否开启全员禁言。`true` 为开启，`false` 为取消。
    /// 默认为 `true`。
    #[serde(default = "default_true")]
    pub is_mute: bool,
}

/// 踢出群成员的请求参数。
#[derive(Serialize)]
pub struct KickGroupMemberParams {
    /// 目标群组的群号。
    pub group_id: i64,
    /// 要踢出的群成员的QQ号。
    pub user_id: i64,
    /// 是否拒绝该用户再次加入群申请。`true` 为拒绝，`false` 为不拒绝。
    /// 默认为 `true`。
    #[serde(default = "default_true")]
    pub reject_add_request: bool,
}

/// 获取群公告列表的请求参数。
#[derive(Serialize)]
pub struct GetGroupAnnouncementListParams {
    /// 目标群组的群号。
    pub group_id: i64,
}

/// 获取群公告列表的响应数据。
#[derive(Deserialize, Debug)]
pub struct GetGroupAnnouncementListResponse {
    /// 获取到的群公告列表。
    pub announcements: Vec<GroupAnnouncement>,
}

/// 发送（发布）群公告的请求参数。
#[derive(Serialize)]
pub struct SendGroupAnnouncementParams {
    /// 目标群组的群号。
    pub group_id: i64,
    /// 公告的文本内容。
    pub content: String,
    /// 公告附带的图像文件URI（可选，若不需要图片则可传入空字符串或根据API具体要求处理）。
    /// 支持格式同 [`SetGroupAvatarParams::image_uri`]。
    pub image_uri: String,
}

/// 删除群公告的请求参数。
#[derive(Serialize)]
pub struct DeleteGroupAnnouncementParams {
    /// 目标群组的群号。
    pub group_id: i64,
    /// 要删除的群公告的ID。
    pub announcement_id: i64,
}

/// 机器人退出群组的请求参数。
#[derive(Serialize)]
pub struct QuitGroupParams {
    /// 要退出的群组的群号。
    pub group_id: i64,
}

/// 发送群消息表情回应的请求参数。
#[derive(Serialize)]
pub struct SendGroupMessageReactionParams {
    /// 要回应的目标消息的序列号 (`message_seq`)。
    pub message_seq: i64,
    /// 要发送的表情回应的ID。
    pub reaction: String,
    /// 操作类型，`true` 为添加表情回应，`false` 为取消表情回应。
    /// 默认为 `true`。
    #[serde(default = "default_true")]
    pub is_add: bool,
}

/// 发送群内戳一戳的请求参数。
#[derive(Serialize)]
pub struct SendGroupNudgeParams {
    /// 目标群组的群号。
    pub group_id: i64,
    /// 要戳一戳的目标群成员的QQ号。
    pub user_id: i64,
}

impl MilkyClient {
    /// 设置指定群组的名称。
    ///
    /// # 参数
    /// * `group_id`: 目标群组的群号。
    /// * `name`: 新的群名称。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn set_group_name(&self, group_id: i64, name: String) -> Result<()> {
        let params = SetGroupNameParams { group_id, name };
        self.send_request("set_group_name", params).await
    }

    /// 设置指定群组的头像。
    ///
    /// # 参数
    /// * `group_id`: 目标群组的群号。
    /// * `image_uri`: 图像文件的URI，支持 `file://`, `http(s)://`, `base64://` 格式。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn set_group_avatar(&self, group_id: i64, image_uri: String) -> Result<()> {
        let params = SetGroupAvatarParams {
            group_id,
            image_uri,
        };
        self.send_request("set_group_avatar", params).await
    }

    /// 设置指定群组成员的群名片（备注）。
    ///
    /// # 参数
    /// * `group_id`: 目标群组的群号。
    /// * `user_id`: 目标成员的QQ号。
    /// * `card`: 新的群名片内容。空字符串通常用于清除名片。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn set_group_member_card(
        &self,
        group_id: i64,
        user_id: i64,
        card: String,
    ) -> Result<()> {
        let params = SetGroupMemberCardParams {
            group_id,
            user_id,
            card,
        };
        self.send_request("set_group_member_card", params).await
    }

    /// 设置指定群组成员的专属头衔。
    ///
    /// # 参数
    /// * `group_id`: 目标群组的群号。
    /// * `user_id`: 目标成员的QQ号。
    /// * `special_title`: 新的专属头衔。空字符串通常用于清除头衔。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn set_group_member_special_title(
        &self,
        group_id: i64,
        user_id: i64,
        special_title: String,
    ) -> Result<()> {
        let params = SetGroupMemberSpecialTitleParams {
            group_id,
            user_id,
            special_title,
        };
        self.send_request("set_group_member_special_title", params)
            .await
    }

    /// 设置或取消指定群组成员的管理员权限。
    ///
    /// # 参数
    /// * `group_id`: 目标群组的群号。
    /// * `user_id`: 目标成员的QQ号。
    /// * `is_set`: 可选参数，`Some(true)` 为设置管理员，`Some(false)` 为取消管理员。
    ///             若为 `None`，则默认为 `true` (设置管理员)。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn set_group_member_admin(
        &self,
        group_id: i64,
        user_id: i64,
        is_set: Option<bool>,
    ) -> Result<()> {
        let is_set = is_set.unwrap_or(true); // 默认为 true
        let params = SetGroupMemberAdminParams {
            group_id,
            user_id,
            is_set,
        };
        self.send_request("set_group_member_admin", params).await
    }

    /// 对指定群组成员进行禁言或解除禁言。
    ///
    /// # 参数
    /// * `group_id`: 目标群组的群号。
    /// * `user_id`: 目标成员的QQ号。
    /// * `duration`: 可选参数，禁言时长（秒）。`Some(0)` 或 `None` 表示解除禁言。
    ///               默认为 `0` (解除禁言)。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn set_group_member_mute(
        &self,
        group_id: i64,
        user_id: i64,
        duration: Option<i64>,
    ) -> Result<()> {
        let duration = duration.unwrap_or(0); // 默认为 0
        let params = SetGroupMemberMuteParams {
            group_id,
            user_id,
            duration,
        };
        self.send_request("set_group_member_mute", params).await
    }

    /// 对指定群组开启或关闭全员禁言。
    ///
    /// # 参数
    /// * `group_id`: 目标群组的群号。
    /// * `is_mute`: 可选参数，`Some(true)` 为开启全员禁言，`Some(false)` 为关闭。
    ///              若为 `None`，则默认为 `true` (开启全员禁言)。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn set_group_whole_mute(&self, group_id: i64, is_mute: Option<bool>) -> Result<()> {
        let is_mute = is_mute.unwrap_or(true); // 默认为 true
        let params = SetGroupWholeMuteParams { group_id, is_mute };
        self.send_request("set_group_whole_mute", params).await
    }

    /// 从指定群组中踢出成员。
    ///
    /// # 参数
    /// * `group_id`: 目标群组的群号。
    /// * `user_id`: 要踢出的成员的QQ号。
    /// * `reject_add_request`: 可选参数，是否拒绝该用户再次加入群的申请。
    ///                         `Some(true)` 为拒绝，`Some(false)` 为不拒绝。
    ///                         若为 `None`，则默认为 `true` (拒绝再次加群)。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn kick_group_member(
        &self,
        group_id: i64,
        user_id: i64,
        reject_add_request: Option<bool>,
    ) -> Result<()> {
        let reject_add_request = reject_add_request.unwrap_or(true); // 默认为 true
        let params = KickGroupMemberParams {
            group_id,
            user_id,
            reject_add_request,
        };
        self.send_request("kick_group_member", params).await
    }

    /// 获取指定群组的公告列表。
    ///
    /// # 参数
    /// * `group_id`: 目标群组的群号。
    ///
    /// # 返回
    /// 成功则返回包含公告列表的 [`GetGroupAnnouncementListResponse`]。
    pub async fn get_group_announcement_list(
        &self,
        group_id: i64,
    ) -> Result<GetGroupAnnouncementListResponse> {
        let params = GetGroupAnnouncementListParams { group_id };
        self.send_request("get_group_announcement_list", params)
            .await
    }

    /// 在指定群组中发送（发布）一条新的公告。
    ///
    /// # 参数
    /// * `group_id`: 目标群组的群号。
    /// * `content`: 公告的文本内容。
    /// * `image_uri`: 公告附带的图片URI。如果不需要图片，处理方式需参照API具体要求（可能为空字符串或省略）。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn send_group_announcement(
        &self,
        group_id: i64,
        content: String,
        image_uri: String,
    ) -> Result<()> {
        let params = SendGroupAnnouncementParams {
            group_id,
            content,
            image_uri,
        };
        self.send_request("send_group_announcement", params).await
    }

    /// 删除指定群组中的一条公告。
    ///
    /// # 参数
    /// * `group_id`: 目标群组的群号。
    /// * `announcement_id`: 要删除的公告的ID。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn delete_group_announcement(
        &self,
        group_id: i64,
        announcement_id: i64,
    ) -> Result<()> {
        let params = DeleteGroupAnnouncementParams {
            group_id,
            announcement_id,
        };
        self.send_request("delete_group_announcement", params).await
    }

    /// 使机器人退出指定的群组。
    ///
    /// # 参数
    /// * `group_id`: 机器人要退出的群组的群号。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn quit_group(&self, group_id: i64) -> Result<()> {
        let params = QuitGroupParams { group_id };
        self.send_request("quit_group", params).await
    }

    /// 对群消息发送表情回应。
    ///
    /// # 参数
    /// * `message_seq`: 要回应的目标消息的序列号。
    /// * `reaction`: 要发送的表情回应的ID。
    /// * `is_add`: 可选参数，`Some(true)` 为添加表情回应，`Some(false)` 为取消。
    ///             若为 `None`，则默认为 `true` (添加回应)。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn send_group_message_reaction(
        &self,
        message_seq: i64,
        reaction: String,
        is_add: Option<bool>,
    ) -> Result<()> {
        let is_add = is_add.unwrap_or(true); // 默认为 true
        let params = SendGroupMessageReactionParams {
            message_seq,
            reaction,
            is_add,
        };
        self.send_request("send_group_message_reaction", params)
            .await
    }

    /// 在群内发送戳一戳给指定成员。
    ///
    /// # 参数
    /// * `group_id`: 目标群组的群号。
    /// * `user_id`: 要戳一戳的目标群成员的QQ号。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn send_group_nudge(&self, group_id: i64, user_id: i64) -> Result<()> {
        let params = SendGroupNudgeParams { group_id, user_id };
        self.send_request("send_group_nudge", params).await
    }
}
