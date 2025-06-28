use actix_web::{web, HttpResponse, Responder}; 
use crate::models::{ Workspace, Blocks, Block, Input, NextBlock };


pub async fn execute(payload: web::Json<Workspace>) -> impl Responder {
    let mut output = Vec::new();

    for block in &payload.blocks.blocks {
        execute_block(block, &mut output);
    }

    HttpResponse::Ok().json(output)
}


fn execute_block(block: &Block, output: &mut Vec<String>) {
    match block.r#type.as_str() {
        "print" => {
            if let Some(fields) = &block.fields {
                if let Some(text) = fields.get("TEXT") {
                    output.push(text.clone());
                }
            }
        }
        _ => {}
    }


    /*if let Some(next) = &block.next {
        execute_block(&next.block, output);
    }*/
}
