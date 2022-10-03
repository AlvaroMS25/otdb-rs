use reqwest::RequestBuilder;
use crate::{options::*, OTDBResult};
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
    pub(crate) fn prepare(&mut self, mut builder: RequestBuilder) -> RequestBuilder {
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

    pub fn question_number(&mut self, number: u8) -> &mut Self {
        assert!(number <= 50);
        self.question_number = Some(number);
        self
    }

    pub fn category(&mut self, category: Category) -> &mut Self {
        self.category = Some(category);
        self
    }

    pub fn difficulty(&mut self, difficulty: Difficulty) -> &mut Self {
        self.difficulty = Some(difficulty);
        self
    }

    pub fn kind(&mut self, kind: Kind) -> &mut Self {
        self.kind = Some(kind);
        self
    }
}

