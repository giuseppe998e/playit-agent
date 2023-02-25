pub mod builder;

use std::{fmt, marker::PhantomData};

use reqwest::{Client as ReqwestClient, Method, Url};
use serde::Deserialize;
use serde_json::Value;

use crate::{Error, Result};

pub trait PlayItClientKind {}

pub struct Guest;
impl PlayItClientKind for Guest {}

pub struct Authorized;
impl PlayItClientKind for Authorized {}

#[derive(Clone)]
pub struct PlayItClient<K: PlayItClientKind = Guest> {
    api_url: Url,
    client: ReqwestClient,
    __phantom: PhantomData<K>,
}

impl<K: PlayItClientKind> PlayItClient<K> {
    pub(crate) async fn post<O>(&self, endpoint: &str, payload: &Value) -> Result<O>
    where
        for<'d> O: Deserialize<'d>,
    {
        let url = self.api_url.join(endpoint).map_err(Error::request)?;
        let response = self
            .client
            .request(Method::POST, url)
            .json(payload)
            .send()
            .await
            .map_err(Error::request)?;

        let response_json = {
            let response_bytes = response.bytes().await.map_err(Error::response)?;
            serde_json::from_slice::<'_, ApiResponse<O>>(&response_bytes).map_err(|_| {
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

impl fmt::Debug for PlayItClient<Guest> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PlayItClient")
            .field("api_url", &self.api_url)
            .field("kind", &"guest")
            .finish()
    }
}

impl fmt::Debug for PlayItClient<Authorized> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PlayItClient")
            .field("api_url", &self.api_url)
            .field("kind", &"authorized")
            .finish()
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ApiResponse<T> {
    Ok(T),
    Error { code: u16, message: String },
}
