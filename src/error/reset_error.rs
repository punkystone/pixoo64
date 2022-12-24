use std::fmt::Display;

use super::send_post_error::SendPostError;

pub struct ResetError {
    pub post_error: SendPostError,
}

impl Display for ResetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.post_error {
            SendPostError::SendPostRequestBuildError(error) => write!(f, "{error}"),
            SendPostError::SendPostRequestError(error) => write!(f, "{error}"),
            SendPostError::SendPostParseError(error) => write!(f, "{error}"),
        }
    }
}

impl From<SendPostError> for ResetError {
    fn from(error: SendPostError) -> Self {
        ResetError { post_error: error }
    }
}
