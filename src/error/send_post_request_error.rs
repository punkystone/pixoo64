use std::fmt::Display;

pub struct SendPostRequestError;

impl Display for SendPostRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error executing request")
    }
}
