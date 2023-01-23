use serde::de::DeserializeOwned;
use reqwest::{Client, RequestBuilder};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;
use crate::error::{HttpError, Result};
use crate::options::*;

/// A request used to make API calls.
///
/// This struct contains unowned fields and cannot be sent between threads, to do so consider
/// using an [owned request](OwnedRequest), it can be obtained by
/// using [into_owned](Request::into_owned)
pub struct Request<'a, T> {
    client: &'a Client,
    token: &'a Option<String>,
    endpoint: String,
    options: Options,
    marker: PhantomData<T>
}

impl<'a, T: DeserializeOwned> Request<'a, T> {
    pub(crate) fn new(client: &'a Client, token: &'a Option<String>, endpoint: impl ToString) -> Self {
        let mut this = Self {
            client,
            token,
            endpoint: endpoint.to_string(),
            options: Default::default(),
            marker: PhantomData
        };

        this.question_number(10);
        this
    }

    /// Converts the request into an [owned request](OwnedRequest)
    ///
    /// # Example
    ///
    /// ```rust
    /// use otdb::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new();
    ///     let owned_request = client.trivia().into_owned();
    ///
    ///     match owned_request.send().await {
    ///         Ok(response) => {
    ///             // ...
    ///         },
    ///         Err(error) => {
    ///             // ...
    ///         }
    ///     }
    /// }
    /// ```
    pub fn into_owned(self) -> OwnedRequest<T> {
        OwnedRequest {
            client: self.client.clone(),
            token: self.token.clone(),
            endpoint: self.endpoint,
            options: self.options,
            marker: PhantomData
        }
    }

    pub(crate) fn prepare(&mut self, mut request: RequestBuilder) -> RequestBuilder {
        if let Some(t) = self.token {
            request = request.query(&[("token", t)]);
        }
        self.options.prepare(request)
    }

    /// Sends the request, returning the proper response or error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use otdb::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new();
    ///     let request = client.trivia();
    ///
    ///     match request.send().await {
    ///         Ok(response) => {
    ///             // ...
    ///         },
    ///         Err(error) => {
    ///             // ...
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn send(mut self) -> Result<T> {
        Self::make_request(self.prepare(self.client.get(&self.endpoint))).await
    }

    async fn make_request(req: RequestBuilder) -> Result<T>
    where
    {
        let response = req.send().await?;

        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            c if c >= 500 => Err(HttpError::InternalServerError(response.text().await?)),
            _ => Err(HttpError::UnsuccessfulRequest(response.status(), response.text().await?)),
        }
    }
}

impl<T> Deref for Request<'_, T> {
    type Target = Options;

    fn deref(&self) -> &Self::Target {
        &self.options
    }
}

impl<T> DerefMut for Request<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.options
    }
}

impl<T: DeserializeOwned> Debug for Request<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Request")
            .field("token", &self.token)
            .field("endpoint", &self.endpoint)
            .field("options", &self.options)
            .finish()
    }
}

/// A request used to make API calls.
///
/// Unlike the normal [request](Request), this struct does not contain any unowned field and can be
/// sent between threads.
pub struct OwnedRequest<T> {
    client: Client,
    token: Option<String>,
    endpoint: String,
    options: Options,
    marker: PhantomData<T>
}

unsafe impl<T: DeserializeOwned> Send for OwnedRequest<T> {}

impl<T: DeserializeOwned> OwnedRequest<T> {
    pub(crate) fn prepare(&mut self, mut request: RequestBuilder) -> RequestBuilder {
        if let Some(t) = &self.token {
            request = request.query(&[("token", t)]);
        }
        self.options.prepare(request)
    }

    /// Sends the request, returning the proper response or error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use otdb::{Client, Difficulty};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new();
    ///     let mut owned_request = client.trivia().into_owned();
    ///     owned_request.difficulty(Difficulty::Easy);
    ///
    ///     match owned_request.send().await {
    ///         Ok(response) => {
    ///             // ...
    ///         }
    ///         Err(error) => {
    ///             // ...
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn send(mut self) -> Result<T> {
        Request::make_request(self.prepare(self.client.get(&self.endpoint))).await
    }
}

impl<T: DeserializeOwned> Deref for OwnedRequest<T> {
    type Target = Options;

    fn deref(&self) -> &Self::Target {
        &self.options
    }
}

impl<T: DeserializeOwned> DerefMut for OwnedRequest<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.options
    }
}

impl<T: DeserializeOwned> Debug for OwnedRequest<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("OwnedRequest")
            .field("token", &self.token)
            .field("endpoint", &self.endpoint)
            .field("options", &self.options)
            .finish()
    }
}
