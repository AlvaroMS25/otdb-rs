use crate::{request::{Request}, model::*};
use reqwest::Client as HttpClient;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use serde::de::DeserializeOwned;
use crate::error::OTDBResult;
use crate::options::Category;

#[derive(Clone)]
pub struct Client {
    token: Option<String>,
    client: HttpClient
}

impl Client {
    pub fn new() -> Self {
        Self {
            token: Default::default(),
            client: HttpClient::builder()
                .user_agent("Otdb-rs")
                .build()
                .expect("Failed to build client")
        }
    }

    pub fn set_token(&mut self, token: impl ToString) {
        self.token = Some(token.to_string());
    }

    pub async fn generate_token(&self) -> OTDBResult<String> {
        Ok(Request::<TokenRequest>::new(
            &self.client,
            &self.token,
            "https://opentdb.com/api_token.php?command=request"
        ).send().await?.token)
    }

    pub fn trivia_request(&self) -> Request<BaseResponse<Vec<Trivia>>> {
        Request::new(
            &self.client,
            &self.token,
            "https://opentdb.com/api.php?encode=base64"
        )
    }

    pub fn category_details(&self, category: Category) -> Request<CategoryDetails> {
        Request::new(
            &self.client,
            &None,
            format!("https://opentdb.com/api_count.php?category={}", category as u8)
        )
    }

    pub fn global_details(&self) -> Request<GlobalDetails> {
        Request::new(
            &self.client,
            &None,
            "https://opentdb.com/api_count_global.php"
        )
    }

    pub fn new_request<T: DeserializeOwned>(&self, ep: impl ToString) -> Request<T> {
        Request::new(
            &self.client,
            &self.token,
            ep
        )
    }

    pub async fn reset_token(&self) -> OTDBResult<String> {
        if self.token.is_some() {
            Ok(Request::<ResetToken>::new(
                &self.client,
                &self.token,
                "https://opentdb.com/api_token.php?command=reset"
            ).send().await?.token)
        } else {
            self.generate_token().await
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
