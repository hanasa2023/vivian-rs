use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Group {
    /// 群号
    pub group_id: i64,
    /// 群名称
    pub name: String,
    /// 群成员数量
    pub member_count: i32,
    /// 群容量
    pub max_member_count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GroupMember {
    /// 群号
    pub group_id: i64,
    /// 成员 QQ 号
    pub user_id: i64,
    /// 成员昵称
    pub nickname: String,
    /// 成员备注
    pub card: String,
    /// 专属头衔（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 性别（可能值：`male`, `female`, `unknown`）
    pub sex: String,
    /// 群等级，注意和 QQ 等级区分
    pub level: i32,
    /// 权限等级（可能值：`owner`, `admin`, `member`）
    pub role: String,
    /// 入群时间，Unix 时间戳（秒）
    pub join_time: i64,
    /// 最后发言时间，Unix 时间戳（秒）
    pub last_sent_time: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GroupAnnouncement {
    /// 群号
    pub group_id: i64,
    /// 公告 ID
    pub announcement_id: String,
    /// 发送者 QQ 号
    pub user_id: i64,
    /// Unix 时间戳（秒）
    pub time: i64,
    /// 公告内容
    pub content: String,
    /// 公告图片 URL（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GroupFile {
    /// 群号
    pub group_id: i64,
    /// 文件 ID
    pub file_id: String,
    /// 文件名称
    pub file_name: String,
    /// 父文件夹 ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// 文件大小（字节）
    pub file_size: i64,
    /// 上传时的 Unix 时间戳（秒）
    pub uploaded_time: i64,
    /// 过期时的 Unix 时间戳（秒）
    pub expire_time: i64,
    /// 上传者 QQ 号
    pub uploader_id: i64,
    /// 下载次数
    pub downloaded_times: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GroupFolder {
    /// 群号
    pub group_id: i64,
    /// 文件夹 ID
    pub folder_id: String,
    /// 父文件夹 ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// 文件夹名称
    pub folder_name: String,
    /// 创建时的 Unix 时间戳（秒）
    pub created_time: i64,
    /// 最后修改时的 Unix 时间戳（秒）
    pub lasy_modified_time: i64,
    /// 创建者 QQ 号
    pub crator_id: i64,
    /// 文件数量
    pub file_count: i32,
}
