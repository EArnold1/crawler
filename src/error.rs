use std::{fmt::Debug, io::Error as IoError};

use url::ParseError;

pub enum CrawlerError {
    Io(IoError),
    HttpError(reqwest::Error),
    ParsingUrlError(ParseError),
}

impl From<IoError> for CrawlerError {
    fn from(err: IoError) -> Self {
        CrawlerError::Io(err)
    }
}

impl From<reqwest::Error> for CrawlerError {
    fn from(err: reqwest::Error) -> Self {
        CrawlerError::HttpError(err)
    }
}

impl From<ParseError> for CrawlerError {
    fn from(err: ParseError) -> Self {
        CrawlerError::ParsingUrlError(err)
    }
}

impl Debug for CrawlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CrawlerError::Io(err) => write!(f, "IO error: {}", err),
            CrawlerError::HttpError(err) => write!(f, "Http error: {}", err),
            CrawlerError::ParsingUrlError(err) => write!(f, "Parsing url failed: {:?}", err),
        }
    }
}
