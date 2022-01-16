use reqwest::RequestBuilder;
use std::cmp::{PartialEq, Eq};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Kind {
    Any,
    TrueOrFalse,
    MultipleChoice
}

impl Kind {
    pub(crate) fn prepare(self, mut builder: RequestBuilder) -> RequestBuilder {
        match self {
            Self::TrueOrFalse => builder.query(&[("type", "boolean")]),
            Self::MultipleChoice => builder.query(&[("type", "multiple")]),
            Self::Any => builder
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
    pub(crate) fn prepare(self, mut builder: RequestBuilder) -> RequestBuilder {
        match self {
            Self::Easy => builder.query(&[("difficulty", "easy")]),
            Self::Medium => builder.query(&[("difficulty", "medium")]),
            Self::Hard => builder.query(&[("difficulty", "hard")]),
            Self::Any => builder
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Category {
    Any = 0,
    GeneralKnowledge = 9,
    Books = 10,
    Films = 11,
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
    AnimeAndManga = 31,
    CartoonAndAnimations = 32
}

impl Category {
    pub(crate) fn prepare(self, mut builder: RequestBuilder) -> RequestBuilder {
        let id = self as u8;
        if id == 0 {
            builder
        } else {
            builder.query(&[("category", id)])
        }
    }
}
