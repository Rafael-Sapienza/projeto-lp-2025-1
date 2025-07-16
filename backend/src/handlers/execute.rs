use actix_web::{web, HttpResponse, Responder}; 
use crate::models::{ EasyInterpreter, Value, Workspace, Block, Input };

pub async fn execute(payload: web::Json<Workspace>) -> impl Responder {
    let mut interpreter = EasyInterpreter::new();

    interpreter.run(&payload.blocks.blocks);        

    HttpResponse::Ok().json(interpreter.into_output()) 
}

