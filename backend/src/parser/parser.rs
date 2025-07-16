use crate::ir::ast::{Expression, FormalArgument, Function, Statement, Type};
use crate::models::{Block2, Blocks2, Input, NextBlock, Workspace2};
use crate::parser::parser_common::identifier;
use crate::parser::parser_expr::parse_expression;
use actix_web::{HttpResponse, Responder, post, web};
use nom::{Err, Finish};
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, char, digit1, multispace0, multispace1},
    combinator::{map, map_res, not, opt, peek, recognize, value, verify},
    error::Error,
    multi::{fold_many0, many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated, tuple},
};
use serde_json;
use std::fmt::format;
use std::io::Write;
use std::str::FromStr;
use std::{fs::File, process::Output};
use crate::parser::parser_expr::parse_actual_arguments;

pub fn parse_chained_blocks(block: &Block2) -> Result<Statement, String> {
    let mut current_block = Some(block);
    let mut statements_vector: Vec<Statement> = Vec::new();

    while let Some(block_iterator) = current_block {
        let statement: Statement = parse_single_block(block_iterator)?;
        statements_vector.push(statement);
        current_block = block_iterator.next.as_ref().map(|next| next.block.as_ref());
    }
    return Ok(Statement::Block(statements_vector));
}

fn parse_single_block(block: &Block2) -> Result<Statement, String> {
    match block.r#type.as_str() {
        "print_block" => {
            if let Some(expression_string) = block
                .inputs
                .as_ref()
                .and_then(|input| input.get("EXPRESSION"))
                .and_then(|input| input.shadow.as_ref())
                .and_then(|shadow_block| shadow_block.fields.as_ref())
                .and_then(|fields| fields.get("TEXT"))
            {
                if expression_string.is_empty() {
                    return Ok(Statement::Print(Box::new(Expression::CString(
                        "".to_string(),
                    ))));
                }
                let (rest, assignment_exp) = parse_expression(expression_string)
                    .map_err(|_e| format!("Parsing error on expression: {}", expression_string))?;
                if !rest.is_empty() {
                    return Err(format!(
                        "Parsing error on print statement expression: {}",
                        expression_string
                    ));
                }
                // Retorna OK com o Statement usando variable_string e assignment_exp
                return Ok(Statement::Print(Box::new(assignment_exp)));
            } else {
                return Ok(Statement::Print(Box::new(Expression::CString(
                    "".to_string(),
                ))));
            }
        }

        "declaration_block" => {
            if let Some(variable_name) = block
                .fields
                .as_ref()
                .and_then(|fields| fields.get("VARIABLE"))
            {
                if variable_name.is_empty() {
                    return Err("Variable name cannot be empty".to_string());
                }

                let (rest, variable_string) =
                    delimited(multispace0, identifier, multispace0)(variable_name)
                        .map_err(|_e| format!("Parsing error on variable: {}", variable_name))?;
                if !rest.is_empty() {
                    return Err(format!("Parsing error on variable: {}", variable_name));
                }

                if let Some(type_name) = block.fields.as_ref().and_then(|fields| fields.get("TYPE"))
                {
                    let initial_expr = match type_name.as_str() {
                        "INT" => Expression::CInt(0),
                        "FLOAT" => Expression::CReal(0.0),
                        "STRING" => Expression::CString(String::new()),
                        "BOOL" => Expression::CTrue,
                        _ => return Err("non-valid type".to_string()),
                    };
                    return Ok(Statement::VarDeclaration(
                        *variable_string,
                        Box::new(initial_expr),
                    ));
                } else {
                    return Err("Variable type field missing".to_string());
                }
            } else {
                return Err("Variable name field missing".to_string());
            }
        }

        "assignment_block" => {
            if let Some(variable_name) = block
                .fields
                .as_ref()
                .and_then(|fields| fields.get("VARIABLE"))
            {
                if variable_name.is_empty() {
                    return Err("Variable name cannot be empty".to_string());
                }

                let (rest, variable_string) =
                    delimited(multispace0, identifier, multispace0)(variable_name)
                        .map_err(|_e| format!("Parsing error on variable: {}", variable_name))?;
                if !rest.is_empty() {
                    return Err(format!("Parsing error on variable: {}", variable_name));
                }

                if let Some(expression_string) = block
                    .inputs
                    .as_ref()
                    .and_then(|input| input.get("EXPRESSION"))
                    .and_then(|input| input.shadow.as_ref())
                    .and_then(|shadow_block| shadow_block.fields.as_ref())
                    .and_then(|fields| fields.get("TEXT"))
                {
                    if expression_string.is_empty() {
                        return Err("Variable assignment requires non-empty expression".to_string());
                    }

                    let (rest, assignment_exp) =
                        parse_expression(expression_string).map_err(|_e| {
                            format!("Parsing error on expression: {}", expression_string)
                        })?;
                    if !rest.is_empty() {
                        return Err(format!(
                            "Parsing error on expression: {}",
                            expression_string
                        ));
                    }

                    // Retorna OK com o Statement usando variable_string e assignment_exp
                    return Ok(Statement::Assignment(
                        *variable_string,
                        Box::new(assignment_exp),
                    ));
                } else {
                    return Err("Variable assignment requires non-empty expression".to_string());
                }
            } else {
                return Err("Variable name field missing".to_string());
            }
        }

        "if_else_block" => {
            if let Some(condition) = block
                .inputs
                .as_ref()
                .and_then(|i| i.get("CONDITION"))
                .and_then(|input| input.shadow.as_ref())
                .and_then(|shadow_block| shadow_block.fields.as_ref())
                .and_then(|fields| fields.get("TEXT"))
            {
                if condition.is_empty() {
                    return Err("If condition is empty".to_string());
                }
                let (rest, condition_exp) = parse_expression(condition)
                    .map_err(|e| format!("Parsing error on condition: {}", condition))?;
                if !rest.is_empty() {
                    return Err(format!("Parsing error on condition: {}", condition));
                }
                if let Some(if_body) = block
                    .inputs
                    .as_ref()
                    .and_then(|i| i.get("IF_BODY"))
                    .and_then(|input| input.block.as_ref())
                {
                    let then_block = parse_chained_blocks(if_body)?;

                    if let Some(else_body) = block
                        .inputs
                        .as_ref()
                        .and_then(|i| i.get("ELSE_BODY"))
                        .and_then(|input| input.block.as_ref())
                    {
                        let else_block = parse_chained_blocks(else_body)?;
                        return Ok(Statement::IfThenElse(
                            Box::new(condition_exp),
                            Box::new(then_block),
                            Some(Box::new(else_block)),
                        ));
                    } else {
                        return Ok(Statement::IfThenElse(
                            Box::new(condition_exp),
                            Box::new(then_block),
                            None,
                        ));
                    }
                } else {
                    return Err("Non-existent if-body".to_string());
                }
            } else {
                return Err("If condition is empty".to_string());
            }
        }

        "while_block" => {
            if let Some(condition) = block
                .inputs
                .as_ref()
                .and_then(|i| i.get("CONDITION"))
                .and_then(|input| input.shadow.as_ref())
                .and_then(|shadow_block| shadow_block.fields.as_ref())
                .and_then(|fields| fields.get("TEXT"))
            {
                if condition.is_empty() {
                    return Err("While condition is empty".to_string());
                }
                let (rest, condition_exp) = parse_expression(condition)
                    .map_err(|e| format!("Parsing error on condition: {}", condition))?;
                if !rest.is_empty() {
                    return Err(format!("Parsing error on condition: {}", condition));
                }
                if let Some(while_body) = block
                    .inputs
                    .as_ref()
                    .and_then(|i| i.get("WHILE_BODY"))
                    .and_then(|input| input.block.as_ref())
                {
                    let while_block = parse_chained_blocks(while_body)?;
                    return Ok(Statement::While(
                        Box::new(condition_exp),
                        Box::new(while_block),
                    ));
                } else {
                    return Err("Non-existent while-body".to_string());
                }
            } else {
                return Err("While condition is empty".to_string());
            }
        }
        "function_declaration_block" => {
            let mut func: Function = Function::new();
            let mut func_body: Option<Statement> = None;
            let mut final_return_statement = Statement::Return(Box::new(Expression::CVoid));
            if let Some(return_type) = block
                .fields
                .as_ref()
                .and_then(|fields| fields.get("RETURN_TYPE"))
            {
                func.kind = match return_type.as_str() {
                    "VOID" => Type::TVoid,
                    "INT" => Type::TInteger,
                    "FLOAT" => Type::TReal,
                    "STRING" => Type::TString,
                    "BOOL" => Type::TBool,
                    _ => return Err(format!("Unknown return type: {}", return_type)),
                }
            } else {
                return Err("Function return type cannot be empty".to_string());
            }
            if let Some(func_name) = block
                .fields
                .as_ref()
                .and_then(|fields| fields.get("FUNCTION_NAME"))
            {
                if func_name.is_empty() {
                    return Err("Function was not named".to_string());
                }
                let (rest, func_string) =
                    delimited(multispace0, identifier, multispace0)(func_name)
                        .map_err(|_e| format!("Parsing error on function: {}", func_name))?;
                if !rest.is_empty() {
                    return Err(format!("Parsing error on function: {}", func_name));
                }
                func.name = *func_string;
            } else {
                return Err("Function was not named".to_string());
            }
            if let Some(formal_argument_block) = block
                .inputs
                .as_ref()
                .and_then(|input| input.get("FORMAL_ARGUMENTS"))
                .and_then(|input| input.block.as_ref())
            {
                let mut current_block = formal_argument_block.as_ref();
                loop {
                    if let Some((arg_name, arg_type_str)) =
                        current_block.fields.as_ref().and_then(|fields| {
                            fields
                                .get("FORMAL_ARGUMENT")
                                .zip(fields.get("ARGUMENT_TYPE"))
                        })
                    {
                        let arg_type = match arg_type_str.as_str() {
                            "INT" => Type::TInteger,
                            "FLOAT" => Type::TReal,
                            "STRING" => Type::TString,
                            "BOOL" => Type::TBool,
                            _ => return Err(format!("Unknown argument type: {}", arg_type_str)),
                        };
                        let formal_argument = FormalArgument {
                            argument_name: arg_name.to_string(),
                            argument_type: arg_type,
                        };
                        func.params.push(formal_argument);
                        let next_block = current_block
                            .inputs
                            .as_ref()
                            .and_then(|inputs| inputs.get("NEXT_ARGUMENT"))
                            .and_then(|input| input.block.as_ref());
                        match next_block {
                            Some(next_block) => current_block = next_block.as_ref(),
                            None => break,
                        }
                    }
                }
            } else {
            }
            if let Some(func_block) = block
                .inputs
                .as_ref()
                .and_then(|input| input.get("FUNCTION_BODY"))
                .and_then(|input| input.block.as_ref())
            {
                func_body = Some(parse_chained_blocks(func_block)?); //func_block agora é do tipo Some(Statement::Block())
            } else {
            }
            if let Some(return_str) = block
                .inputs
                .as_ref()
                .and_then(|inputs| inputs.get("RETURN_EXPRESSION"))
                .and_then(|input| input.shadow.as_ref())
                .and_then(|shadow_block| shadow_block.fields.as_ref())
                .and_then(|fields| fields.get("TEXT"))
            {
                if !return_str.is_empty() {
                    let (rest, return_exp) = parse_expression(return_str).map_err(|error| {
                        format!(
                            "Parsing error on return statement {}: {:?}",
                            return_str, error
                        )
                    })?;
                    if !rest.is_empty() {
                        return Err(format!("Parsing error on return_str: {}", return_str));
                    }
                    final_return_statement = Statement::Return(Box::new(return_exp));
                } else {
                }
            } else {
            }
            func.body = match func_body {
                Some(Statement::Block(mut statement_vec)) => {
                    statement_vec.push(final_return_statement);
                    Some(Box::new(Statement::Block(statement_vec)))
                }
                None => Some(Box::new(Statement::Block(vec![final_return_statement]))),
                Some(_) => None,
            };
            if func.body.is_none() {
                return Err(format!(
                    "Parse Error on function {}: function body needs to be a Statement::Block",
                    func.name
                ));
            }
            return Ok(Statement::FuncDef(func));
        }

        "sigle_func_call_block" => 
        {
            if let Some(func_name) = block
                .inputs
                .as_ref()
                .and_then(|i| i.get("FUNC_NAME"))
                .and_then(|input| input.shadow.as_ref())
                .and_then(|shadow_block| shadow_block.fields.as_ref())
                .and_then(|fields| fields.get("TEXT"))
            {
                if func_name.is_empty() {
                    return Err("Function name is empty".to_string());
                }
                else 
                {
                    if let Some(actual_args) = block
                    .inputs
                    .as_ref()
                    .and_then(|i| i.get("ACTUAL_ARGS"))
                    .and_then(|input| input.shadow.as_ref())
                    .and_then(|shadow_block| shadow_block.fields.as_ref())
                    .and_then(|fields| fields.get("TEXT"))
                    {
                        let (input, args) = parse_actual_arguments(&format!("({})", actual_args)) 
                        .map_err(|e| format!("Erro ao fazer parse dos argumentos: {}", e))?;
                        return Ok(Statement::SingleFuncCall(func_name.to_string(), args));                        
                    }
                    else {
                        return Err(format!("Parse Error on single call of function {}", func_name));
                    }
                }
            }
            else {
                return Err(format!("Parse Error on single function call"));
            }
        }
        _ => {
            //output.push(format!("Unknown block type: {}", block.r#type));
            return Err("Non-existent block".to_string());
        }
    }
}