use crate::models::{Block, EasyInterpreter, Input, Value, Workspace};
use actix_web::{HttpResponse, Responder, web};

pub async fn execute(payload: web::Json<Workspace>) -> impl Responder {
    let mut interpreter = EasyInterpreter::new();

    interpreter.run(&payload.blocks.blocks);

    HttpResponse::Ok().json(interpreter.into_output())
}
