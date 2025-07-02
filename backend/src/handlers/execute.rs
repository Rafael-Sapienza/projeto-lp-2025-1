use actix_web::{web, HttpResponse, Responder}; 
use crate::models::{ Value, Workspace, Blocks, Block, Input, NextBlock };


pub async fn execute(payload: web::Json<Workspace>) -> impl Responder {
    let mut output = Vec::new();

    for block in &payload.blocks.blocks {
        // In this assignment, only the top blocks will be assigned
        let mut current_block = Some(block);

        while let Some(sub_block) = current_block {
            if let Some(Value::String(s)) = execute_block(sub_block) {
                if !s.is_empty() {
                    output.push(s);
                }
            }

            current_block = match sub_block.next.as_ref() {
                Some(b) => Some(&*b.block),
                None => None
            }
        }
    }

    HttpResponse::Ok().json(output)
}


fn execute_block(block: &Block,) -> Option<Value> {
    match block.r#type.as_str() {
        "print" => {
            let mut text = String::new();
            if let Some(inputs) = &block.inputs {
                if let Some(input) = inputs.get("TEXT") {
                    text = match get_string_input(input) {
                        Some(s) => s,
                        None => "".to_string()
                    }
                }
            }
            
            Some(Value::String(text))
        }
        "join" => {
            let mut left_text = String::new();
            let mut right_text = String::new();

            if let Some(inputs) = &block.inputs {
                if let Some(input) = inputs.get("TEXT1") {
                    left_text = match get_string_input(input) {
                        Some(s) => s,
                        None => "".to_string()
                    }
                }
            }

            if let Some(inputs) = &block.inputs {
                if let Some(input) = inputs.get("TEXT2") {
                    right_text = match get_string_input(input) {
                        Some(s) => s,
                        None => "".to_string()
                    }
                }
            }

            Some(Value::String(format!("{}{}", left_text, right_text)))
        }
        "textTemplate" => {
            if let Some(s) = get_text_template(block) {
                Some(Value::String(s))
            }
            else {
                Some(Value::String("".to_string()))
            }
        }
        _ => { None }
    }
}


fn get_string_input(input : &Input) -> Option<String> {
    let value = if let Some(sub_block) = &input.block {
        execute_block(sub_block)
    }
    else if let Some(shadow_block) = &input.shadow {
        execute_block(shadow_block)
    }
    else {
        None
    };

    match value {
        Some(found_value) => match found_value {
            Value::String(s) => Some(s),
            Value::Number(n) => Some(n.to_string()),
            Value::Boolean(b) => Some(b.to_string()),
        }
        None => None
    }
}

fn get_text_template(block: &Block) -> Option<String> {
    if let Some(fields) = &block.fields {
        if let Some(value) = fields.get("TEXT") {
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }
    None
}
