use serde_json::Value as JsonValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub blocks: Blocks,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blocks {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub r#type: String, // "r#" is because "type" is reserved
    pub id: String,
    pub fields: Option<HashMap<String, JsonValue>>,
    pub inputs: Option<HashMap<String, Input>>,
    pub next: Option<NextBlock>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    pub block: Option<Box<Block>>,
    pub shadow: Option<Box<Block>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NextBlock {
    pub block: Box<Block>,
}

pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
}
