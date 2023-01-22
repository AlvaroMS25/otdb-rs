use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt::Formatter;
use serde::de::{MapAccess, Visitor};
use crate::options::{Category, Difficulty, Kind};
use base64::engine::Engine;

#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    pub token: String
}

#[derive(Debug, Deserialize)]
pub struct CategoryDetails {
    #[serde(rename = "category_id")]
    pub id: u8,
    #[serde(rename = "category_question_count")]
    pub question_count: QuestionCount
}

#[derive(Debug, Deserialize)]
pub struct QuestionCount {
    #[serde(rename = "total_question_count")]
    pub total_questions: u32,
    #[serde(rename = "total_easy_question_count")]
    pub easy_questions: u32,
    #[serde(rename = "total_medium_question_count")]
    pub medium_questions: u32,
    #[serde(rename = "total_hard_question_count")]
    pub hard_questions: u32
}

#[derive(Debug)]
pub struct GlobalDetails {
    pub overall: GlobalDetail,
    pub categories: HashMap<Category, GlobalDetail>
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct GlobalDetail {
    #[serde(rename = "total_num_of_questions")]
    total_questions: u32,
    #[serde(rename = "total_num_of_pending_questions")]
    pending_questions: u32,
    #[serde(rename = "total_num_of_verified_questions")]
    verified_questions: u32,
    #[serde(rename = "total_num_of_rejected_questions")]
    rejected_questions: u32
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ResponseCode {
    Success = 0,
    NoResults = 1,
    InvalidParameter = 2,
    TokenNotFound = 3,
    TokenEmpty = 4
}

#[derive(Debug, Deserialize)]
pub struct BaseResponse<T> {
    #[serde(deserialize_with = "deserialize_response_code")]
    pub response_code: ResponseCode,
    pub results: T
}

#[derive(Debug, Deserialize)]
pub struct Trivia {
    pub category: Category,
    #[serde(rename = "type")]
    pub kind: Kind,
    pub difficulty: Difficulty,
    #[serde(deserialize_with = "base64_string")]
    pub question: String,
    #[serde(deserialize_with = "base64_string")]
    pub correct_answer: String,
    #[serde(deserialize_with = "base64_vec")]
    pub incorrect_answers: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct ResetToken {
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
