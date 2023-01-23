use tokio::runtime::Runtime;
use crate::client::Client as AsyncClient;
use crate::request::{Request as AsyncRequest, OwnedRequest as AsyncOwnedRequest};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use serde::de::DeserializeOwned;
use crate::error::Result;
use crate::model::*;
use crate::options::Category;

/// A blocking request used to make API calls.
///
/// This struct contains unowned fields and cannot be sent between threads, to do so consider
/// using an [owned request](OwnedRequest), it can be obtained by
/// using [into_owned](Request::into_owned)
pub struct Request<'a, T> {
    inner: AsyncRequest<'a, T>,
    rt: &'a Arc<Runtime>
}

impl<T: DeserializeOwned> Request<'_, T> {
    /// Converts the request into an [owned request](OwnedRequest)
    ///
    /// # Example
    ///
    /// ```rust
    /// use otdb::blocking::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new();
    ///     let owned_request = client.trivia().into_owned();
    ///
    ///     match owned_request.send() {
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
            rt: Arc::clone(self.rt),
            inner: self.inner.into_owned()
        }
    }

    /// Sends the request, returning the proper response or error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use otdb::blocking::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new();
    ///     let request = client.trivia();
    ///
    ///     match request.send() {
    ///         Ok(response) => {
    ///             // ...
    ///         },
    ///         Err(error) => {
    ///             // ...
    ///         }
    ///     }
    /// }
    /// ```
    pub fn send(self) -> Result<T> {
        Self::make_request(self.rt, self.inner.send())
    }

    fn make_request<F: Future>(rt: &Runtime, fut: F) -> F::Output {
        rt.block_on(fut)
    }
}

impl<'a, T> Deref for Request<'a, T> {
    type Target = AsyncRequest<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Request<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// A blocking request used to make API calls.
///
/// Unlike the normal [request](Request), this struct does not contain any unowned field and can be
/// sent between threads.
pub struct OwnedRequest<T> {
    inner: AsyncOwnedRequest<T>,
    rt: Arc<Runtime>
}

impl<T: DeserializeOwned> OwnedRequest<T> {
    /// Sends the request, returning the proper response or error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use otdb::Difficulty;
    /// use otdb::blocking::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new();
    ///     let mut owned_request = client.trivia().into_owned();
    ///     owned_request.difficulty(Difficulty::Easy);
    ///
    ///     match owned_request.send() {
    ///         Ok(response) => {
    ///             // ...
    ///         }
    ///         Err(error) => {
    ///             // ...
    ///         }
    ///     }
    /// }
    /// ```
    pub fn send(self) -> Result<T> {
        Request::<'_, T>::make_request(&self.rt, self.inner.send())
    }
}

impl<T> Deref for OwnedRequest<T> {
    type Target = AsyncOwnedRequest<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for OwnedRequest<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// A blocking client to make requests with.
#[derive(Clone)]
pub struct Client {
    rt: Arc<Runtime>,
    inner: AsyncClient
}

impl Client {
    /// Creates a new `Client`.
    pub fn new() -> Self {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        Self {
            rt: Arc::new(rt),
            inner: AsyncClient::new()
        }
    }

    /// Sets the provided token to be used with http requests.
    pub fn set_token(&mut self, token: impl ToString) {
        self.inner.set_token(token);
    }

    /// Returns the token of the client, if it has one.
    pub fn get_token(&self) -> Option<String> {
        self.inner.get_token()
    }

    /// Generates a new OTDB token, this allows the client to not receive twice the same question.
    pub fn generate_token(&self) -> Result<String> {
        self.rt.block_on(self.inner.generate_token())
    }

    /// Creates a new http request used to retrieve trivia questions, all options can be set before
    /// sending the request.
    ///
    /// # Example
    ///
    /// ```rust
    /// use otdb::blocking::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new();
    ///     let mut request = client.trivia();
    ///
    ///     // We can set various request options here.
    ///     request.question_number(10);
    ///
    ///     match request.send() {
    ///         Ok(response) => {
    ///             // Do something with the response
    ///         },
    ///         Err(error) => {
    ///             // Do something with the error
    ///         }
    ///     }
    /// }
    /// ```
    pub fn trivia(&self) -> Request<BaseResponse<Vec<Trivia>>> {
        self.block(self.inner.trivia())
    }

    /// Creates a new http request used to retrieve trivia questions, all options can be set before
    /// sending the request.
    ///
    /// # Example
    ///
    /// ```rust
    /// use otdb::Category;
    /// use otdb::blocking::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new();
    ///     match client.category_details(Category::Animals).send() {
    ///         Ok(response) => {
    ///             // Do something with the response
    ///         }
    ///         Err(error) => {
    ///             // Do something with the error
    ///         }
    ///     }
    /// }
    /// ```
    pub fn category_details(&self, category: Category) -> Request<CategoryDetails> {
        self.block(self.inner.category_details(category))
    }

    /// Creates a new http request that fetches the global OTDB API details.
    ///
    /// # Example
    ///
    /// ```rust
    /// use otdb::blocking::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new();
    ///     match client.global_details().send() {
    ///         Ok(response) => {
    ///             // Do something with the response
    ///         },
    ///         Err(error) => {
    ///             // Do something with the error
    ///         }
    ///     }
    /// }
    /// ```
    pub fn global_details(&self) -> Request<GlobalDetails> {
        self.block(self.inner.global_details())
    }

    /// Creates a new http request with a custom endpoint and a custom return body.
    ///
    /// # Example
    ///
    /// ```rust
    /// use otdb::blocking::Client;
    ///
    /// #[derive(serde::Deserialize)]
    /// struct SuperCoolResponse {
    ///     // ...
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new();
    ///     match client.new_request::<SuperCoolResponse>("<ENDPOINT>").send() {
    ///         Ok(response) => {
    ///             // Do something with the response
    ///         },
    ///         Err(error) => {
    ///             // Do something with the error
    ///         }
    ///     }
    /// }
    /// ```
    pub fn new_request<T: DeserializeOwned>(&self, endpoint: impl ToString) -> Request<T> {
        self.block(self.inner.new_request(endpoint))
    }

    /// Resets the token the client has, this clears the past memory of the token, and allows the
    /// client to receive all the available questions again. If the client doesn't have a token,
    /// this method will create one and set it.
    ///
    /// This method returns the token used by the client or the generated one in case the client
    /// didn't have one. However, it is **NOT** required to set the token again, because this operation
    /// only resets the token if it was present, it doesn't change. In case it wasn't present it will
    /// also be set in the client.
    ///
    /// # Example
    ///
    /// ```rust
    /// use otdb::blocking::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::new();
    ///     client.set_token(client.generate_token().unwrap());
    ///     client.reset_token().unwrap();
    /// }
    /// ```
    pub fn reset_token(&mut self) -> Result<String> {
        self.rt.block_on(self.inner.reset_token())
    }

    fn block<'a, T>(&'a self, item: AsyncRequest<'a, T>) -> Request<'a, T> {
        Request {
            rt: &self.rt,
            inner: item
        }
    }
}

impl Debug for Client {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&self.inner, f)
    }
}
