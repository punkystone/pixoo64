#![allow(clippy::missing_errors_doc)]
use crate::{
    constants::{SIZE_X, SIZE_Y},
    error::{
        reset_error::ResetError,
        send_error::SendError,
        send_post_error::SendPostError,
        set_access_error::SetAccessError,
        set_error::SetError,
        set_out_of_range_error::{SetOutOfRangeError, SetOutOfRangeErrorInput},
    },
    model::color::Color,
    model::pixoo64_response::Pixoo64Response,
};
use base64::encode;
use hyper::{body::to_bytes, Body, Client, Method, Request};
pub struct Pixoo64<'a> {
    address: &'a str,
    screen: [[Color; SIZE_X]; SIZE_Y],
}

impl<'a> Pixoo64<'a> {
    #[must_use]
    pub fn new(address: &'a str) -> Self {
        Self {
            address,
            screen: [[Color { r: 0, g: 0, b: 0 }; SIZE_X]; SIZE_Y],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: &Color) -> Result<(), SetError> {
        if x > SIZE_X - 1 {
            return Err(SetError::SetOutOfRangeError(SetOutOfRangeError {
                input: SetOutOfRangeErrorInput::X(x),
            }));
        }
        if y > SIZE_Y - 1 {
            return Err(SetError::SetOutOfRangeError(SetOutOfRangeError {
                input: SetOutOfRangeErrorInput::Y(y),
            }));
        }
        if self.screen.get(x).is_none() {
            return Err(SetError::SetAccessError(SetAccessError));
        }
        if self.screen[x].get(y).is_none() {
            return Err(SetError::SetAccessError(SetAccessError));
        }
        self.screen[x][y] = *color;
        Ok(())
    }

    pub async fn send(&self) -> Result<bool, SendError> {
        let base64 = self.encode_screen();
        let command = format!(
            "{{ \"Command\": \"Draw/SendHttpGif\", \"PicNum\": 1, \"PicWidth\":{}, \"PicOffset\": 0, \"PicID\": 1 , \"PicSpeed\": 1000, \"PicData\": \"{}\"}}",
           SIZE_X, base64
        );
        if self.reset().await? {
            return self.send_post(command).await;
        }
        Ok(false)
    }

    async fn reset(&self) -> Result<bool, ResetError> {
        let command = "{ \"Command\": \"Draw/ResetHttpGifId\" }".to_string();
        self.send_post(command).await
    }

    fn encode_screen(&self) -> String {
        let mut result = vec![0u8; SIZE_X * SIZE_Y * 3];
        for row in self.screen.iter().enumerate() {
            for column in row.1.iter().enumerate() {
                let pos = SIZE_X * column.0 + row.0;
                result[pos * 3] = column.1.r;
                result[pos * 3 + 1] = column.1.g;
                result[pos * 3 + 2] = column.1.b;
            }
        }
        encode(result)
    }

    async fn send_post(&self, command: String) -> Result<bool, SendPostError> {
        let client = Client::new();
        let request = Request::builder()
            .method(Method::POST)
            .uri(format!("http://{}/post", self.address))
            .header("Content-Type", "application/json")
            .header("Content-Length", command.len())
            .body(Body::from(command))?;
        let response = client.request(request).await?;
        let response_string = String::from_utf8(to_bytes(response.into_body()).await?.to_vec())?;
        let pixoo64_response = serde_json::from_str::<Pixoo64Response>(&response_string)?;
        Ok(pixoo64_response.error_code == 0)
    }
}
