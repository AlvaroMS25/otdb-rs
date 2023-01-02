#[derive(Debug)]
pub enum HttpError {
    Request(reqwest::Error),
    UnsuccessfulRequest(reqwest::StatusCode, String),
    InternalServerError(String),
    InvalidOption(String)
}

pub type OTDBResult<T> = std::result::Result<T, HttpError>;

impl From<reqwest::Error> for HttpError {
    fn from(e: reqwest::Error) -> Self {
        Self::Request(e)
    }
}

impl std::error::Error for HttpError {}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Request(why) => write!(f, "Reqwest error: {}", why),
            Self::UnsuccessfulRequest(code, body) => write!(f, "Unsuccessful response, code: {}, body: {}", code, body),
            Self::InternalServerError(why) => write!(f, "Internal server error: {}", why),
            Self::InvalidOption(why) => write!(f, "Invalid option: {}", why)
        }
    }
}