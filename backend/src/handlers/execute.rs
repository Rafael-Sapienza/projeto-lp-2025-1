use actix_web::{web, HttpResponse, Responder}; use crate::models::{ Workspace, Blocks, Block, Input, NextBlock };


pub async fn execute(payload: web::Json<Workspace>) -> impl Responder {
    let mut output = Vec::new();

    for block in &payload.blocks.blocks {
        execute_block(block, &mut output);
    }

    HttpResponse::Ok().json(output)
}


fn execute_block(block: &Block, output: &mut Vec<String>) {
    match block.r#type.as_str() {
        "print_block" => {
            if let Some(fields) = &block.fields {
                if let Some(text) = fields.get("TEXT") {
                    output.push(text.clone());
                }
            }
        }
        "logic_boolean" => {
            // No execution here directly
        }
        "if_else_block" => {
            let condition = block.inputs.as_ref()
                .and_then(|i| i.get("CONDITION"))
                .and_then(|input| input.block.as_ref())
                .map_or(false, |b| {
                    if let Some(fields) = &b.fields {
                        fields.get("BOOL").map(|v| v == "TRUE").unwrap_or(false)
                    } else {
                        false
                    }
                });

            if condition {
                if let Some(input) = block.inputs.as_ref().and_then(|i| i.get("IF_BODY")) {
                    if let Some(body_block) = &input.block {
                        execute_block(body_block, output);
                    }
                }
            } else {
                if let Some(input) = block.inputs.as_ref().and_then(|i| i.get("ELSE_BODY")) {
                    if let Some(body_block) = &input.block {
                        execute_block(body_block, output);
                    }
                }
            }
        }

        _ => {
            output.push(format!("Unknown block type: {}", block.r#type));
        }
    }

    if let Some(next) = &block.next {
        execute_block(&next.block, output);
    }
}
