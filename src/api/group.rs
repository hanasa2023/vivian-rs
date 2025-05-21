use crate::client::MilkyClient;
use crate::error::Result;
use crate::types::group::GroupAnnouncement;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct SetGroupNameParams {
    /// 群号
    pub group_id: i64,
    /// 新群名称
    pub name: String,
}

#[derive(Serialize)]
pub struct SetGroupAvatarParams {
    /// 群号
    pub group_id: i64,
    /// 图像文件 URI，支持 `file://` `http(s)://` `base64://` 三种格式
    pub image_uri: String,
}

#[derive(Serialize)]
pub struct SetGroupMemberCardParams {
    /// 群号
    pub group_id: i64,
    /// 被设置的群成员 QQ 号
    pub user_id: i64,
    /// 新群名片
    pub card: String,
}

#[derive(Serialize)]
pub struct SetGroupMemberSpecialTitleParams {
    /// 群号
    pub group_id: i64,
    /// 被设置的群成员 QQ 号
    pub user_id: i64,
    /// 新专属头衔
    pub special_title: String,
}

#[derive(Serialize)]
pub struct SetGroupMemberAdminParams {
    /// 群号
    pub group_id: i64,
    /// 被设置的 QQ 号
    pub user_id: i64,
    /// 是否设置为管理员，false 为取消管理员（默认值：true）
    #[serde(default = "default_true")]
    pub is_set: bool,
}

#[allow(dead_code)]
fn default_true() -> bool {
    true
}

#[derive(Serialize)]
pub struct SetGroupMemberMuteParams {
    /// 群号
    pub group_id: i64,
    /// 被设置的 QQ 号
    pub user_id: i64,
    /// 禁言持续时间（秒），设为 0 为取消禁言（默认值：0）
    #[serde(default)]
    pub duration: i64,
}

#[derive(Serialize)]
pub struct SetGroupWholeMuteParams {
    /// 群号
    pub group_id: i64,
    /// 是否开启全员禁言，false 为取消全员禁言（默认值：true）
    #[serde(default = "default_true")]
    pub is_mute: bool,
}

#[derive(Serialize)]
pub struct KickGroupMemberParams {
    /// 群号
    pub group_id: i64,
    /// 被踢的 QQ 号
    pub user_id: i64,
    /// 是否拒绝加群申请，false 为不拒绝（默认值：true）
    #[serde(default = "default_true")]
    pub reject_add_request: bool,
}

#[derive(Serialize)]
pub struct GetGroupAnnouncementListParams {
    /// 群号
    pub group_id: i64,
}

#[derive(Deserialize, Debug)]
pub struct GetGroupAnnouncementListResponse {
    /// 群公告列表
    pub announcements: Vec<GroupAnnouncement>,
}

#[derive(Serialize)]
pub struct SendGroupAnnouncementParams {
    /// 群号
    pub group_id: i64,
    /// 公告内容
    pub content: String,
    /// 图像文件 URI，支持 `file://` `http(s)://` `base64://` 三种格式
    pub image_uri: String,
}

#[derive(Serialize)]
pub struct DeleteGroupAnnouncementParams {
    /// 群号
    pub group_id: i64,
    /// 公告 ID
    pub announcement_id: i64,
}

#[derive(Serialize)]
pub struct QuitGroupParams {
    /// 群号
    pub group_id: i64,
}

#[derive(Serialize)]
pub struct SendGroupMessageReactionParams {
    /// 要回应的消息序列号
    pub message_seq: i64,
    /// 表情 ID
    pub reaction: String,
    /// 是否添加表情，false 为取消（默认值：true）
    #[serde(default = "default_true")]
    pub is_add: bool,
}

#[derive(Serialize)]
pub struct SendGroupNudgeParams {
    /// 群号
    pub group_id: i64,
    /// 被戳的群成员 QQ 号
    pub user_id: i64,
}

impl MilkyClient {
    /// 设置群名称
    pub async fn set_group_name(&self, group_id: i64, name: String) -> Result<()> {
        let params = SetGroupNameParams { group_id, name };
        self.send_request("set_group_name", params).await
    }

    /// 设置群头像
    pub async fn set_group_avatar(&self, group_id: i64, image_uri: String) -> Result<()> {
        let params = SetGroupAvatarParams {
            group_id,
            image_uri,
        };
        self.send_request("set_group_avatar", params).await
    }

    /// 设置群名片
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

    /// 设置群成员专属头衔
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

    /// 设置群管理员
    pub async fn set_group_member_admin(
        &self,
        group_id: i64,
        user_id: i64,
        is_set: Option<bool>,
    ) -> Result<()> {
        let is_set = is_set.unwrap_or(true);
        let params = SetGroupMemberAdminParams {
            group_id,
            user_id,
            is_set,
        };
        self.send_request("set_group_member_admin", params).await
    }

    /// 设置群成员禁言
    pub async fn set_group_member_mute(
        &self,
        group_id: i64,
        user_id: i64,
        duration: Option<i64>,
    ) -> Result<()> {
        let duration = duration.unwrap_or(0);
        let params = SetGroupMemberMuteParams {
            group_id,
            user_id,
            duration,
        };
        self.send_request("set_group_member_mute", params).await
    }

    /// 设置群全员禁言
    pub async fn set_group_whole_mute(&self, group_id: i64, is_mute: Option<bool>) -> Result<()> {
        let is_mute = is_mute.unwrap_or(true);
        let params = SetGroupWholeMuteParams { group_id, is_mute };
        self.send_request("set_group_whole_mute", params).await
    }

    /// 踢出群成员
    pub async fn kick_group_member(
        &self,
        group_id: i64,
        user_id: i64,
        reject_add_request: Option<bool>,
    ) -> Result<()> {
        let reject_add_request = reject_add_request.unwrap_or(true);
        let params = KickGroupMemberParams {
            group_id,
            user_id,
            reject_add_request,
        };
        self.send_request("kick_group_member", params).await
    }

    /// 获取群公告列表
    pub async fn get_group_announcement_list(
        &self,
        group_id: i64,
    ) -> Result<GetGroupAnnouncementListResponse> {
        let params = GetGroupAnnouncementListParams { group_id };
        self.send_request("get_group_announcement_list", params)
            .await
    }

    /// 发送群公告
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

    /// 删除群公告
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

    /// 退出群
    pub async fn quit_group(&self, group_id: i64) -> Result<()> {
        let params = QuitGroupParams { group_id };
        self.send_request("quit_group", params).await
    }

    /// 发送群消息表情回应
    pub async fn send_group_message_reaction(
        &self,
        message_seq: i64,
        reaction: String,
        is_add: Option<bool>,
    ) -> Result<()> {
        let is_add = is_add.unwrap_or(true);
        let params = SendGroupMessageReactionParams {
            message_seq,
            reaction,
            is_add,
        };
        self.send_request("send_group_message_reaction", params)
            .await
    }

    /// 发送群戳一戳
    pub async fn send_group_nudge(&self, group_id: i64, user_id: i64) -> Result<()> {
        let params = SendGroupNudgeParams { group_id, user_id };
        self.send_request("send_group_nudge", params).await
    }
}
