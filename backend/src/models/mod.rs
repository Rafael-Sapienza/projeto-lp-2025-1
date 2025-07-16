use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub blocks: Blocks,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace2 {
    pub blocks: Blocks2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blocks {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blocks2 {
    pub blocks: Vec<Block2>,
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
pub struct Block2 {
    pub r#type: String, // "r#" is because "type" is reserved
    pub id: String,
    pub fields: Option<HashMap<String, String>>,
    pub inputs: Option<HashMap<String, Input2>>,
    pub next: Option<NextBlock2>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    pub block: Option<Box<Block>>,
    pub shadow: Option<Box<Block>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Input2 {
    pub block: Option<Box<Block2>>,
    pub shadow: Option<Box<Block2>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NextBlock {
    pub block: Box<Block>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NextBlock2 {
    pub block: Box<Block2>,
}

pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
}
