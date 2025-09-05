use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub level: i32,
    pub vocation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginHttpRequest {
    pub account: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginHttpResponse {
    pub ok: bool,
    pub message: String,
    pub characters: Vec<Character>,
    pub world: World,
}
