pub mod builder;

use std::{fmt, marker::PhantomData};

use reqwest::{Client as ReqwestClient, Method, Url};
use serde::Deserialize;
use serde_json::{Map, Value};

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
    // FIXME A different server response format can simplify client code
    pub(crate) async fn post<O>(&self, endpoint: &str, payload: &Value) -> Result<O>
    where
        for<'d> O: Deserialize<'d>,
    {
        let url = self.api_url.join(endpoint).map_err(Error::request)?;
        let response_bytes = {
            let response = self
                .client
                .request(Method::POST, url)
                .json(payload)
                .send()
                .await
                .map_err(Error::request)?;

            response.bytes().await.map_err(Error::response)?
        };

        let mut json_map = serde_json::from_slice::<'_, Map<String, Value>>(&response_bytes)
            .map_err(|_| {
                let stringify = String::from_utf8_lossy(&response_bytes);
                Error::unexpected_data(stringify)
            })?;

        let kind = json_map
            .remove("type")
            .and_then(|val| match val {
                Value::String(str) => Some(str),
                _ => None,
            })
            .ok_or_else(|| {
                let stringify = String::from_utf8_lossy(&response_bytes);
                Error::unexpected_data(stringify)
            })?;

        match kind.as_str() {
            "error" => {
                let ApiError { code, message } =
                    serde_json::from_value::<ApiError>(json_map.into()).map_err(|_| {
                        let stringify = String::from_utf8_lossy(&response_bytes);
                        Error::unexpected_data(stringify)
                    })?;

                Err(Error::server_status(code, message))
            }
            _ => serde_json::from_value::<O>(json_map.into()).map_err(|_| {
                let stringify = String::from_utf8_lossy(&response_bytes);
                Error::unexpected_data(stringify)
            }),
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
struct ApiError {
    code: u16,
    message: String,
}
