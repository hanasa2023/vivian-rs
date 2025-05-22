//! 提供了与系统信息查询相关的API接口功能。
//!
//! 这包括获取登录QQ的信息、好友列表、群列表、以及特定好友或群成员的详细信息等。
//! 大部分查询接口都提供了一个 `no_cache` 选项，用于强制从服务器获取最新数据而非使用缓存。
//! 所有功能均通过 [`MilkyClient`] 的方法暴露。

use crate::client::MilkyClient;
use crate::error::Result;
use crate::types::{
    friend::Friend,              // 假设 Friend 定义在 vivian/src/types/friend.rs
    group::{Group, GroupMember}, // 假设 Group 和 GroupMember 定义在 vivian/src/types/group.rs
};
use serde::{Deserialize, Serialize};

/// 获取当前登录账号信息的请求参数。
/// 此结构体为空，因为此API调用通常不需要额外参数。
#[derive(Serialize)]
pub struct GetLoginInfoParams;

/// 获取好友列表的请求参数。
#[derive(Serialize)]
pub struct GetFriendListParams {
    /// 是否强制不使用缓存，直接从服务器获取最新列表。
    /// 默认为 `false`（即允许使用缓存）。
    /// `#[serde(default)]` 确保如果调用者未提供此字段，则使用类型的默认值（对于bool是false）。
    #[serde(default)]
    pub no_cache: bool,
}

/// 获取指定好友信息的请求参数。
#[derive(Serialize)]
pub struct GetFriendInfoParams {
    /// 要查询的好友的QQ号。
    pub user_id: i64,
    /// 是否强制不使用缓存。默认为 `false`。
    #[serde(default)]
    pub no_cache: bool,
}

/// 获取群列表的请求参数。
#[derive(Serialize)]
pub struct GetGroupListParams {
    /// 是否强制不使用缓存。默认为 `false`。
    #[serde(default)]
    pub no_cache: bool,
}

/// 获取指定群信息的请求参数。
#[derive(Serialize)]
pub struct GetGroupInfoParams {
    /// 要查询的群组的群号。
    pub group_id: i64,
    /// 是否强制不使用缓存。默认为 `false`。
    #[serde(default)]
    pub no_cache: bool,
}

/// 获取指定群成员列表的请求参数。
#[derive(Serialize)]
pub struct GetGroupMemberListParams {
    /// 要查询的群组的群号。
    pub group_id: i64,
    /// 是否强制不使用缓存。默认为 `false`。
    #[serde(default)]
    pub no_cache: bool,
}

/// 获取指定群成员信息的请求参数。
#[derive(Serialize)]
pub struct GetGroupMemberInfoParams {
    /// 成员所属群组的群号。
    pub group_id: i64,
    /// 要查询的群成员的QQ号。
    pub user_id: i64,
    /// 是否强制不使用缓存。默认为 `false`。
    #[serde(default)]
    pub no_cache: bool,
}

/// 获取当前登录账号信息的响应数据。
#[derive(Deserialize, Debug)]
pub struct GetLoginInfoResponse {
    /// 当前登录的QQ号 (UIN)。
    pub uin: i64,
    /// 当前登录账号的昵称。
    pub nickname: String,
}

/// 获取好友列表的响应数据。
#[derive(Deserialize, Debug)]
pub struct GetFriendListResponse {
    /// 获取到的好友信息列表。
    pub friends: Vec<Friend>,
}

/// 获取指定好友信息的响应数据类型别名。
/// 响应直接是 [`Friend`] 结构体。
pub type GetFriendInfoResponse = Friend;

/// 获取群列表的响应数据。
#[derive(Deserialize, Debug)]
pub struct GetGroupListResponse {
    /// 获取到的群信息列表。
    pub groups: Vec<Group>,
}

/// 获取指定群信息的响应数据类型别名。
/// 响应直接是 [`Group`] 结构体。
pub type GetGroupInfoResponse = Group;

/// 获取指定群成员列表的响应数据。
#[derive(Deserialize, Debug)]
pub struct GetGroupMemberListResponse {
    /// 获取到的群成员信息列表。
    pub members: Vec<GroupMember>,
}

/// 获取指定群成员信息的响应数据类型别名。
/// 响应直接是 [`GroupMember`] 结构体。
pub type GetGroupMemberInfoResponse = GroupMember;

impl MilkyClient {
    /// 获取当前登录账号的基本信息。
    ///
    /// # 返回
    /// 成功则返回包含登录QQ号和昵称的 [`GetLoginInfoResponse`]。
    pub async fn get_login_info(&self) -> Result<GetLoginInfoResponse> {
        let params = GetLoginInfoParams; // 此API通常无参数
        self.send_request("get_login_info", params).await
    }

    /// 获取当前账号的好友列表。
    ///
    /// # 参数
    /// * `no_cache`: 是否强制不使用缓存，直接从服务器获取。
    ///
    /// # 返回
    /// 成功则返回包含好友列表的 [`GetFriendListResponse`]。
    pub async fn get_friend_list(&self, no_cache: bool) -> Result<GetFriendListResponse> {
        let params = GetFriendListParams { no_cache };
        self.send_request("get_friend_list", params).await
    }

    /// 获取指定好友的详细信息。
    ///
    /// # 参数
    /// * `user_id`: 要查询的好友的QQ号。
    /// * `no_cache`: 是否强制不使用缓存。
    ///
    /// # 返回
    /// 成功则返回该好友的详细信息 [`GetFriendInfoResponse`] (即 [`Friend`] 类型)。
    pub async fn get_friend_info(
        &self,
        user_id: i64,
        no_cache: bool,
    ) -> Result<GetFriendInfoResponse> {
        let params = GetFriendInfoParams { user_id, no_cache };
        self.send_request("get_friend_info", params).await
    }

    /// 获取当前账号加入的群列表。
    ///
    /// # 参数
    /// * `no_cache`: 是否强制不使用缓存。
    ///
    /// # 返回
    /// 成功则返回包含群列表的 [`GetGroupListResponse`]。
    pub async fn get_group_list(&self, no_cache: bool) -> Result<GetGroupListResponse> {
        let params = GetGroupListParams { no_cache };
        self.send_request("get_group_list", params).await
    }

    /// 获取指定群的详细信息。
    ///
    /// # 参数
    /// * `group_id`: 要查询的群组的群号。
    /// * `no_cache`: 是否强制不使用缓存。
    ///
    /// # 返回
    /// 成功则返回该群的详细信息 [`GetGroupInfoResponse`] (即 [`Group`] 类型)。
    pub async fn get_group_info(
        &self,
        group_id: i64,
        no_cache: bool,
    ) -> Result<GetGroupInfoResponse> {
        let params = GetGroupInfoParams { group_id, no_cache };
        self.send_request("get_group_info", params).await
    }

    /// 获取指定群的成员列表。
    ///
    /// # 参数
    /// * `group_id`: 要查询的群组的群号。
    /// * `no_cache`: 是否强制不使用缓存。
    ///
    /// # 返回
    /// 成功则返回包含群成员列表的 [`GetGroupMemberListResponse`]。
    pub async fn get_group_member_list(
        &self,
        group_id: i64,
        no_cache: bool,
    ) -> Result<GetGroupMemberListResponse> {
        let params = GetGroupMemberListParams { group_id, no_cache };
        self.send_request("get_group_member_list", params).await
    }

    /// 获取指定群成员的详细信息。
    ///
    /// # 参数
    /// * `group_id`: 成员所属群组的群号。
    /// * `user_id`: 要查询的群成员的QQ号。
    /// * `no_cache`: 是否强制不使用缓存。
    ///
    /// # 返回
    /// 成功则返回该群成员的详细信息 [`GetGroupMemberInfoResponse`] (即 [`GroupMember`] 类型)。
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
