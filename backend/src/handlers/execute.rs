use actix_web::{web, HttpResponse, Responder}; 
use crate::models::{ Value, Workspace, Blocks, Block, Input, NextBlock };


pub async fn execute(payload: web::Json<Workspace>) -> impl Responder {
    let mut output = Vec::new();

    for block in &payload.blocks.blocks {
        if let Some(value) = execute_block(block) {
            if let Value::String(s) = value {
                if !s.is_empty() {
                    output.push(s);
                }
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
                    let value = if let Some(sub_block) = &input.block {
                        execute_block(sub_block)
                    } else if let Some(shadow_block) = &input.shadow {
                        execute_block(shadow_block)
                    } else {
                        None
                    };
                    
                    if let Some(found_value) = value {
                        text = match found_value {
                            Value::String(s) => s,
                            Value::Number(n) => n.to_string(),
                            Value::Boolean(b) => b.to_string(),
                        };
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
                    let left_value = if let Some(sub_block) = &input.block {
                        execute_block(sub_block)
                    } else if let Some(shadow_block) = &input.shadow {
                        execute_block(shadow_block)
                    } else {
                        None
                    };
                    
                    if let Some(found_value) = left_value {
                        left_text = match found_value {
                            Value::String(s) => s,
                            Value::Number(n) => n.to_string(),
                            Value::Boolean(b) => b.to_string()
                        }
                    }
                }
            }

            if let Some(inputs) = &block.inputs {
                if let Some(input) = inputs.get("TEXT2") {
                    let right_value = if let Some(sub_block) = &input.block {
                        execute_block(sub_block)
                    } else if let Some(shadow_block) = &input.shadow {
                        execute_block(shadow_block)
                    } else {
                        None
                    };

                    if let Some(found_value) = right_value {
                        right_text = match found_value {
                            Value::String(s) => s,
                            Value::Number(n) => n.to_string(),
                            Value::Boolean(b) => b.to_string()
                        }
                    }
                }
            }

            let result = format!("{}{}", left_text, right_text);
            Some(Value::String(result))
        }
        "textTemplate" => {
            println!("Inside textTemplate");
            let mut text = String::new();
            if let Some(fields) = &block.fields {
                if let Some(field) = fields.get("TEXT") {
                    text.push_str(field);
                }
            }
            Some(Value::String(text))
        }
        _ => { None }
    }


    /*if let Some(next) = &block.next {
        execute_block(&next.block, output);
    }*/
}
