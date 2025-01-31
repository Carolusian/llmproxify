use std::error::Error;
use std::fmt;
use url::ParseError;

#[derive(Debug)]
pub enum AppError {
    UrlParseError(ParseError),
    RequestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    Other(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::UrlParseError(e) => write!(f, "URL parse error: {}", e),
            AppError::RequestError(e) => write!(f, "Request error: {}", e),
            AppError::SerdeJsonError(e) => {
                write!(f, "JSON serialization/deserialization error: {}", e)
            }
            AppError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl Error for AppError {}

impl From<ParseError> for AppError {
    fn from(e: ParseError) -> Self {
        AppError::UrlParseError(e)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::RequestError(e)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::SerdeJsonError(e)
    }
}

impl From<String> for AppError {
    fn from(msg: String) -> Self {
        AppError::Other(msg)
    }
}