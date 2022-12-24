use std::{fmt::Display, string::FromUtf8Error};

use super::{
    send_post_parse_error::SendPostParseError,
    send_post_request_build_error::SendPostRequestBuildError,
    send_post_request_error::SendPostRequestError,
};

pub enum SendPostError {
    SendPostRequestBuildError(SendPostRequestBuildError),
    SendPostRequestError(SendPostRequestError),
    SendPostParseError(SendPostParseError),
}

impl Display for SendPostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SendPostError::SendPostRequestBuildError(error) => write!(f, "{error}"),
            SendPostError::SendPostRequestError(error) => write!(f, "{error}"),
            SendPostError::SendPostParseError(error) => write!(f, "{error}"),
        }
    }
}

impl From<hyper::http::Error> for SendPostError {
    fn from(_: hyper::http::Error) -> Self {
        SendPostError::SendPostRequestBuildError(SendPostRequestBuildError)
    }
}

impl From<hyper::Error> for SendPostError {
    fn from(_: hyper::Error) -> Self {
        SendPostError::SendPostRequestError(SendPostRequestError)
    }
}

impl From<FromUtf8Error> for SendPostError {
    fn from(_: FromUtf8Error) -> Self {
        SendPostError::SendPostParseError(SendPostParseError)
    }
}

impl From<serde_json::Error> for SendPostError {
    fn from(_: serde_json::Error) -> Self {
        SendPostError::SendPostParseError(SendPostParseError)
    }
}
