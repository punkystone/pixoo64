use std::fmt::Display;

use super::{set_access_error::SetAccessError, set_out_of_range_error::SetOutOfRangeError};

pub enum SetError {
    SetOutOfRangeError(SetOutOfRangeError),
    SetAccessError(SetAccessError),
}

impl Display for SetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetError::SetOutOfRangeError(error) => write!(f, "{error}"),
            SetError::SetAccessError(error) => write!(f, "{error}"),
        }
    }
}
