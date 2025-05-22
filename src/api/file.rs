//! 提供了与文件操作相关的API接口功能，包括私聊文件和群文件的上传、下载、管理等。
//!
//! 这些功能通过 [`MilkyClient`] 的方法暴露出来，每个方法对应一个特定的API端点。
//! 参数和响应都定义为独立的结构体，以增强类型安全和代码清晰度。

use crate::client::MilkyClient;
use crate::error::Result;
use crate::types::group::{GroupFile, GroupFolder};
use serde::{Deserialize, Serialize};

/// 上传私聊文件的请求参数。
#[derive(Serialize)]
pub struct UploadPrivateFileParams {
    /// 接收文件的好友QQ号。
    pub user_id: i64,
    /// 文件的统一资源标识符 (URI)。
    /// 支持 `file:///path/to/file` (本地文件),
    /// `http(s)://example.com/file` (网络URL),
    /// 以及 `base64://<BASE64编码的文件数据>` (Base64编码内容)。
    pub file_uri: String,
}

/// 上传私聊文件的响应数据。
#[derive(Deserialize, Debug)]
pub struct UploadPrivateFileResponse {
    /// 上传成功后，文件在服务器上的唯一ID。
    pub file_id: String,
}

/// 上传群文件的请求参数。
#[derive(Serialize)]
pub struct UploadGroupFileParams {
    /// 文件要上传到的目标群组的群号。
    pub group_id: i64,
    /// 文件的统一资源标识符 (URI)，格式同 [`UploadPrivateFileParams::file_uri`]。
    pub file_uri: String,
}

/// 上传群文件的响应数据。
#[derive(Deserialize, Debug)]
pub struct UploadGroupFileResponse {
    /// 上传成功后，文件在服务器上的唯一ID。
    pub file_id: String,
}

/// 获取私聊文件下载链接的请求参数。
#[derive(Serialize)]
pub struct GetPrivateFileDownloadUrlParams {
    /// 文件所属好友的QQ号。
    pub user_id: i64,
    /// 要获取下载链接的文件的ID。
    pub file_id: String,
}

/// 获取私聊文件下载链接的响应数据。
#[derive(Deserialize, Debug)]
pub struct GetPrivateFileDownloadUrlResponse {
    /// 文件的可直接访问的下载链接。
    pub download_url: String,
}

/// 获取群文件下载链接的请求参数。
#[derive(Serialize)]
pub struct GetGroupFileDownloadUrlParams {
    /// 文件所属群组的群号。
    pub group_id: i64,
    /// 要获取下载链接的文件的ID。
    pub file_id: String,
}

/// 获取群文件下载链接的响应数据。
#[derive(Deserialize, Debug)]
pub struct GetGroupFileDownloadUrlResponse {
    /// 文件的可直接访问的下载链接。
    pub download_url: String,
}

/// 获取群文件列表的请求参数。
#[derive(Serialize)]
pub struct GetGroupFilesParams {
    /// 要查询的群组的群号。
    pub group_id: i64,
    /// 要查询的父文件夹ID。如果为 `None` 或空字符串，则表示获取根目录下的文件和文件夹列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
}

/// 获取群文件列表的响应数据。
#[derive(Deserialize, Debug)]
pub struct GetGroupFilesResponse {
    /// 获取到的文件列表。
    pub files: Vec<GroupFile>,
    /// 获取到的文件夹列表。
    pub folder: Vec<GroupFolder>,
}

/// 移动群文件的请求参数。
#[derive(Serialize)]
pub struct MoveGroupFileParams {
    /// 文件所属群组的群号。
    pub group_id: i64,
    /// 要移动的文件的ID。
    pub file_id: String,
    /// 目标文件夹的ID。如果为 `None` 或空字符串，则表示移动到根目录。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_folder_id: Option<String>,
}

/// 重命名群文件的请求参数。
#[derive(Serialize)]
pub struct RenameGroupFileParams {
    /// 文件所属群组的群号。
    pub group_id: i64,
    /// 要重命名的文件的ID。
    pub file_id: String,
    /// 文件的新名称。
    pub new_name: String,
}

/// 删除群文件的请求参数。
#[derive(Serialize)]
pub struct DeleteGroupFileParams {
    /// 文件所属群组的群号。
    pub group_id: i64,
    /// 要删除的文件的ID。
    pub file_id: String,
}

/// 创建群文件夹的请求参数。
#[derive(Serialize)]
pub struct CreateGroupFolderParams {
    /// 要在其中创建文件夹的群组的群号。
    pub group_id: i64,
    /// 新文件夹的名称。
    pub folder_name: String,
}

/// 创建群文件夹的响应数据。
#[derive(Deserialize, Debug)]
pub struct CreateGroupFolderResponse {
    /// 创建成功后，新文件夹的唯一ID。
    pub folder_id: String,
}

/// 重命名群文件夹的请求参数。
#[derive(Serialize)]
pub struct RenameGroupFolderParams {
    /// 文件夹所属群组的群号。
    pub group_id: i64,
    /// 要重命名的文件夹的ID。
    pub folder_id: String,
    /// 文件夹的新名称。
    pub new_name: String,
}

/// 删除群文件夹的请求参数。
#[derive(Serialize)]
pub struct DeleteGroupFolderParams {
    /// 文件夹所属群组的群号。
    pub group_id: i64,
    /// 要删除的文件夹的ID。
    pub folder_id: String,
}

impl MilkyClient {
    /// 上传私聊文件到指定好友。
    ///
    /// # 参数
    /// * `user_id`: 接收文件的好友QQ号。
    /// * `file_uri`: 文件的URI，支持 `file://`, `http(s)://`, `base64://` 格式。
    ///
    /// # 返回
    /// 成功则返回包含文件ID的 [`UploadPrivateFileResponse`]。
    pub async fn upload_private_file(
        &self,
        user_id: i64,
        file_uri: String,
    ) -> Result<UploadPrivateFileResponse> {
        let params = UploadPrivateFileParams { user_id, file_uri };
        self.send_request("upload_private_file", params).await
    }

    /// 上传文件到指定群组。
    ///
    /// # 参数
    /// * `group_id`: 文件要上传到的目标群组的群号。
    /// * `file_uri`: 文件的URI，格式同上。
    ///
    /// # 返回
    /// 成功则返回包含文件ID的 [`UploadGroupFileResponse`]。
    pub async fn upload_group_file(
        &self,
        group_id: i64,
        file_uri: String,
    ) -> Result<UploadGroupFileResponse> {
        let params = UploadGroupFileParams { group_id, file_uri };
        self.send_request("upload_group_file", params).await
    }

    /// 获取指定私聊文件的下载链接。
    ///
    /// # 参数
    /// * `user_id`: 文件所属好友的QQ号。
    /// * `file_id`: 要获取下载链接的文件的ID。
    ///
    /// # 返回
    /// 成功则返回包含下载链接的 [`GetPrivateFileDownloadUrlResponse`]。
    pub async fn get_private_file_download_url(
        &self,
        user_id: i64,
        file_id: String,
    ) -> Result<GetPrivateFileDownloadUrlResponse> {
        let params = GetPrivateFileDownloadUrlParams { user_id, file_id };
        self.send_request("get_private_file_download_url", params)
            .await
    }

    /// 获取指定群文件的下载链接。
    ///
    /// # 参数
    /// * `group_id`: 文件所属群组的群号。
    /// * `file_id`: 要获取下载链接的文件的ID。
    ///
    /// # 返回
    /// 成功则返回包含下载链接的 [`GetGroupFileDownloadUrlResponse`]。
    pub async fn get_group_file_download_url(
        &self,
        group_id: i64,
        file_id: String,
    ) -> Result<GetGroupFileDownloadUrlResponse> {
        let params = GetGroupFileDownloadUrlParams { group_id, file_id };
        self.send_request("get_group_file_download_url", params)
            .await
    }

    /// 获取指定群组的文件和文件夹列表。
    ///
    /// # 参数
    /// * `group_id`: 要查询的群组的群号。
    /// * `parent_folder_id`: 父文件夹ID。若为 `None`，则获取根目录内容。
    ///
    /// # 返回
    /// 成功则返回包含文件和文件夹列表的 [`GetGroupFilesResponse`]。
    pub async fn get_group_files(
        &self,
        group_id: i64,
        parent_folder_id: Option<String>,
    ) -> Result<GetGroupFilesResponse> {
        let params = GetGroupFilesParams {
            group_id,
            parent_folder_id,
        };
        self.send_request("get_group_files", params).await
    }

    /// 移动群文件到指定文件夹。
    ///
    /// # 参数
    /// * `group_id`: 文件所属群组的群号。
    /// * `file_id`: 要移动的文件的ID。
    /// * `target_folder_id`: 目标文件夹ID。若为 `None`，则移动到根目录。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn move_group_file(
        &self,
        group_id: i64,
        file_id: String,
        target_folder_id: Option<String>,
    ) -> Result<()> {
        let params = MoveGroupFileParams {
            group_id,
            file_id,
            target_folder_id,
        };
        self.send_request("move_group_file", params).await
    }

    /// 重命名群文件。
    ///
    /// # 参数
    /// * `group_id`: 文件所属群组的群号。
    /// * `file_id`: 要重命名的文件的ID。
    /// * `new_name`: 文件的新名称。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn rename_group_file(
        &self,
        group_id: i64,
        file_id: String,
        new_name: String,
    ) -> Result<()> {
        let params = RenameGroupFileParams {
            group_id,
            file_id,
            new_name,
        };
        self.send_request("rename_group_file", params).await
    }

    /// 删除指定的群文件。
    ///
    /// # 参数
    /// * `group_id`: 文件所属群组的群号。
    /// * `file_id`: 要删除的文件的ID。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn delete_group_file(&self, group_id: i64, file_id: String) -> Result<()> {
        let params = DeleteGroupFileParams { group_id, file_id };
        self.send_request("delete_group_file", params).await
    }

    /// 在指定群组中创建新的文件夹。
    ///
    /// # 参数
    /// * `group_id`: 要在其中创建文件夹的群组的群号。
    /// * `folder_name`: 新文件夹的名称。
    ///
    /// # 返回
    /// 成功则返回包含新文件夹ID的 [`CreateGroupFolderResponse`]。
    pub async fn create_group_folder(
        &self,
        group_id: i64,
        folder_name: String,
    ) -> Result<CreateGroupFolderResponse> {
        let params = CreateGroupFolderParams {
            group_id,
            folder_name,
        };
        self.send_request("create_group_folder", params).await
    }

    /// 重命名群文件夹。
    ///
    /// # 参数
    /// * `group_id`: 文件夹所属群组的群号。
    /// * `folder_id`: 要重命名的文件夹的ID。
    /// * `new_name`: 文件夹的新名称。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn rename_group_folder(
        &self,
        group_id: i64,
        folder_id: String,
        new_name: String,
    ) -> Result<()> {
        let params = RenameGroupFolderParams {
            group_id,
            folder_id,
            new_name,
        };
        self.send_request("rename_group_folder", params).await
    }

    /// 删除指定的群文件夹。
    ///
    /// # 参数
    /// * `group_id`: 文件夹所属群组的群号。
    /// * `folder_id`: 要删除的文件夹的ID。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`。
    pub async fn delete_group_folder(&self, group_id: i64, folder_id: String) -> Result<()> {
        let params = DeleteGroupFolderParams {
            group_id,
            folder_id,
        };
        self.send_request("delete_group_folder", params).await
    }
}
