use std::fmt;

use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client as ReqwestClient, ClientBuilder, IntoUrl, Url,
};

use crate::{error::ErrorKind, request::ApiRequest, Error, Result};

static CLIENT_USER_AGENT: &str = concat!("playit-agent/", env!("CARGO_PKG_VERSION"));

#[derive(Clone)]
pub struct ApiClient {
    client: ReqwestClient,
    base_url: Url,
}

impl ApiClient {
    pub fn new_guest<U: IntoUrl>(api_url: U) -> Result<Self> {
        let client = ClientBuilder::new()
            .user_agent(CLIENT_USER_AGENT)
            .build()
            .map_err(Error::builder)?;

        Self::with_client(api_url, client)
    }

    pub fn new<U, S>(api_url: U, secret: S) -> Result<Self>
    where
        U: IntoUrl,
        S: AsRef<str> + fmt::Display,
    {
        let headers = {
            let auth_value = {
                let string = format!("agent-key {}", secret);
                let bytes = string.into_bytes();
                HeaderValue::from_bytes(&bytes).map_err(Error::builder)?
            };

            let mut headers = HeaderMap::with_capacity(1);
            headers.insert(header::AUTHORIZATION, auth_value);
            headers
        };

        let client = ClientBuilder::new()
            .user_agent(CLIENT_USER_AGENT)
            .default_headers(headers)
            .build()
            .map_err(Error::builder)?;

        Self::with_client(api_url, client)
    }

    pub fn with_client<U: IntoUrl>(api_url: U, client: ReqwestClient) -> Result<Self> {
        let base_url = api_url.into_url().map_err(Error::builder)?;
        Ok(Self { base_url, client })
    }

    pub async fn request<R>(&self, request: R) -> Result<R::Output>
    where
        R: ApiRequest,
    {
        let url = self.base_url.join(R::ENDPOINT).map_err(Error::request)?;
        let response = self
            .client
            .request(R::METHOD, url)
            .json(&request)
            .send()
            .await
            .map_err(Error::request)?;

        let api_result = response
            .json::<ApiResponse<R::Output>>()
            .await
            .map_err(Error::response)?;

        match api_result {
            ApiResponse::Ok(obj) => Ok(obj),
            ApiResponse::Error { code, message } => {
                Err(Error::new(ErrorKind::ServerStatus(code)).with(message))
            }
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum ApiResponse<T> {
    Ok(T),
    Error {
        code: u16,
        message: String,
    },
}
