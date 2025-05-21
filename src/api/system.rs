use crate::client::MilkyClient;
use crate::error::Result;
use crate::types::{
    friend::Friend,
    group::{Group, GroupMember},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GetLoginInfoParams;

#[derive(Serialize)]
pub struct GetFriendListParams {
    /// 是否强制不使用缓存（默认值：`false`）
    #[serde(default)]
    pub no_cache: bool,
}

#[derive(Serialize)]
pub struct GetFriendInfoParams {
    /// 好友 QQ 号
    pub user_id: i64,
    /// 是否强制不使用缓存（默认值：`false`）
    #[serde(default)]
    pub no_cache: bool,
}

#[derive(Serialize)]
pub struct GetGroupListParams {
    /// 是否强制不使用缓存（默认值：`false`）
    #[serde(default)]
    pub no_cache: bool,
}

#[derive(Serialize)]
pub struct GetGroupInfoParams {
    /// 群号
    pub group_id: i64,
    /// 是否强制不使用缓存（默认值：`false`）
    #[serde(default)]
    pub no_cache: bool,
}

#[derive(Serialize)]
pub struct GetGroupMemberListParams {
    /// 群号
    pub group_id: i64,
    /// 是否强制不使用缓存（默认值：`false`）
    #[serde(default)]
    pub no_cache: bool,
}

#[derive(Serialize)]
pub struct GetGroupMemberInfoParams {
    /// 群号
    pub group_id: i64,
    /// 群成员 QQ 号
    pub user_id: i64,
    /// 是否强制不使用缓存（默认值：`false`）
    #[serde(default)]
    pub no_cache: bool,
}

#[derive(Deserialize, Debug)]
pub struct GetLoginInfoResponse {
    /// 登录 QQ 号
    pub uin: i64,
    /// 登录昵称
    pub nickname: String,
}

#[derive(Deserialize, Debug)]
pub struct GetFriendListResponse {
    /// 好友列表
    pub friends: Vec<Friend>,
}

pub type GetFriendInfoResponse = Friend;

#[derive(Deserialize, Debug)]
pub struct GetGroupListResponse {
    /// 群列表
    pub groups: Vec<Group>,
}

pub type GetGroupInfoResponse = Group;

#[derive(Deserialize, Debug)]
pub struct GetGroupMemberListResponse {
    /// 群成员列表
    pub members: Vec<GroupMember>,
}

pub type GetGroupMemberInfoResponse = GroupMember;

impl MilkyClient {
    /// 获取登录信息
    pub async fn get_login_info(&self) -> Result<GetLoginInfoResponse> {
        let params = GetLoginInfoParams;
        self.send_request("get_login_info", params).await
    }

    /// 获取好友列表
    pub async fn get_friend_list(&self, no_cache: bool) -> Result<GetFriendListResponse> {
        let params = GetFriendListParams { no_cache };
        self.send_request("get_friend_list", params).await
    }

    /// 获取好友信息
    pub async fn get_friend_info(
        &self,
        user_id: i64,
        no_cache: bool,
    ) -> Result<GetFriendInfoResponse> {
        let params = GetFriendInfoParams { user_id, no_cache };
        self.send_request("get_friend_info", params).await
    }

    /// 获取群列表
    pub async fn get_group_list(&self, no_cache: bool) -> Result<GetGroupListResponse> {
        let params = GetGroupListParams { no_cache };
        self.send_request("get_group_list", params).await
    }

    /// 获取群信息
    pub async fn get_group_info(
        &self,
        group_id: i64,
        no_cache: bool,
    ) -> Result<GetGroupInfoResponse> {
        let params = GetGroupInfoParams { group_id, no_cache };
        self.send_request("get_group_info", params).await
    }

    /// 获取群成员列表
    pub async fn get_group_member_list(
        &self,
        group_id: i64,
        no_cache: bool,
    ) -> Result<GetGroupMemberListResponse> {
        let params = GetGroupMemberListParams { group_id, no_cache };
        self.send_request("get_group_member_list", params).await
    }

    /// 获取群成员信息
    pub async fn get_group_member_info(
        &self,
        group_id: i64,
        user_id: i64,
        no_cache: bool,
    ) -> Result<GetGroupMemberInfoResponse> {
        let params = GetGroupMemberInfoParams {
            group_id,
            user_id,
            no_cache,
        };
        self.send_request("get_group_member_info", params).await
    }
}
