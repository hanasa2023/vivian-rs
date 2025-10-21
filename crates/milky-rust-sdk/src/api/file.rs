//! 提供了与文件操作相关的API接口功能，包括私聊文件和群文件的上传、下载、管理等

use crate::client::MilkyClient;
use crate::error::Result;
use milky_types::group::{GroupFile, GroupFolder};
use serde::{Deserialize, Serialize};

/// 上传私聊文件的请求参数
#[derive(Serialize)]
pub struct UploadPrivateFileRequest {
    /// 接收文件的好友QQ号
    pub user_id: i64,
    /// 文件的统一资源标识符 (URI)
    /// 支持:
    /// - `file:///path/to/file` (本地文件),
    /// - `http(s)://example.com/file` (网络URL),
    /// - `base64://<BASE64编码的文件数据>` (Base64编码内容)
    pub file_uri: String,
    /// 文件名称
    pub file_name: String,
}

/// 上传私聊文件的响应数据
#[derive(Deserialize, Debug)]
pub struct UploadPrivateFileResponse {
    /// 文件ID
    pub file_id: String,
}

/// 上传群文件的请求参数
#[derive(Serialize)]
pub struct UploadGroupFileRequest {
    /// 文件要上传到的目标群组的群号
    pub group_id: i64,
    /// 目标文件夹ID
    pub parent_folder_id: String,
    /// 文件的统一资源标识符 (URI)
    /// 支持:
    /// - `file:///path/to/file` (本地文件),
    /// - `http(s)://example.com/file` (网络URL),
    /// - `base64://<BASE64编码的文件数据>` (Base64编码内容)
    pub file_uri: String,
    /// 文件名称
    pub file_name: String,
}

/// 上传群文件的响应数据
#[derive(Deserialize, Debug)]
pub struct UploadGroupFileResponse {
    /// 上传成功后，文件在服务器上的唯一ID
    pub file_id: String,
}

/// 获取私聊文件下载链接的请求参数
#[derive(Serialize)]
pub struct GetPrivateFileDownloadUrlRequest {
    /// 文件所属好友的QQ号
    pub user_id: i64,
    /// 文件ID
    pub file_id: String,
    /// 文件的 TriSHA1 哈希值
    pub file_hash: String,
}

/// 获取私聊文件下载链接的响应数据
#[derive(Deserialize, Debug)]
pub struct GetPrivateFileDownloadUrlResponse {
    /// 文件的可直接访问的下载链接
    pub download_url: String,
}

/// 获取群文件下载链接的请求参数
#[derive(Serialize)]
pub struct GetGroupFileDownloadUrlRequest {
    /// 文件所属群组的群号
    pub group_id: i64,
    /// 要获取下载链接的文件的ID
    pub file_id: String,
}

/// 获取群文件下载链接的响应数据
#[derive(Deserialize, Debug)]
pub struct GetGroupFileDownloadUrlResponse {
    /// 文件的可直接访问的下载链接
    pub download_url: String,
}

/// 获取群文件列表的请求参数
#[derive(Serialize)]
pub struct GetGroupFilesRequest {
    /// 要查询的群组的群号
    pub group_id: i64,
    /// 父文件夹ID
    pub parent_folder_id: String,
}

/// 获取群文件列表的响应数据
#[derive(Deserialize, Debug)]
pub struct GetGroupFilesResponse {
    /// 获取到的文件列表
    pub files: Vec<GroupFile>,
    /// 获取到的文件夹列表
    pub folder: Vec<GroupFolder>,
}

/// 移动群文件的请求参数
#[derive(Serialize)]
pub struct MoveGroupFileRequest {
    /// 文件所属群组的群号
    pub group_id: i64,
    /// 要移动的文件的ID
    pub file_id: String,
    /// 文件所在的文件夹 ID
    pub parent_folder_id: String,
    /// 目标文件夹ID
    pub target_folder_id: String,
}

/// 重命名群文件的请求参数
#[derive(Serialize)]
pub struct RenameGroupFileRequest {
    /// 文件所属群组的群号
    pub group_id: i64,
    /// 要重命名的文件的ID
    pub file_id: String,
    /// 文件所在的文件夹 ID
    pub parent_folder_id: String,
    /// 文件的新名称
    pub new_file_name: String,
}

/// 删除群文件的请求参数
#[derive(Serialize)]
pub struct DeleteGroupFileRequest {
    /// 文件所属群组的群号
    pub group_id: i64,
    /// 要删除的文件的ID
    pub file_id: String,
}

/// 创建群文件夹的请求参数
#[derive(Serialize)]
pub struct CreateGroupFolderRequest {
    /// 要在其中创建文件夹的群组的群号
    pub group_id: i64,
    /// 新文件夹的名称
    pub folder_name: String,
}

/// 创建群文件夹的响应数据
#[derive(Deserialize, Debug)]
pub struct CreateGroupFolderResponse {
    /// 创建成功后，新文件夹的唯一ID
    pub folder_id: String,
}

/// 重命名群文件夹的请求参数
#[derive(Serialize)]
pub struct RenameGroupFolderRequest {
    /// 文件夹所属群组的群号
    pub group_id: i64,
    /// 要重命名的文件夹的ID
    pub folder_id: String,
    /// 文件夹的新名称
    pub new_folder_name: String,
}

/// 删除群文件夹的请求参数
#[derive(Serialize)]
pub struct DeleteGroupFolderRequest {
    /// 文件夹所属群组的群号
    pub group_id: i64,
    /// 要删除的文件夹的ID
    pub folder_id: String,
}

impl MilkyClient {
    /// 上传私聊文件到指定好友
    ///
    /// # 参数
    /// * `user_id`: 接收文件的好友QQ号
    /// * `file_uri`: 文件的URI，支持 `file://`, `http(s)://`, `base64://` 格式
    /// * `file_name`: 文件名称
    ///
    /// # 返回
    /// 成功则返回包含文件ID的 [`UploadPrivateFileResponse`]
    pub async fn upload_private_file(
        &self,
        user_id: i64,
        file_uri: String,
        file_name: String,
    ) -> Result<UploadPrivateFileResponse> {
        let params = UploadPrivateFileRequest {
            user_id,
            file_uri,
            file_name,
        };
        self.send_request("upload_private_file", params).await
    }

    /// 上传文件到指定群组
    ///
    /// # 参数
    /// * `group_id`: 文件要上传到的目标群组的群号
    /// * `parent_folder_id`: 目标文件夹 ID
    /// * `file_uri`: 文件的URI，格式同上
    /// * `file_name`: 文件名称
    ///
    /// # 返回
    /// 成功则返回包含文件ID的 [`UploadGroupFileResponse`]
    pub async fn upload_group_file(
        &self,
        group_id: i64,
        parent_folder_id: Option<String>,
        file_uri: String,
        file_name: String,
    ) -> Result<UploadGroupFileResponse> {
        let parent_folder_id = parent_folder_id.unwrap_or("/".to_string());
        let params = UploadGroupFileRequest {
            parent_folder_id,
            group_id,
            file_uri,
            file_name,
        };
        self.send_request("upload_group_file", params).await
    }

    /// 获取指定私聊文件的下载链接
    ///
    /// # 参数
    /// * `user_id`: 文件所属好友的QQ号
    /// * `file_id`: 要获取下载链接的文件的ID
    /// * `file_hash`: 文件的 TriSHA1 哈希值
    ///
    /// # 返回
    /// 成功则返回包含下载链接的 [`GetPrivateFileDownloadUrlResponse`]
    pub async fn get_private_file_download_url(
        &self,
        user_id: i64,
        file_id: String,
        file_hash: String,
    ) -> Result<GetPrivateFileDownloadUrlResponse> {
        let params = GetPrivateFileDownloadUrlRequest {
            user_id,
            file_id,
            file_hash,
        };
        self.send_request("get_private_file_download_url", params)
            .await
    }

    /// 获取指定群文件的下载链接
    ///
    /// # 参数
    /// * `group_id`: 文件所属群组的群号
    /// * `file_id`: 要获取下载链接的文件的ID
    ///
    /// # 返回
    /// 成功则返回包含下载链接的 [`GetGroupFileDownloadUrlResponse`]
    pub async fn get_group_file_download_url(
        &self,
        group_id: i64,
        file_id: String,
    ) -> Result<GetGroupFileDownloadUrlResponse> {
        let params = GetGroupFileDownloadUrlRequest { group_id, file_id };
        self.send_request("get_group_file_download_url", params)
            .await
    }

    /// 获取指定群组的文件和文件夹列表
    ///
    /// # 参数
    /// * `group_id`: 要查询的群组的群号
    /// * `parent_folder_id`: 父文件夹ID若为 `None`，则获取根目录内容。
    ///
    /// # 返回
    /// 成功则返回包含文件和文件夹列表的 [`GetGroupFilesResponse`]
    pub async fn get_group_files(
        &self,
        group_id: i64,
        parent_folder_id: Option<String>,
    ) -> Result<GetGroupFilesResponse> {
        let parent_folder_id = parent_folder_id.unwrap_or("/".to_string());
        let params = GetGroupFilesRequest {
            group_id,
            parent_folder_id,
        };
        self.send_request("get_group_files", params).await
    }

    /// 移动群文件到指定文件夹
    ///
    /// # 参数
    /// * `group_id`: 文件所属群组的群号
    /// * `file_id`: 要移动的文件的ID
    /// * `parent_folder_id`: 文件所在的文件夹 ID
    /// * `target_folder_id`: 目标文件夹ID若为 `None`，则移动到根目录。
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`
    pub async fn move_group_file(
        &self,
        group_id: i64,
        file_id: String,
        parent_folder_id: Option<String>,
        target_folder_id: Option<String>,
    ) -> Result<()> {
        let parent_folder_id = parent_folder_id.unwrap_or("/".to_string());
        let target_folder_id = target_folder_id.unwrap_or("/".to_string());
        let params = MoveGroupFileRequest {
            group_id,
            file_id,
            parent_folder_id,
            target_folder_id,
        };
        self.send_request("move_group_file", params).await
    }

    /// 重命名群文件夹
    ///
    /// # 参数
    /// * `group_id`: 文件所属群组的群号
    /// * `file_id`: 要重命名的文件的ID
    /// * `parent_folder_id`: 文件所在的文件夹 ID
    /// * `new_file_name`: 文件的新名称
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`
    pub async fn rename_group_file(
        &self,
        group_id: i64,
        file_id: String,
        parent_folder_id: Option<String>,
        new_file_name: String,
    ) -> Result<()> {
        let parent_folder_id = parent_folder_id.unwrap_or("/".to_string());
        let params = RenameGroupFileRequest {
            group_id,
            file_id,
            parent_folder_id,
            new_file_name,
        };
        self.send_request("rename_group_file", params).await
    }

    /// 删除指定的群文件
    ///
    /// # 参数
    /// * `group_id`: 文件所属群组的群号
    /// * `file_id`: 要删除的文件的ID
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`
    pub async fn delete_group_file(&self, group_id: i64, file_id: String) -> Result<()> {
        let params = DeleteGroupFileRequest { group_id, file_id };
        self.send_request("delete_group_file", params).await
    }

    /// 在指定群组中创建新的文件夹
    ///
    /// # 参数
    /// * `group_id`: 要在其中创建文件夹的群组的群号
    /// * `folder_name`: 新文件夹的名称
    ///
    /// # 返回
    /// 成功则返回包含新文件夹ID的 [`CreateGroupFolderResponse`]
    pub async fn create_group_folder(
        &self,
        group_id: i64,
        folder_name: String,
    ) -> Result<CreateGroupFolderResponse> {
        let params = CreateGroupFolderRequest {
            group_id,
            folder_name,
        };
        self.send_request("create_group_folder", params).await
    }

    /// 重命名群文件夹
    ///
    /// # 参数
    /// * `group_id`: 文件夹所属群组的群号
    /// * `folder_id`: 要重命名的文件夹的ID
    /// * `new_folder_name`: 文件夹的新名称
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`
    pub async fn rename_group_folder(
        &self,
        group_id: i64,
        folder_id: String,
        new_folder_name: String,
    ) -> Result<()> {
        let params = RenameGroupFolderRequest {
            group_id,
            folder_id,
            new_folder_name,
        };
        self.send_request("rename_group_folder", params).await
    }

    /// 删除指定的群文件夹
    ///
    /// # 参数
    /// * `group_id`: 文件夹所属群组的群号
    /// * `folder_id`: 要删除的文件夹的ID
    ///
    /// # 返回
    /// 成功则返回 `Ok(())`
    pub async fn delete_group_folder(&self, group_id: i64, folder_id: String) -> Result<()> {
        let params = DeleteGroupFolderRequest {
            group_id,
            folder_id,
        };
        self.send_request("delete_group_folder", params).await
    }
}
