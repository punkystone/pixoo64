use std::fmt::Display;

use crate::constants::{SIZE_X, SIZE_Y};
pub enum SetOutOfRangeErrorInput {
    X(usize),
    Y(usize),
}
pub struct SetOutOfRangeError {
    pub input: SetOutOfRangeErrorInput,
}

impl Display for SetOutOfRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.input {
            SetOutOfRangeErrorInput::X(value) => {
                write!(
                    f,
                    "Invalid X value: {} (Min: 0, Max: {})",
                    value,
                    SIZE_X - 1
                )
            }
            SetOutOfRangeErrorInput::Y(value) => {
                write!(
                    f,
                    "Invalid Y value: {} (Min: 0, Max: {})",
                    value,
                    SIZE_Y - 1
                )
            }
        }
    }
}
