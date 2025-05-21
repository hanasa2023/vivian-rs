use crate::client::MilkyClient;
use crate::error::Result;
use serde::Serialize;

#[derive(Serialize)]
pub struct AcceptRequestParams {
    /// 请求ID
    pub request_id: String,
}

#[derive(Serialize)]
pub struct RejectRequestParams {
    /// 请求ID
    pub request_id: String,
    /// 拒绝理由（可选）
    pub reason: Option<String>,
}

impl MilkyClient {
    pub async fn accept_request(&self, request_id: &str) -> Result<()> {
        let params = AcceptRequestParams {
            request_id: request_id.to_string(),
        };
        self.send_request("accept_request", params).await
    }

    pub async fn reject_request(&self, request_id: &str, reason: Option<&str>) -> Result<()> {
        let reason = match reason {
            Some(reason_str) => Some(reason_str.to_string()),
            None => None,
        };
        let params = RejectRequestParams {
            request_id: request_id.to_string(),
            reason,
        };
        self.send_request("reject_request", params).await
    }
}
