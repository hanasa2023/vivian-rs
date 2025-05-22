//! 定义与群组及其相关信息（如成员、公告、文件等）的数据结构。

use serde::{Deserialize, Serialize};

/// 代表一个群组的基本信息。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Group {
    /// 群组的唯一标识符（群号）。
    pub group_id: i64,
    /// 群组的名称。
    pub name: String,
    /// 当前群组的成员数量。
    pub member_count: i32,
    /// 群组的最大成员容量。
    pub max_member_count: i32,
}

/// 代表一个群组成员的详细信息。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GroupMember {
    /// 该成员所属群组的唯一标识符（群号）。
    pub group_id: i64,
    /// 成员的QQ号。
    pub user_id: i64,
    /// 成员在群组中显示的昵称。
    pub nickname: String,
    /// 成员在群组中的备注名（群名片）。
    pub card: String,
    /// 成员的专属头衔（可选）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 成员的性别。
    /// 可能的值包括: "male" (男), "female" (女), "unknown" (未知)。
    pub sex: String,
    /// 成员在群内的等级（注意与QQ等级区分）。
    pub level: i32,
    /// 成员在群内的权限角色。
    /// 可能的值包括: "owner" (群主), "admin" (管理员), "member" (普通成员)。
    pub role: String,
    /// 成员加入群组的时间，表示为Unix时间戳（秒）。
    pub join_time: i64,
    /// 成员最后发言的时间，表示为Unix时间戳（秒）。
    pub last_sent_time: i64,
}

/// 代表一条群公告的信息。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GroupAnnouncement {
    /// 该公告所属群组的唯一标识符（群号）。
    pub group_id: i64,
    /// 公告的唯一ID。
    pub announcement_id: String,
    /// 发布公告的成员的QQ号。
    pub user_id: i64,
    /// 公告发布的时间，表示为Unix时间戳（秒）。
    pub time: i64,
    /// 公告的文本内容。
    pub content: String,
    /// 公告中附带的图片URL（可选）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

/// 代表群文件系统中的一个文件。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GroupFile {
    /// 该文件所属群组的唯一标识符（群号）。
    pub group_id: i64,
    /// 文件的唯一ID。
    pub file_id: String,
    /// 文件的名称。
    pub file_name: String,
    /// 文件所在的父文件夹ID（可选，如果文件位于根目录则可能为None）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// 文件的大小（字节）。
    pub file_size: i64,
    /// 文件上传的时间，表示为Unix时间戳（秒）。
    pub uploaded_time: i64,
    /// 文件过期的时间，表示为Unix时间戳（秒）。
    pub expire_time: i64,
    /// 文件上传者的QQ号。
    pub uploader_id: i64,
    /// 文件的下载次数。
    pub downloaded_times: i32,
}

/// 代表群文件系统中的一个文件夹。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GroupFolder {
    /// 该文件夹所属群组的唯一标识符（群号）。
    pub group_id: i64,
    /// 文件夹的唯一ID。
    pub folder_id: String,
    /// 文件夹所在的父文件夹ID（可选，如果文件夹位于根目录则可能为None）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// 文件夹的名称。
    pub folder_name: String,
    /// 文件夹创建的时间，表示为Unix时间戳（秒）。
    pub created_time: i64,
    /// 文件夹最后修改的时间，表示为Unix时间戳（秒）。
    pub last_modified_time: i64,
    /// 文件夹创建者的QQ号。
    pub creator_id: i64,
    /// 文件夹中包含的文件数量。
    pub file_count: i32,
}
