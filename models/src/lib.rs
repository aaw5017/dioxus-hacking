use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PingResponse {
    pub message: String,
}
impl PingResponse {
    pub fn new() -> Self {
        Self {
            message: String::from("pong"),
        }
    }
}
