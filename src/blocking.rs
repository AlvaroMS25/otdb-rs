use tokio::runtime::Runtime;
use crate::client::Client as AsyncClient;
use crate::request::{Request as AsyncRequest, OwnedRequest as AsyncOwnedRequest};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use serde::de::DeserializeOwned;
use crate::error::OTDBResult;
use crate::model::*;
use crate::options::Category;

pub struct Request<'a, T> {
    inner: AsyncRequest<'a, T>,
    rt: &'a Arc<Runtime>
}

impl<T: DeserializeOwned> Request<'_, T> {
    pub fn into_owned(self) -> OwnedRequest<T> {
        OwnedRequest {
            rt: Arc::clone(self.rt),
            inner: self.inner.into_owned()
        }
    }

    pub fn send(self) -> OTDBResult<T> {
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

pub struct OwnedRequest<T> {
    inner: AsyncOwnedRequest<T>,
    rt: Arc<Runtime>
}

impl<T: DeserializeOwned> OwnedRequest<T> {
    pub fn send(self) -> OTDBResult<T> {
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

#[derive(Clone)]
pub struct Client {
    rt: Arc<Runtime>,
    inner: AsyncClient
}

impl Client {
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

    pub fn set_token(&mut self, token: impl ToString) {
        self.inner.set_token(token);
    }

    pub fn generate_token(&self) -> OTDBResult<String> {
        self.rt.block_on(self.inner.generate_token())
    }

    pub fn trivia(&self) -> Request<BaseResponse<Vec<Trivia>>> {
        self.block(self.inner.trivia())
    }

    pub fn category_details(&self, category: Category) -> Request<CategoryDetails> {
        self.block(self.inner.category_details(category))
    }

    pub fn global_details(&self) -> Request<GlobalDetails> {
        self.block(self.inner.global_details())
    }

    pub fn new_request<T: DeserializeOwned>(&self, ep: impl ToString) -> Request<T> {
        self.block(self.inner.new_request(ep))
    }

    pub fn reset_token(&self) -> OTDBResult<String> {
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
