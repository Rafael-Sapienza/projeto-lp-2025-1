use actix_web::{post, web, HttpResponse, Responder}; use crate::models::{ Workspace, Blocks, Block, Input, NextBlock };
use std::fs::File;
use std::io::Write;
use serde_json;


pub async fn execute(payload: web::Json<Workspace>) -> impl Responder {
    let mut output:Vec<String> = Vec::new();
    output.push("a".to_string());

    println!("Recebi execução");
    //println!("{:#?}",payload.blocks.blocks);
    // Serializa o JSON como string formatada

    let blocks_only = &payload.blocks.blocks; //payload.into_inner()

    let json_str = match serde_json::to_string_pretty(blocks_only) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Erro ao serializar JSON: {}", e);
            return HttpResponse::InternalServerError().body("Erro ao salvar JSON");
        }
    };

    // Cria ou sobrescreve o arquivo
    let mut file = match File::create("log.json") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Erro ao criar arquivo: {}", e);
            return HttpResponse::InternalServerError().body("Erro ao salvar JSON");
        }
    };

    // Escreve no arquivo
    if let Err(e) = file.write_all(json_str.as_bytes()) {
        eprintln!("Erro ao escrever no arquivo: {}", e);
        return HttpResponse::InternalServerError().body("Erro ao salvar JSON");
    }

    /*
    for block in &payload.blocks.blocks {
        execute_block(block, &mut output);
    }
    */
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
