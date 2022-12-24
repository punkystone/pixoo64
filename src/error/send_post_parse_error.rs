use std::fmt::Display;

pub struct SendPostParseError;

impl Display for SendPostParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to parse response from device")
    }
}

