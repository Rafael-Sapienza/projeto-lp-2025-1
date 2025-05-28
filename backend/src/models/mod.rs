use serde::Deserialize;
use std::collections::HashMap;


#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub blocks: Blocks,
}

#[derive(Debug, Deserialize)]
pub struct Blocks {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Deserialize)]
pub struct Block {
    pub r#type: String, // "r#" is because "type" is reserved
    pub id: String,
    pub fields: Option<HashMap<String, String>>,
    pub inputs: Option<HashMap<String, Input>>,
    pub next: Option<NextBlock>,
}

#[derive(Debug, Deserialize)]
pub struct Input {
    pub block: Option<Box<Block>>,
}

#[derive(Debug, Deserialize)]
pub struct NextBlock {
    pub block: Box<Block>,
}
