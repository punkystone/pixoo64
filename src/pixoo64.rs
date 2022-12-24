use base64::encode;
use hyper::{body::to_bytes, Body, Client, Method, Request};

use crate::{
    color::Color,
    constants::{SIZE_X, SIZE_Y},
};
pub struct Pixoo64<'a> {
    address: &'a str,
    screen: [[Color; SIZE_X]; SIZE_Y],
}

impl<'a> Pixoo64<'a> {
    pub fn new(address: &'a str) -> Self {
        Self {
            address,
            screen: [[Color { r: 0, g: 0, b: 0 }; SIZE_X]; SIZE_Y],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: &Color) {
        if x < SIZE_X && y < SIZE_Y {
            self.screen[x][y] = *color;
        }
        //TODO: throw error / use get
    }

    pub async fn send(&self) {
        let base64 = self.encode_screen();
        let command = format!(
            "{{ \"Command\": \"Draw/SendHttpGif\", \"PicNum\": 1, \"PicWidth\":{}, \"PicOffset\": 0, \"PicID\": 1 , \"PicSpeed\": 1000, \"PicData\": \"{}\"}}",
           SIZE_X, base64
        );
        self.reset().await;
        self.send_post(command).await;
    }

    async fn reset(&self) {
        let command = "{ \"Command\": \"Draw/ResetHttpGifId\" }".to_string();
        self.send_post(command).await;
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

        return encode(result);
    }

    async fn send_post(&self, command: String) {
        let client = Client::new();
        let request = Request::builder()
            .method(Method::POST)
            .uri(format!("http://{}/post", self.address))
            .header("Content-Type", "application/json")
            .header("Content-Length", command.len())
            .body(Body::from(command))
            .unwrap();
        let response = client.request(request).await.unwrap();

        println!(
            "{}, {}",
            response.status(),
            String::from_utf8(to_bytes(response.into_body()).await.unwrap().to_vec()).unwrap()
        )
        //TODO read error code
    }
}
