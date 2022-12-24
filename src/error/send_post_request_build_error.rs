use std::fmt::Display;

pub struct SendPostRequestBuildError;

impl Display for SendPostRequestBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error building Request")
    }
}
