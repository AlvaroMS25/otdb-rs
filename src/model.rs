use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt::Formatter;
use serde::de::{MapAccess, Visitor};
use crate::options::{Category, Difficulty, Kind};
use base64::engine::Engine;

#[derive(Debug, Deserialize)]
pub(crate) struct TokenRequest {
    pub token: String
}

/// The details of a specified category.
#[derive(Debug, Deserialize)]
pub struct CategoryDetails {
    /// The id of the category.
    #[serde(rename = "category_id")]
    pub id: u8,
    /// The question data about the category.
    #[serde(rename = "category_question_count")]
    pub question_count: QuestionCount
}

#[derive(Debug, Deserialize)]
pub struct QuestionCount {
    /// The total number of questions the category has.
    #[serde(rename = "total_question_count")]
    pub total_questions: u32,
    /// The total number of **easy** questions the category has.
    #[serde(rename = "total_easy_question_count")]
    pub easy_questions: u32,
    /// The total number of **medium** questions the category has.
    #[serde(rename = "total_medium_question_count")]
    pub medium_questions: u32,
    /// The total number of **hard** questions the category has.
    #[serde(rename = "total_hard_question_count")]
    pub hard_questions: u32
}

/// The global details of the API
#[derive(Debug)]
pub struct GlobalDetails {
    /// The overall details of the API.
    pub overall: GlobalDetail,
    /// The overall details of every category.
    pub categories: HashMap<Category, GlobalDetail>
}

/// De global details about a category or about the global API.
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct GlobalDetail {
    /// The total number of questions.
    #[serde(rename = "total_num_of_questions")]
    total_questions: u32,
    /// The total number of pending questions.
    #[serde(rename = "total_num_of_pending_questions")]
    pending_questions: u32,
    /// The total number of verified questions.
    #[serde(rename = "total_num_of_verified_questions")]
    verified_questions: u32,
    /// The total number of rejected questions.
    #[serde(rename = "total_num_of_rejected_questions")]
    rejected_questions: u32
}

/// All the response codes that can be returned from a request using a [base response](BaseResponse)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ResponseCode {
    /// The request finished successfully.
    Success = 0,
    /// There are not enough questions for the provided query.
    NoResults = 1,
    /// The request contains an invalid parameter.
    InvalidParameter = 2,
    /// The provided token does not exist.
    TokenNotFound = 3,
    /// Token has returned all possible questions for the specified query. When this code is
    /// present, that means that is necessary to either reset the token or create a new one.
    TokenEmpty = 4
}

/// The base response the API uses.
#[derive(Debug, Deserialize)]
pub struct BaseResponse<T> {
    /// The response code returned by the API, this contains information about the result of the
    /// request.
    #[serde(deserialize_with = "deserialize_response_code")]
    pub response_code: ResponseCode,
    /// The results of the request.
    pub results: T
}

/// A trivia containing all the data about itself.
#[derive(Debug, Deserialize)]
pub struct Trivia {
    /// The category this trivia belongs to.
    pub category: Category,
    /// The kind of answers this trivia has.
    #[serde(rename = "type")]
    pub kind: Kind,
    /// The difficulty of this trivia.
    pub difficulty: Difficulty,
    /// The question of this trivia.
    #[serde(deserialize_with = "base64_string")]
    pub question: String,
    /// The correct answer of this trivia.
    #[serde(deserialize_with = "base64_string")]
    pub correct_answer: String,
    /// The incorrect answers of this trivia.
    #[serde(deserialize_with = "base64_vec")]
    pub incorrect_answers: Vec<String>
}

#[derive(Debug, Deserialize)]
pub(crate) struct ResetToken {
    pub token: String
}

fn deserialize_response_code<'de, D>(deserializer: D) -> Result<ResponseCode, D::Error>
where
    D: Deserializer<'de>
{
    match u8::deserialize(deserializer)? {
        0 => Ok(ResponseCode::Success),
        1 => Ok(ResponseCode::NoResults),
        2 => Ok(ResponseCode::InvalidParameter),
        3 => Ok(ResponseCode::TokenNotFound),
        4 => Ok(ResponseCode::TokenEmpty),
        e => Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Unsigned(e as u64),
            &"A number contained between 0 and 4"
        ))
    }
}


pub(crate) fn base64_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>
{
    let bytes = base64::engine::general_purpose::STANDARD.decode(String::deserialize(deserializer)?)
        .map_err(serde::de::Error::custom)?;

    String::from_utf8(bytes)
        .map_err(serde::de::Error::custom)
}

fn base64_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>
{
    let v: Vec<String> = serde::de::Deserialize::deserialize(deserializer)?;

    let decoded = v.into_iter()
        .map(|item| base64::engine::general_purpose::STANDARD.decode(item))
        .collect::<Result<Vec<_>, _>>()
        .map_err(serde::de::Error::custom)?
        .into_iter()
        .map(String::from_utf8)
        .collect::<Result<Vec<_>, _>>()
        .map_err(serde::de::Error::custom)?;

    Ok(decoded)
}

impl<'de> Deserialize<'de> for GlobalDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        struct MapVisitor;

        impl<'de> Visitor<'de> for MapVisitor {
            type Value = HashMap<Category, GlobalDetail>;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A map of categories and details")
            }

            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>
            {
                let mut map = HashMap::with_capacity(access.size_hint().unwrap_or(0));

                while let Some((k, v)) = access.next_entry()? {

                    // SAFETY: All the possible category numbers are covered in the category enum
                    // so the value will always correspond to a category variant.
                    let category = unsafe { std::mem::transmute::<u8, Category>(k) };
                    map.insert(category, v);
                }

                Ok(map)
            }
        }

        struct CategoryMap(HashMap<Category, GlobalDetail>);

        impl<'de> Deserialize<'de> for CategoryMap {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>
            {
                Ok(CategoryMap(deserializer.deserialize_map(MapVisitor)?))
            }
        }

        struct GlobalVisitor;

        impl<'de> Visitor<'de> for GlobalVisitor {
            type Value = GlobalDetails;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("GlobalDetails struct")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>
            {
                let mut overall = None;
                let mut categories = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "overall" => {
                            overall = Some(map.next_value::<GlobalDetail>()?);
                        },
                        "categories" => {
                            categories = Some(map.next_value::<CategoryMap>()?.0);
                        },
                        k => panic!("Unrecognized key {k}")
                    }
                }

                assert!(overall.is_some() && categories.is_some());

                Ok(GlobalDetails {
                    overall: overall.unwrap(),
                    categories: categories.unwrap()
                })
            }
        }

        deserializer
            .deserialize_struct("GlobalDetails", &["overall", "categories"], GlobalVisitor)
    }
}
