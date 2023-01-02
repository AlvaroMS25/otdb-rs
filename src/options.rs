use reqwest::RequestBuilder;
use std::cmp::{PartialEq, Eq};
use serde::de::Deserialize;
use serde::Deserializer;
use crate::model::base64_string;

#[derive(Debug, Clone, Default)]
pub struct Options {
    question_number: Option<u8>,
    category: Option<Category>,
    difficulty: Option<Difficulty>,
    kind: Option<Kind>
}

impl Options {
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



#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Kind {
    Any,
    TrueOrFalse,
    MultipleChoice
}

impl Kind {
    pub(crate) fn prepare(self, builder: RequestBuilder) -> RequestBuilder {
        match self {
            Self::TrueOrFalse => builder.query(&[("type", "boolean")]),
            Self::MultipleChoice => builder.query(&[("type", "multiple")]),
            Self::Any => builder
        }
    }
}

impl<'de> Deserialize<'de> for Kind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        match base64_string(deserializer)?.as_str() {
            "boolean" => Ok(Kind::TrueOrFalse),
            "multiple" => Ok(Kind::MultipleChoice),
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Difficulty {
    Any,
    Easy,
    Medium,
    Hard
}

impl Difficulty {
    pub(crate) fn prepare(self, builder: RequestBuilder) -> RequestBuilder {
        match self {
            Self::Easy => builder.query(&[("difficulty", "easy")]),
            Self::Medium => builder.query(&[("difficulty", "medium")]),
            Self::Hard => builder.query(&[("difficulty", "hard")]),
            Self::Any => builder
        }
    }
}

impl<'de> Deserialize<'de> for Difficulty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        match base64_string(deserializer)?.as_str() {
            "easy" => Ok(Difficulty::Easy),
            "medium" => Ok(Difficulty::Medium),
            "hard" => Ok(Difficulty::Hard),
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[repr(u8)]
pub enum Category {
    Any = 0,
    GeneralKnowledge = 9,
    Books = 10,
    Film = 11,
    Music = 12,
    MusicalAndTheatres = 13,
    Television = 14,
    VideoGames = 15,
    BoardGames = 16,
    ScienceAndNature = 17,
    Computers = 18,
    Mathematics = 19,
    Mythology = 20,
    Sports = 21,
    Geography = 22,
    History = 23,
    Politics = 24,
    Art = 25,
    Celebrities = 26,
    Animals = 27,
    Vehicles = 28,
    Comics = 29,
    Gadgets = 30,
    JapaneseAnimeAndManga = 31,
    CartoonAndAnimations = 32
}

impl Category {
    pub(crate) fn prepare(self, builder: RequestBuilder) -> RequestBuilder {
        let id = self as u8;
        if id == 0 {
            builder
        } else {
            builder.query(&[("category", id)])
        }
    }
}

impl<'de> Deserialize<'de> for Category {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let mut cat = base64_string(deserializer)?.replace(" ", "");

        if cat.contains(":") {
            let (_, rest) = cat.rsplit_once(":").expect("Invalid option");
            cat = rest.to_string();
        }

        if cat.contains("&") {
            cat = cat.replace("&", "And");
        }

        for i in 9..=32 {
            // SAFETY: All the numbers contained in the loop are valid category variants,
            // so transmuting them is safe.
            let category = unsafe { std::mem::transmute::<u8, Category>(i) };

            if format!("{category:?}") == cat {
                return Ok(category);
            }
        }

        Ok(Category::Any)
    }
}
