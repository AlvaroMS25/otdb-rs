use reqwest::{RequestBuilder, Client as HttpClient};
use crate::{options::*, OTDBResult};
use std::{marker::PhantomData, future::Future, task::{Context, Poll}, pin::Pin, marker::Unpin, collections::HashSet, fmt::{Debug, Formatter, Result as FmtResult}, cmp::PartialEq};
use crate::model::BaseResponse;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Default)]
pub struct EndPointOptions {
    question_number: Option<u8>,
    category: Option<Category>,
    difficulty: Option<Difficulty>,
    kind: Option<Kind>
}

impl EndPointOptions {
    fn prepare(&mut self, mut builder: RequestBuilder) -> RequestBuilder {
        if let Some(n) = self.question_number.take() {
            builder = builder.query(&[("amount", n)]);
        }
        if let Some(c) = self.category.take() {
            builder = c.prepare(builder);
        }
        if let Some(d) = self.difficulty.take() {
            builder = d.prepare(builder);
        }
        if let Some(k) = self.kind.take() {
            builder = k.prepare(builder);
        }

        builder
    }
}

pub struct Request<'a, T: 'static + DeserializeOwned + Unpin> {
    client: &'a HttpClient,
    token: &'a Option<String>,
    endpoint: &'static str,
    pub options: EndPointOptions,
    future: Option<Pin<Box<dyn Future<Output = OTDBResult<T>>>>>
}

unsafe impl<T: DeserializeOwned + Unpin> Send for Request<'_, T> {}

impl<'a, T: DeserializeOwned + Unpin> Request<'a, T> {
    pub(crate) fn default(client: &'a HttpClient, token: &'a Option<String>, endpoint: &'static str) -> Self {
        Self {
            client,
            token,
            endpoint,
            options: Default::default(),
            future: None
        }
    }

    pub(crate) fn new(client: &'a HttpClient, token: &'a Option<String>, endpoint: &'static str) -> Self {
        let mut request = Self::default(client, token, endpoint);
        request.question_number(10);

        request
    }

    pub fn question_number(&mut self, number: u8) -> &mut Self {
        assert!(number <= 50);
        self.options.question_number = Some(number);
        self
    }

    pub fn category(&mut self, category: Category) -> &mut Self {
        self.options.category = Some(category);
        self
    }

    pub fn difficulty(&mut self, difficulty: Difficulty) -> &mut Self {
        self.options.difficulty = Some(difficulty);
        self
    }

    pub fn kind(&mut self, kind: Kind) -> &mut Self {
        self.options.kind = Some(kind);
        self
    }

    pub fn into_owned(self) -> OwnedRequest<T> {
        OwnedRequest {
            client: self.client.clone(),
            token: self.token.clone(),
            endpoint: self.endpoint,
            options: self.options,
            future: self.future
        }
    }

    pub fn prepare(&mut self, mut request: RequestBuilder) -> RequestBuilder {
        if let Some(t) = self.token {
            request = request.query(&[("token", t)]);
        }
        self.options.prepare(request)
    }

    fn _prepare_future(&mut self) {
        if self.future.is_none() {
            let mut request = self.client.get(self.endpoint);
            request = self.prepare(request);
            self.future = Some(Box::pin(crate::make_request(request)));
        }
    }
}

impl<T: DeserializeOwned + Unpin> Future for Request<'_, T> {
    type Output = OTDBResult<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.as_mut()._prepare_future();
        if let Poll::Ready(res) = self.as_mut().future.as_mut().unwrap().as_mut().poll(cx) {
            Poll::Ready(res)
        } else {
            Poll::Pending
        }
    }
}

impl<T: DeserializeOwned + Unpin> Debug for Request<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Request")
            .field("token", &self.token)
            .field("endpoint", &self.endpoint)
            .field("options", &self.options)
            .finish()
    }
}

pub struct OwnedRequest<T: 'static + DeserializeOwned + Unpin> {
    client: HttpClient,
    token: Option<String>,
    endpoint: &'static str,
    options: EndPointOptions,
    future: Option<Pin<Box<dyn Future<Output = OTDBResult<T>>>>>
}

unsafe impl<T: DeserializeOwned + Unpin> Send for OwnedRequest<T> {}

impl<T: DeserializeOwned + Unpin> OwnedRequest<T> {
    pub fn prepare(&mut self, mut request: RequestBuilder) -> RequestBuilder {
        if let Some(t) = &self.token {
            request = request.query(&[("token", t)]);
        }
        self.options.prepare(request)
    }

    fn _prepare_future(&mut self) {
        if self.future.is_none() {
            let mut request = self.client.get(self.endpoint);
            request = self.prepare(request);
            self.future = Some(Box::pin(crate::make_request(request)));
        }
    }
}

impl<T: 'static + DeserializeOwned + Unpin> Future for OwnedRequest<T> {
    type Output = OTDBResult<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.as_mut()._prepare_future();
        if let Poll::Ready(res) = self.as_mut().future.as_mut().unwrap().as_mut().poll(cx) {
            Poll::Ready(res)
        } else {
            Poll::Pending
        }
    }
}

impl<T: DeserializeOwned + Unpin> Debug for OwnedRequest<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("OwnedRequest")
            .field("token", &self.token)
            .field("endpoint", &self.endpoint)
            .field("options", &self.options)
            .finish()
    }
}
