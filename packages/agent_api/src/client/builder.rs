use std::{marker::PhantomData, str::FromStr};

use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    ClientBuilder as ReqwestBuilder, Url,
};

use crate::{ApiClient, Error, Result, DEFAULT_API_BASE_URL, DEFAULT_CLIENT_USER_AGENT};

use super::{ApiClientKind, Authorized, Guest};

#[must_use]
pub struct ApiClientBuilder<K: ApiClientKind = Guest> {
    api_url: String,
    secret: Option<Result<HeaderValue>>,
    reqw_builder: ReqwestBuilder,
    __phantom: PhantomData<K>,
}

impl<K: ApiClientKind> ApiClientBuilder<K> {
    pub fn with_base_url<S: Into<String>>(api_url: S) -> Self {
        Self {
            api_url: api_url.into(),
            secret: None,
            reqw_builder: ReqwestBuilder::new().user_agent(DEFAULT_CLIENT_USER_AGENT),
            __phantom: PhantomData,
        }
    }

    pub fn user_agent<V: AsRef<str>>(mut self, value: V) -> Self {
        self.reqw_builder = self.reqw_builder.user_agent(value.as_ref());
        self
    }

    pub fn build(mut self) -> Result<ApiClient<K>> {
        if let Some(secret_header) = self.secret {
            let mut headers = HeaderMap::with_capacity(1);
            headers.insert(header::AUTHORIZATION, secret_header?);
            self.reqw_builder = self.reqw_builder.default_headers(headers);
        }

        let api_url = Url::from_str(&self.api_url).map_err(Error::builder)?;
        let client = self.reqw_builder.build().map_err(Error::builder)?;

        Ok(ApiClient {
            api_url,
            client,
            __phantom: PhantomData,
        })
    }
}

impl ApiClientBuilder<Guest> {
    pub fn secret<S: AsRef<str>>(self, secret: S) -> ApiClientBuilder<Authorized> {
        let secret = format!("agent-key {}", secret.as_ref());
        let bytes = secret.into_bytes();
        let secret = HeaderValue::from_bytes(&bytes).map_err(Error::builder);

        ApiClientBuilder {
            api_url: self.api_url,
            secret: Some(secret),
            reqw_builder: self.reqw_builder,
            __phantom: PhantomData,
        }
    }
}

impl Default for ApiClientBuilder<Guest> {
    #[inline]
    fn default() -> Self {
        Self::with_base_url(DEFAULT_API_BASE_URL)
    }
}
