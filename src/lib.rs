use base64::encode;
use hyper::{body::to_bytes, Body, Client, Method, Request};

const SIZE_X: usize = 64;
const SIZE_Y: usize = 64;
const SIZE: usize = SIZE_X * SIZE_Y;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

struct Screen {
    r: Vec<u8>,
    g: Vec<u8>,
    b: Vec<u8>,
}

impl Screen {
    fn new() -> Self {
        Self {
            r: vec![0; SIZE],
            g: vec![0; SIZE],
            b: vec![0; SIZE],
        }
    }
}

pub struct Pixoo64<'a> {
    address: &'a str,
    screen: Screen,
}

impl<'a> Pixoo64<'a> {
    pub fn new(address: &'a str) -> Self {
        Self {
            address,
            screen: Screen::new(),
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        if x < SIZE_X && y < SIZE_Y {
            let pos = y * SIZE_X + x;
            self.screen.r[pos] = color.r;
            self.screen.g[pos] = color.g;
            self.screen.b[pos] = color.b;
        }
        //TODO: throw error / use get
    }

    pub async fn send(&self) {
        let base64 = self.encode_screen();
        let command = format!(
            "{{ \"Command\": \"Draw/SendHttpGif\", \"PicNum\": 1, \"PicWidth\":64, \"PicOffset\": 0, \"PicID\": 1 , \"PicSpeed\": 1000, \"PicData\": \"{}\"}}",
            base64
        );
        self.reset().await;
        self.send_post(command).await;
    }

    async fn reset(&self) {
        let command = "{ \"Command\": \"Draw/ResetHttpGifId\" }".to_string();
        self.send_post(command).await;
    }

    fn encode_screen(&self) -> String {
        let mut result = vec![0 as u8; SIZE * 3];
        for i in 0..SIZE {
            result[i * 3] = self.screen.r[i];
            result[i * 3 + 1] = self.screen.g[i];
            result[i * 3 + 2] = self.screen.b[i];
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
