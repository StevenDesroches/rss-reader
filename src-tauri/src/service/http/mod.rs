use crate::error;
use crate::shared::errors::*;
use crate::shared::types::Url;
use tauri_plugin_http::reqwest;

pub trait IHttp {
    async fn fetch(&self, url: &Url) -> Result<String>;
}
pub struct HttpReqwest {}

impl IHttp for HttpReqwest {
    async fn fetch(&self, url: &Url) -> Result<String> {
        let response = reqwest::get(url)
            .await
            .map_err(|e| error!(ErrorType::ReqwestBadUrl(e.to_string())))?;

        // if !response.status().is_success() {//might be good to check the status before going further
        //     return Err(Error::ReqwestBadStatus(response.status().as_u16()));
        // }

        let content = response
            .text()
            .await
            .map_err(|e| error!(ErrorType::ReqwestBadResponse(e.to_string())))?;
        Ok(content)
    }
}
