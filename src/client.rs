use crate::{request::{Request}, model::*};
use reqwest::Client as HttpClient;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use serde::de::DeserializeOwned;
use crate::error::Result;
use crate::options::Category;

/// A client to make requests with.
#[derive(Clone)]
pub struct Client {
    token: Option<String>,
    client: HttpClient
}

impl Client {
    /// Creates a new `Client`.
    pub fn new() -> Self {
        Self {
            token: Default::default(),
            client: HttpClient::builder()
                .user_agent("Otdb-rs")
                .build()
                .expect("Failed to build client")
        }
    }

    /// Sets the provided token to be used with http requests.
    pub fn set_token(&mut self, token: impl ToString) {
        self.token = Some(token.to_string());
    }

    /// Returns the token of the client, if it has one.
    pub fn get_token(&self) -> Option<String> {
        self.token.clone()
    }

    /// Generates a new OTDB token, this allows the client to not receive twice the same question.
    pub async fn generate_token(&self) -> Result<String> {
        Ok(Request::<TokenRequest>::new(
            &self.client,
            &self.token,
            "https://opentdb.com/api_token.php?command=request"
        ).send().await?.token)
    }

    /// Creates a new http request used to retrieve trivia questions, all options can be set before
    /// sending the request.
    ///
    /// # Example
    ///
    /// ```rust
    /// use otdb::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new();
    ///     let mut request = client.trivia();
    ///
    ///     // We can set various request options here.
    ///     request.question_number(10);
    ///
    ///     match request.send().await {
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
        Request::new(
            &self.client,
            &self.token,
            "https://opentdb.com/api.php?encode=base64"
        )
    }

    /// Creates a new http request used to retrieve trivia questions, all options can be set before
    /// sending the request.
    ///
    /// # Example
    ///
    /// ```rust
    /// use otdb::{Category, Client};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new();
    ///     match client.category_details(Category::Animals).send().await {
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
        Request::new(
            &self.client,
            &None,
            format!("https://opentdb.com/api_count.php?category={}", category as u8)
        )
    }


    /// Creates a new http request that fetches the global OTDB API details.
    ///
    /// # Example
    ///
    /// ```rust
    /// use otdb::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new();
    ///     match client.global_details().send().await {
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
        Request::new(
            &self.client,
            &None,
            "https://opentdb.com/api_count_global.php"
        )
    }

    /// Creates a new http request with a custom endpoint and a custom return body.
    ///
    /// # Example
    ///
    /// ```rust
    /// use otdb::Client;
    ///
    /// #[derive(serde::Deserialize)]
    /// struct SuperCoolResponse {
    ///     // ...
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new();
    ///     match client.new_request::<SuperCoolResponse>("<ENDPOINT>").send().await {
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
        Request::new(
            &self.client,
            &self.token,
            endpoint
        )
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
    /// use otdb::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::new();
    ///     client.set_token(client.generate_token().await.unwrap());
    ///     client.reset_token().await.unwrap();
    /// }
    /// ```
    pub async fn reset_token(&mut self) -> Result<String> {
        if self.token.is_some() {
            Ok(Request::<ResetToken>::new(
                &self.client,
                &self.token,
                "https://opentdb.com/api_token.php?command=reset"
            ).send().await?.token)
        } else {
            let token = self.generate_token().await?;
            self.set_token(token.clone());
            Ok(token)
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for Client {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Client")
            .field("token", &self.token)
            .finish()
    }
}
