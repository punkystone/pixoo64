use std::fmt::Display;

pub struct SetAccessError;

impl Display for SetAccessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid access")
    }
}
