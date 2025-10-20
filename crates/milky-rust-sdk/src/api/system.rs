//! 提供了与系统信息查询相关的API接口功能。
//!
//! 这包括获取登录QQ的信息、好友列表、群列表、以及特定好友或群成员的详细信息等。
//! 大部分查询接口都提供了一个 `no_cache` 选项，用于强制从服务器获取最新数据而非使用缓存。
//! 所有功能均通过 [`MilkyClient`] 的方法暴露。

use crate::client::MilkyClient;
use crate::error::Result;
use crate::types::{
    common::{Platform, Sex},
    friend::Friend,
    group::{Group, GroupMember},
};
use serde::{Deserialize, Serialize};

/// 获取当前登录账号信息的请求参数。
#[derive(Serialize)]
pub struct GetLoginInfoParams {}

/// 获取当前登录账号信息的响应数据。
#[derive(Deserialize, Debug)]
pub struct GetLoginInfoResponse {
    /// 当前登录的QQ号 (UIN)。
    pub uin: i64,
    /// 当前登录账号的昵称。
    pub nickname: String,
}

/// 获取协议端信息的请求参数。
#[derive(Serialize)]
pub struct GetImplInfoParams {}

/// 获取协议端信息的响应数据。
#[derive(Deserialize, Debug)]
pub struct GetImplInfoResponse {
    /// 协议端名称
    pub impl_name: String,
    /// 协议端版本
    pub impl_version: String,
    /// 协议端使用的QQ协议版本
    pub qq_protocol_version: String,
    /// 协议端使用的QQ协议平台
    pub qq_protocol_type: Platform,
    /// Milky版本
    pub milky_version: String,
}

/// 获取用户个人信息的请求参数。
#[derive(Serialize)]
pub struct GetUserProfileParams {
    /// 用户的QQ号
    pub user_id: i64,
}

/// 获取用户个人信息的响应数据。
#[derive(Deserialize, Debug)]
pub struct GetUserProfileResponse {
    /// 昵称
    pub nickname: String,
    /// QID
    pub qid: String,
    /// 性别
    pub sex: Sex,
    /// 用户的备注
    pub remark: String,
    /// 个性签名
    pub bio: String,
    /// 等级
    pub level: i32,
    /// 国家或地区
    pub country: String,
    /// 城市
    pub city: String,
    /// 学校
    pub school: String,
}

/// 获取好友列表的请求参数。
#[derive(Serialize)]
pub struct GetFriendListParams {
    /// 是否强制不使用缓存。默认为 `false`（即允许使用缓存）。
    #[serde(default)]
    pub no_cache: bool,
}

/// 获取好友列表的响应数据。
#[derive(Deserialize, Debug)]
pub struct GetFriendListResponse {
    /// 获取到的好友信息列表。
    pub friends: Vec<Friend>,
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

/// 获取指定好友信息的响应数据。
#[derive(Deserialize, Debug)]
pub struct GetFriendInfoResponse {
    pub friend: Friend,
}

/// 获取群列表的请求参数。
#[derive(Serialize)]
pub struct GetGroupListParams {
    /// 是否强制不使用缓存。默认为 `false`。
    #[serde(default)]
    pub no_cache: bool,
}

/// 获取群列表的响应数据。
#[derive(Deserialize, Debug)]
pub struct GetGroupListResponse {
    /// 获取到的群信息列表。
    pub groups: Vec<Group>,
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

/// 获取指定群信息的响应数据。
#[derive(Deserialize, Debug)]
pub struct GetGroupInfoResponse {
    pub group: Group,
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

/// 获取指定群成员列表的响应数据。
#[derive(Deserialize, Debug)]
pub struct GetGroupMemberListResponse {
    /// 获取到的群成员信息列表。
    pub members: Vec<GroupMember>,
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

/// 获取指定群成员信息的响应数据。
#[derive(Deserialize, Debug)]
pub struct GetGroupMemberInfoResponse {
    pub member: GroupMember,
}

/// 获取 Cookies 的请求参数。
#[derive(Serialize)]
pub struct GetCookiesParams {
    /// 需要获取 Cookies 的域名。
    pub domain: String,
}

/// 获取指定群的 Cookies 的响应数据。
#[derive(Deserialize, Debug)]
pub struct GetCookiesResponse {
    /// 域名对应的 Cookies 字符串
    pub cookies: String,
}

/// 获取 CSRF Token 的请求参数。
#[derive(Serialize)]
pub struct GetCsrfTokenParams {}

/// 获取 CSRF Token的响应数据
#[derive(Deserialize, Debug)]
pub struct GetCsrfTokenResponse {
    /// 获取到的 CSRF Token。
    pub csrf_token: String,
}

impl MilkyClient {
    /// 获取当前登录账号的基本信息。
    ///
    /// # 返回
    /// 成功则返回包含登录QQ号和昵称的 [`GetLoginInfoResponse`]。
    pub async fn get_login_info(&self) -> Result<GetLoginInfoResponse> {
        let params = GetLoginInfoParams {}; // 此API通常无参数
        self.send_request("get_login_info", params).await
    }

    /// 获取协议端信息
    ///
    /// # 返回
    /// 成功则返回包含协议端信息的 [`GetImplInfoResponse`]。
    pub async fn get_impl_info(&self) -> Result<GetImplInfoResponse> {
        let params = GetImplInfoParams {}; // 此API通常无参数
        self.send_request("get_impl_info", params).await
    }

    /// 获取指定用户的详细信息。
    ///
    /// # 参数
    /// * `user_id`: 要查询的用户的QQ号。
    ///
    /// # 返回
    /// 成功则返回该用户的详细信息 [`GetUserProfileResponse`]。
    pub async fn get_user_profile(&self, user_id: i64) -> Result<GetUserProfileResponse> {
        let params = GetUserProfileParams { user_id };
        self.send_request("get_user_profile", params).await
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
    /// 成功则返回该好友的详细信息 [`GetFriendInfoResponse`]。
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
    /// 成功则返回该群的详细信息 [`GetGroupInfoResponse`]。
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
    /// 成功则返回该群成员的详细信息 [`GetGroupMemberInfoResponse`]。
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

    /// 获取指定域名的 Cookies。
    ///
    /// # 返回
    /// 成功则返回包含 Cookies 的 [`GetCookiesResponse`]。
    pub async fn get_cookies(&self, domain: String) -> Result<GetCookiesResponse> {
        let params = GetCookiesParams { domain };
        self.send_request("get_cookies", params).await
    }

    /// 获取 CSRF Token。
    ///
    /// # 返回
    /// 成功则返回包含 CSRF Token 的 [`GetCsrfTokenResponse`]。
    pub async fn get_csrf_token(&self) -> Result<GetCsrfTokenResponse> {
        let params = GetCsrfTokenParams {}; // 此API通常无参数
        self.send_request("get_csrf_token", params).await
    }
}
