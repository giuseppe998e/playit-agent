use reqwest::{Client as ReqwestClient, Method, Url};
use serde::Deserialize;
use serde_json::Value;

use crate::{Error, Result};

mod builder;
pub use builder::ApiClientBuilder;

#[derive(Clone)]
pub struct ApiClient {
    api_url: Url,
    client: ReqwestClient,
}

impl ApiClient {
    #[inline]
    pub(crate) async fn post<T>(&self, endpoint: &str, payload: &Value) -> Result<T>
    where
        for<'d> T: Deserialize<'d>,
    {
        self.request(Method::POST, endpoint, payload).await
    }

    async fn request<T>(&self, method: Method, endpoint: &str, payload: &Value) -> Result<T>
    where
        for<'d> T: Deserialize<'d>,
    {
        let url = self.api_url.join(endpoint).map_err(Error::request)?;
        let response = self
            .client
            .request(method, url)
            .json(payload)
            .send()
            .await
            .map_err(Error::request)?;

        let response_json = {
            let response_bytes = response.bytes().await.map_err(Error::response)?;
            serde_json::from_slice::<'_, ApiResponse<T>>(&response_bytes).map_err(|_| {
                let stringify = String::from_utf8_lossy(&response_bytes);
                Error::unexpected_data(stringify)
            })?
        };

        match response_json {
            ApiResponse::Ok(obj) => Ok(obj),
            ApiResponse::Error { code, message } => Err(Error::server_status(code, message)),
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ApiResponse<T> {
    Ok(T),
    Error { code: u16, message: String },
}
