use crate::endpoints::EndPointOptions;
use serde::de::DeserializeOwned;
use reqwest::{Client, RequestBuilder};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;
use crate::options::*;
use crate::OTDBResult;

pub struct Request<'a, T> {
    client: &'a Client,
    token: &'a Option<String>,
    endpoint: &'static str,
    options: EndPointOptions,
    marker: PhantomData<T>
}

impl<'a, T: DeserializeOwned> Request<'a, T> {
    pub fn new(client: &'a Client, token: &'a Option<String>, endpoint: &'static str) -> Self {
        let mut this = Self {
            client,
            token,
            endpoint,
            options: Default::default(),
            marker: PhantomData
        };

        this.question_number(10);
        this
    }

    pub fn into_owned(self) -> OwnedRequest<T> {
        OwnedRequest {
            client: self.client.clone(),
            token: self.token.clone(),
            endpoint: self.endpoint.to_string(),
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

    pub async fn send(mut self) -> OTDBResult<T> {
        self.prepare(self.client.get(self.endpoint))
            .send()
            .await?
            .json()
            .await
            .map_err(From::from)
    }
}

impl<T> Deref for Request<'_, T> {
    type Target = EndPointOptions;

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

pub struct OwnedRequest<T> {
    client: Client,
    token: Option<String>,
    endpoint: String,
    options: EndPointOptions,
    marker: PhantomData<T>
}

unsafe impl<T: DeserializeOwned> Send for OwnedRequest<T> {}

impl<T: DeserializeOwned> OwnedRequest<T> {
    pub fn new(client: &Client, token: &Option<String>, endpoint: String) -> Self {
        let mut this = Self {
            client: client.clone(),
            token: token.clone(),
            endpoint,
            options: Default::default(),
            marker: PhantomData
        };

        this.question_number(10);
        this
    }

    pub(crate) fn prepare(&mut self, mut request: RequestBuilder) -> RequestBuilder {
        if let Some(t) = &self.token {
            request = request.query(&[("token", t)]);
        }
        self.options.prepare(request)
    }

    pub async fn send(mut self) -> OTDBResult<T> {
        self.prepare(self.client.get(&self.endpoint))
            .send()
            .await?
            .json()
            .await
            .map_err(From::from)
    }
}

impl<T: DeserializeOwned> Deref for OwnedRequest<T> {
    type Target = EndPointOptions;

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
