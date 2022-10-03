use crate::endpoints::EndPointOptions;
use serde::de::DeserializeOwned;
use reqwest::{Client, RequestBuilder};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::marker::PhantomData;
use crate::options::*;

pub struct Request<'a, T: 'static + DeserializeOwned> {
    client: &'a Client,
    token: &'a Option<String>,
    endpoint: &'static str,
    pub options: EndPointOptions,
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

    pub fn prepare(&mut self, mut request: RequestBuilder) -> RequestBuilder {
        if let Some(t) = self.token {
            request = request.query(&[("token", t)]);
        }
        self.options.prepare(request)
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

pub struct OwnedRequest<T: 'static + DeserializeOwned> {
    client: Client,
    token: Option<String>,
    endpoint: String,
    options: EndPointOptions,
    marker: PhantomData<T>
}

unsafe impl<T: DeserializeOwned> Send for OwnedRequest<T> {}

impl<T: DeserializeOwned> OwnedRequest<T> {
    pub fn new(client: Client, token: Option<String>, endpoint: String) -> Self {
        let mut this = Self {
            client: client.clone(),
            token,
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

    fn _prepare_future(&mut self) {
        if self.future.is_none() {
            let mut request = self.client.get(self.endpoint);
            request = self.prepare(request);
            self.future = Some(Box::pin(crate::make_request(request)));
        }
    }
}

impl<T: DeserializeOwned> std::ops::Deref for OwnedRequest<T> {
    type Target = EndPointOptions;

    fn deref(&self) -> &Self::Target {
        &self.options
    }
}

impl<T: DeserializeOwned> std::ops::DerefMut for OwnedRequest<T> {
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
