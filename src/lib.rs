mod client;
pub mod model;
mod endpoints;
mod options;
mod request;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum HttpError {
    Request(reqwest::Error),
    Unsuccessful(reqwest::StatusCode, String),
    InternalServerError(String),
    InvalidOption(String)
}

pub type OTDBResult<T> = Result<T, HttpError>;

impl From<reqwest::Error> for HttpError {
    fn from(e: reqwest::Error) -> Self {
        Self::Request(e)
    }
}

impl std::error::Error for HttpError {}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Request(why) => write!(f, "Reqwest error: {}", why),
            Self::Unsuccessful(code, body) => write!(f, "Unsuccessful response, code: {}, body: {}", code, body),
            Self::InternalServerError(why) => write!(f, "Internal server error: {}", why),
            Self::InvalidOption(why) => write!(f, "Invalid option: {}", why)
        }
    }
}

pub(crate) async fn make_request<T>(req: reqwest::RequestBuilder) -> OTDBResult<T>
    where
        T: serde::de::DeserializeOwned
{
    let response = req.send().await?;

    match response.status().as_u16() {
        c if c >= 500 => Err(HttpError::InternalServerError(response.text().await?)),
        200 => Ok(response.json().await?),
        _ => Err(HttpError::Unsuccessful(response.status(), response.text().await?)),
    }
}

//https://opentdb.com/api_config.php

pub mod prelude {
    pub use crate::{
        options::*,
        request::{OwnedRequest, Request},
        client::*
    };
}