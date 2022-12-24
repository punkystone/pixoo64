use super::{set_access_error::SetAccessError, set_out_of_range_error::SetOutOfRangeError};

pub enum SetError {
    SetOutOfRangeError(SetOutOfRangeError),
    SetAccessError(SetAccessError),
}
