use serde::Deserialize;
use std::collections::HashMap;
use crate::{OTDBResult, HttpError};
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub(crate) struct TokenRequestResponse {
    pub token: String
}

#[derive(Debug, Deserialize)]
pub struct CategoryQuestionCountResponse {
    pub category_id: String,
    pub category_question_count: CategoryQuestionCount
}

#[derive(Debug, Deserialize)]
pub struct CategoryQuestionCount {
    #[serde(rename = "total_question_count")]
    pub total_questions: u32,
    #[serde(rename = "total_question_count")]
    pub easy_questions: u32,
    #[serde(rename = "total_question_count")]
    pub medium_questions: u32,
    #[serde(rename = "total_question_count")]
    pub hard_questions: u32
}

#[derive(Debug, Deserialize)]
pub struct ApiCountGlobalResponse {
    pub overall: ApiCountGlobalDetail,
    pub categories: HashMap<String, ApiCountGlobalDetail>
}

#[derive(Debug, Deserialize)]
pub struct ApiCountGlobalDetail {
    #[serde(rename = "total_num_of_questions")]
    total_questions: u32,
    #[serde(rename = "total_num_of_pending_questions")]
    pending_questions: u32,
    #[serde(rename = "total_num_of_verified_questions")]
    verified_questions: u32,
    #[serde(rename = "total_num_of_rejected_questions")]
    rejected_questions: u32
}

#[derive(Debug, Copy, Clone)]
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
    #[serde(rename = "results")]
    pub response: T
}

#[derive(Debug, Deserialize)]
pub struct Trivia {
    #[serde(deserialize_with = "deserialize_base64_string")]
    pub category: String,
    #[serde(rename = "type")]
    #[serde(deserialize_with = "deserialize_base64_string")]
    pub kind: String,
    #[serde(deserialize_with = "deserialize_base64_string")]
    pub difficulty: String,
    #[serde(deserialize_with = "deserialize_base64_string")]
    pub question: String,
    #[serde(deserialize_with = "deserialize_base64_string")]
    pub correct_answer: String,
    #[serde(deserialize_with = "deserialize_base64_vec")]
    pub incorrect_answers: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct ResetToken {
    pub token: String
}

fn deserialize_response_code<'de, D>(deserializer: D) -> Result<ResponseCode, D::Error>
where
    D: serde::de::Deserializer<'de>
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


fn deserialize_base64_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::de::Deserializer<'de>
{
    let bytes = base64::decode(String::deserialize(deserializer)?)
        .map_err(serde::de::Error::custom)?;

    Ok(String::from_utf8(bytes)
        .map_err(serde::de::Error::custom)?)
}

fn deserialize_base64_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::de::Deserializer<'de>
{
    let v: Vec<String> = serde::de::Deserialize::deserialize(deserializer)?;

    let decoded = v.into_iter()
        .map(|i| base64::decode(i))
        .collect::<Result<Vec<_>, _>>()
        .map_err(serde::de::Error::custom)?
        .into_iter()
        .map(|s| String::from_utf8(s))
        .collect::<Result<Vec<_>, _>>()
        .map_err(serde::de::Error::custom)?;

    Ok(decoded)
}
