use crate::{request::{Request, OwnedRequest}, OTDBResult, model::*};
use reqwest::Client as HttpClient;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use serde::de::DeserializeOwned;

#[derive(Clone)]
pub struct Client {
    token: Option<String>,
    client: HttpClient
}

impl Client {
    pub fn create_http_client() -> HttpClient {
        HttpClient::builder()
            .user_agent("Rust-OTDB-wrapper")
            .build()
            .expect("Failed to build client")
    }

    pub fn new() -> Self {
        Self {
            token: Default::default(),
            client: Self::create_http_client()
        }
    }

    pub fn new_with_http(client: HttpClient) -> Self {
        Self {
            token: Default::default(),
            client
        }
    }

    pub fn new_with_token(token: impl ToString) -> Self {
        let mut s = Self::new();
        s.set_token(token);
        s
    }

    pub fn new_with_http_and_token(client: HttpClient, token: impl ToString) -> Self {
        let mut s = Self::new_with_http(client);
        s.set_token(token);
        s
    }

    pub fn set_token(&mut self, token: impl ToString) {
        self.token = Some(token.to_string());
    }

    pub async fn generate_token(&self) -> OTDBResult<String> {
        use crate::model::TokenRequestResponse;

        Ok(Request::<TokenRequestResponse>::new(
            &self.client,
            &self.token,
            "https://opentdb.com/api_token.php?command=request"
        ).await?.token)
    }

    pub fn trivia_request(&self) -> Request<BaseResponse<Vec<Trivia>>> {
        Request::new(
            &self.client,
            &self.token,
            "https://opentdb.com/api.php?encode=base64"
        )
    }

    pub fn new_request<T: DeserializeOwned + Unpin>(&self, ep: impl ToString) -> Request<T> {
        Request::new(
            &self.client,
            &self.token,
            Box::leak(ep.to_string().into_boxed_str())
        )
    }

    pub async fn reset_token(&self) -> OTDBResult<String> {
        if self.token.is_some() {
            Ok(Request::<ResetToken>::new(
                &self.client,
                &self.token,
                "https://opentdb.com/api_token.php?command=reset"
            ).await?.token)
        } else {
            self.generate_token().await
        }
    }
}

impl Debug for Client {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Client")
            .field("token", &self.token as &dyn Debug)
            .finish()
    }
}
