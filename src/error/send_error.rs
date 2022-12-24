use std::fmt::Display;

use super::{reset_error::ResetError, send_post_error::SendPostError};

pub struct SendError {
    post_error: SendPostError,
}

impl Display for SendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.post_error {
            SendPostError::SendPostRequestBuildError(error) => write!(f, "{error}"),
            SendPostError::SendPostRequestError(error) => write!(f, "{error}"),
            SendPostError::SendPostParseError(error) => write!(f, "{error}"),
        }
    }
}

impl From<SendPostError> for SendError {
    fn from(error: SendPostError) -> Self {
        SendError { post_error: error }
    }
}
impl From<ResetError> for SendError {
    fn from(error: ResetError) -> Self {
        SendError {
            post_error: error.post_error,
        }
    }
}
