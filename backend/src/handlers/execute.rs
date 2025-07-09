use crate::COUNTER;
use crate::environment::environment::Environment;
use crate::interpreter::run;
use crate::ir::ast::{Expression, FormalArgument, Function, Statement, Type};
use crate::models::{Block, Blocks, Input, NextBlock, Workspace};
use crate::parser::parse_chained_blocks;
use crate::type_checker::check_stmt;
use actix_web::{HttpResponse, Responder, post, web};
use nom::{Err, Finish};
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, char, digit1, multispace0},
    combinator::{map, map_res, not, opt, peek, recognize, value, verify},
    error::Error,
    multi::{fold_many0, many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated, tuple},
};
use serde::Serialize;
use serde_json;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::str::FromStr;
use std::{fs::File, process::Output};

pub async fn execute(payload: web::Json<Workspace>) -> impl Responder {
    reset_txt_files();

    let mut output: Vec<String> = Vec::new();

    println!("Recebi execução");

    let blocks_only = &payload.blocks.blocks; //payload.into_inner()

    if let Err(err_msg) = save_json_in_file(blocks_only, "log.json") {
        // Retorna InternalServerError com mensagem de erro
        return HttpResponse::InternalServerError().body(err_msg);
    }
    
    output = process_blocks(blocks_only);

    //output = process_blocks(blocks_only);
    HttpResponse::Ok().json(output)
}

pub fn execute_with_json() {
    let mut output = Vec::new();
    let blocks_only_res = generate_blocks_only("factorial.json");
    if let Ok(blocks_only) = &blocks_only_res {
        output = process_blocks(blocks_only);
    }
    show_ex(format!("{:?}", output));
}

pub fn generate_blocks_only(path: &str) -> Result<Vec<Block>, String> {
    let json_str =
        fs::read_to_string(path).map_err(|e| format!("Erro ao ler arquivo '{}': {}", path, e))?;

    let blocks_only: Vec<Block> = serde_json::from_str(&json_str)
        .map_err(|e| format!("Erro ao desserializar JSON: {}", e))?;

    Ok(blocks_only)
}

pub fn save_json_in_file<T: Serialize>(blocks_only: &T, caminho: &str) -> Result<(), String> {
    let json_str = serde_json::to_string_pretty(blocks_only)
        .map_err(|e| format!("Erro ao serializar JSON: {}", e))?;

    let mut file = File::create(caminho).map_err(|e| format!("Erro ao criar arquivo: {}", e))?;

    file.write_all(json_str.as_bytes())
        .map_err(|e| format!("Erro ao escrever no arquivo: {}", e))?;

    Ok(())
}

pub fn process_blocks(blocks_only: &Vec<Block>) -> Vec<String> {
    let mut output = vec!["output is empty".to_string()];
    let mut number_of_global_estatements: u16 = 0;
    let mut global_statements: Option<Vec<Statement>> = None;
    let mut func_def_statements: Vec<Statement> = Vec::new();
    for block in blocks_only {
        if block.r#type == "function_declaration_block" {
            let func_name_opt = block
                .fields
                .as_ref()
                .and_then(|fields| fields.get("FUNCTION_NAME"));
            match parse_chained_blocks(block) {
                Ok(statement) => {
                    show_counter_ex();
                    show_ex(format!(
                        "Function {} parsed successfully",
                        func_name_opt.unwrap()
                    ));
                    show_ex(format!("Function statement: {:?}", statement));
                    match statement.clone() {
                        Statement::Block(func_def) => func_def_statements.push(func_def[0].clone()),
                        _ => show_ex("parse chained tem que retornar block".to_string()),
                    }
                }
                Err(e) => {
                    show_ex(format!("{:?}", e));
                }
            }
        } else {
            number_of_global_estatements += 1;
            match parse_chained_blocks(block) {
                Ok(statement) => {
                    show_counter_ex();
                    show_ex("main statement parsed successfully".to_string());
                    show_ex(format!("main statement: {:?}", statement));
                    match statement {
                        Statement::Block(vector) => global_statements = Some(vector),
                        _ => show_ex("main body cannot be empty".to_string()),
                    }
                }
                Err(e) => {
                    show_ex(format!("{:?}", e));
                }
            }
        }
    }
    if number_of_global_estatements != 1 {
        show_ex("There must be one and only one global statement".to_string());
    } else {
        let global_statements = global_statements.unwrap();
        for vector in global_statements {
            func_def_statements.push(vector);
        }
        let final_statement = Statement::Block(func_def_statements);
        show_counter_ex();
        show_ex(format!("final statement: {:?}", final_statement));
        let type_env: Environment<Type> = Environment::new();
        show_counter_ex();
        show_ex(format!("Initial Type Env: {:?}", type_env));
        match check_stmt(final_statement.clone(), &type_env) {
            Ok(new_type_env) => {
                show_counter_ex();
                show_ex(format!("Final Type Env: {:?}", new_type_env));
                let exp_env: Environment<Expression> = Environment::new();
                show_counter_ex();
                show_ex(format!("Initial Exp Env: {:?}", exp_env));
                match run(final_statement.clone(), &exp_env) {
                    Ok(mut new_exp_env) => {
                        show_counter_ex();
                        show_ex(format!("Final Exp Env: {:?}", new_exp_env));
                        show_ex(format!("Variables: {:?} ", new_exp_env.get_all_variables()));
                        if !new_exp_env.get_output().is_empty() {
                            output = new_exp_env.get_output();
                        }
                    }
                    Err(e) => {
                        show_ex(format!("{:?}", e));
                    }
                }
            }
            Err(e) => {
                show_ex(format!("{:?}", e));
            }
        }
    }
    return output;
}

pub fn reset_txt_files() {
    let txt_files = ["ex.txt", "env.txt", "exp_eval.txt", "statement_exec.txt", "tp_statement.txt", "tp_exp.txt"];
    for file in txt_files {
        match File::create(file) {
            Ok(_) => println!("Arquivo '{}' limpo com sucesso.", file),
            Err(e) => eprintln!("Erro ao limpar '{}': {}", file, e),
        }
    }
}

pub fn show(texto: String, path: &str) {
    let mut file = OpenOptions::new()
        //.create(true)
        .append(true)
        .open(path)
        .unwrap(); // Ignora erro de abrir/criar o arquivo, panica se falhar

    writeln!(file, "{}\n", texto).unwrap(); // Ignora erro de escrita, panica se falhar
}

pub fn show_counter(path: &str) {
    let mut count = COUNTER.lock().unwrap();
    *count += 1;

    let string = format!(
        "

-----------------------------------------
            counter: {}\n",
        *count
    );
    show(string, path);
}

fn show_ex(texto: String) {
    show(texto, "ex.txt");
}

fn show_counter_ex() {
    show_counter("ex.txt");
}
