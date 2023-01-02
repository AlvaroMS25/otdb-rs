pub mod client;
pub mod error;
pub mod model;
pub mod options;
pub mod request;

#[cfg(test)]
mod tests;

use error::{OTDBResult, HttpError};

pub(crate) async fn make_request<T>(req: reqwest::RequestBuilder) -> OTDBResult<T>
    where
        T: serde::de::DeserializeOwned
{
    let response = req.send().await?;

    match response.status().as_u16() {
        200 => Ok(response.json().await?),
        c if c >= 500 => Err(HttpError::InternalServerError(response.text().await?)),
        _ => Err(HttpError::UnsuccessfulRequest(response.status(), response.text().await?)),
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