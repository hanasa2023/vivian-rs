use crate::client::MilkyClient;
use crate::error::Result;
use crate::types::group::{GroupFile, GroupFolder};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UploadPrivateFileParams {
    /// 好友 QQ 号
    pub user_id: i64,
    /// 文件 URI，支持 `file://` `http(s)://` `base64://` 三种格式
    pub file_uri: String,
}

#[derive(Deserialize, Debug)]
pub struct UploadPrivateFileResponse {
    /// 文件 ID
    pub file_id: String,
}

#[derive(Serialize)]
pub struct UploadGroupFileParams {
    /// 群号
    pub group_id: i64,
    /// 文件 URI，支持 `file://` `http(s)://` `base64://` 三种格式
    pub file_uri: String,
}

#[derive(Deserialize, Debug)]
pub struct UploadGroupFileResponse {
    /// 文件 ID
    pub file_id: String,
}

#[derive(Serialize)]
pub struct GetPrivateFileDownloadUrlParams {
    /// 好友 QQ 号
    pub user_id: i64,
    /// 文件 ID
    pub file_id: String,
}

#[derive(Deserialize, Debug)]
pub struct GetPrivateFileDownloadUrlResponse {
    /// 文件下载链接
    pub download_url: String,
}

#[derive(Serialize)]
pub struct GetGroupFileDownloadUrlParams {
    /// 群号
    pub group_id: i64,
    /// 文件 ID
    pub file_id: String,
}

#[derive(Deserialize, Debug)]
pub struct GetGroupFileDownloadUrlResponse {
    /// 文件下载链接
    pub download_url: String,
}

#[derive(Serialize)]
pub struct GetGroupFilesParams {
    /// 群号
    pub group_id: i64,
    /// 父文件夹 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GetGroupFilesResponse {
    /// 文件列表
    pub files: Vec<GroupFile>,
    /// 文件夹列表
    pub folder: Vec<GroupFolder>,
}

#[derive(Serialize)]
pub struct MoveGroupFileParams {
    /// 群号
    pub group_id: i64,
    /// 文件 ID
    pub file_id: String,
    /// 目标文件夹 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_folder_id: Option<String>,
}

#[derive(Serialize)]
pub struct RenameGroupFileParams {
    /// 群号
    pub group_id: i64,
    /// 文件 ID
    pub file_id: String,
    /// 新文件名称
    pub new_name: String,
}

#[derive(Serialize)]
pub struct DeleteGroupFileParams {
    /// 群号
    pub group_id: i64,
    /// 文件 ID
    pub file_id: String,
}

#[derive(Serialize)]
pub struct CreateGroupFolderParams {
    /// 群号
    pub group_id: i64,
    /// 文件夹名称
    pub folder_name: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateGroupFolderResponse {
    /// 文件夹 ID
    pub folder_id: String,
}

#[derive(Serialize)]
pub struct RenameGroupFolderParams {
    /// 群号
    pub group_id: i64,
    /// 文件夹 ID
    pub folder_id: String,
    /// 新文件夹名
    pub new_name: String,
}

#[derive(Serialize)]
pub struct DeleteGroupFolderParams {
    /// 群号
    pub group_id: i64,
    /// 文件夹 ID
    pub folder_id: String,
}

impl MilkyClient {
    /// 上传私聊文件
    pub async fn upload_private_file(
        &self,
        user_id: i64,
        file_uri: String,
    ) -> Result<UploadPrivateFileResponse> {
        let params = UploadPrivateFileParams { user_id, file_uri };
        self.send_request("upload_private_file", params).await
    }

    /// 上传群文件
    pub async fn upload_group_file(
        &self,
        group_id: i64,
        file_uri: String,
    ) -> Result<UploadGroupFileResponse> {
        let params = UploadGroupFileParams { group_id, file_uri };
        self.send_request("upload_group_file", params).await
    }

    /// 获取私聊文件下载链接
    pub async fn get_private_file_download_url(
        &self,
        user_id: i64,
        file_id: String,
    ) -> Result<GetPrivateFileDownloadUrlResponse> {
        let params = GetPrivateFileDownloadUrlParams { user_id, file_id };
        self.send_request("get_private_file_download_url", params)
            .await
    }

    /// 获取群文件下载链接
    pub async fn get_group_file_download_url(
        &self,
        group_id: i64,
        file_id: String,
    ) -> Result<GetGroupFileDownloadUrlResponse> {
        let params = GetGroupFileDownloadUrlParams { group_id, file_id };
        self.send_request("get_group_file_download_url", params)
            .await
    }

    /// 获取群文件列表
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

    /// 移动群文件
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

    /// 重命名群文件
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

    /// 删除群文件
    pub async fn delete_group_file(&self, group_id: i64, file_id: String) -> Result<()> {
        let params = DeleteGroupFileParams { group_id, file_id };
        self.send_request("delete_group_file", params).await
    }

    /// 创建群文件夹
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

    /// 重命名群文件夹
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

    /// 删除群文件夹
    pub async fn delete_group_folder(&self, group_id: i64, folder_id: String) -> Result<()> {
        let params = DeleteGroupFolderParams {
            group_id,
            folder_id,
        };
        self.send_request("delete_group_folder", params).await
    }
}
