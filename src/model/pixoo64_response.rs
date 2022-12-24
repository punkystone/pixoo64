use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pixoo64Response {
    pub error_code: usize,
}
