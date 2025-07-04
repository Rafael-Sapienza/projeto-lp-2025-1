use crate::ir::ast::{Expression, FormalArgument, Function, Statement};
use crate::models::{Block, Blocks, Input, NextBlock, Workspace};
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
use serde_json;
use std::io::Write;
use std::str::FromStr;
use std::{fs::File, process::Output};

pub fn parse_chained_blocks(block: &Block) -> Result<Statement, String> {
    let mut current_block = Some(block);
    let mut statements_vector: Vec<Statement> = Vec::new();

    while let Some(block_iterator) = current_block {
        let statement: Statement = parse_single_block(block_iterator)?;
        statements_vector.push(statement);
        current_block = block_iterator.next.as_ref().map(|next| next.block.as_ref());
    }
    return Ok(Statement::Block(statements_vector));
}

fn parse_single_block(block: &Block) -> Result<Statement, String> {
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
                let (rest, assignment_exp) = parse_expression(expression_string.as_str())
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
                println!("Variável: {}", variable_name);
                if variable_name.is_empty() {
                    return Err("Variable name cannot be empty".to_string());
                }

                let (rest, variable_string) =
                    delimited(multispace0, identifier, multispace0)(variable_name.as_str())
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
                println!("Variável: {}", variable_name);
                if variable_name.is_empty() {
                    return Err("Variable name cannot be empty".to_string());
                }

                let (rest, variable_string) =
                    delimited(multispace0, identifier, multispace0)(variable_name.as_str())
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

                    let (rest, assignment_exp) = parse_expression(expression_string.as_str())
                        .map_err(|_e| {
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
                let (rest, condition_exp) = parse_expression(condition.as_str())
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
                    let then_block = parse_chained_blocks(if_body.as_ref())?;

                    if let Some(else_body) = block
                        .inputs
                        .as_ref()
                        .and_then(|i| i.get("ELSE_BODY"))
                        .and_then(|input| input.block.as_ref())
                    {
                        let else_block = parse_chained_blocks(else_body.as_ref())?;
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
                let (rest, condition_exp) = parse_expression(condition.as_str())
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
                    println!("while_body: {:?}", *while_body);
                    let while_block = parse_chained_blocks(&*while_body)?;
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
        _ => {
            //output.push(format!("Unknown block type: {}", block.r#type));
            return Err("Non-existent block".to_string());
        }
    }
}

pub fn identifier(input: &str) -> IResult<&str, Box<String>> {
    let (input, _) = multispace0(input)?;

    let (input, first_char) = identifier_start(input)?;
    let (input, rest) = identifier_continue(input)?;

    let ident = format!("{}{}", first_char, rest);

    Ok((input, Box::from(ident.clone())))
}

/// First character of an identifier: [a-zA-Z_]
fn identifier_start(input: &str) -> IResult<&str, &str> {
    alt((alpha1, tag("_")))(input)
}

/// Remaining characters: [a-zA-Z0-9_]*
fn identifier_continue(input: &str) -> IResult<&str, &str> {
    recognize(many0(identifier_start_or_continue))(input)
}

/// A single identifier character: alphanumeric or underscore
fn identifier_start_or_continue(input: &str) -> IResult<&str, &str> {
    recognize(alt((alpha1, tag("_"), nom::character::complete::digit1)))(input)
}

// Parse Expression
/// Parses a reserved keyword (e.g., "if") surrounded by optional spaces
/// Fails if followed by an identifier character

pub const KEYWORDS: &[&str] = &[
    "if",
    "in",
    "else",
    "def",
    "while",
    "for",
    "val",
    "var",
    "return",
    "Ok",
    "Err",
    "Just",
    "Nothing",
    "unwrap",
    "tryUnwrap",
    "isNothing",
    "isError",
    "and",
    "or",
    "not",
    "True",
    "False",
];

// Type name constants
pub const INT_TYPE: &str = "Int";
pub const REAL_TYPE: &str = "Real";
pub const BOOLEAN_TYPE: &str = "Boolean";
pub const STRING_TYPE: &str = "String";
pub const UNIT_TYPE: &str = "Unit";
pub const ANY_TYPE: &str = "Any";

// Special type constructor constants
pub const MAYBE_TYPE: &str = "Maybe";
pub const RESULT_TYPE: &str = "Result";

// Keyword constants
pub const DATA_KEYWORD: &str = "data";
pub const END_KEYWORD: &str = "end";

// Statement keyword constants
pub const IF_KEYWORD: &str = "if";
pub const ELSE_KEYWORD: &str = "else";
pub const WHILE_KEYWORD: &str = "while";
pub const FOR_KEYWORD: &str = "for";
pub const IN_KEYWORD: &str = "in";
pub const ASSERT_KEYWORD: &str = "assert";
pub const DEF_KEYWORD: &str = "def";

// Operator and symbol constants
pub const FUNCTION_ARROW: &str = "->";
pub const PIPE_SYMBOL: &str = "|";
pub const COLON_SYMBOL: &str = ":";
pub const COMMA_SYMBOL: &str = ",";
pub const SEMICOLON_SYMBOL: &str = ";";

// Bracket and parentheses constants
pub const LEFT_BRACKET: char = '[';
pub const RIGHT_BRACKET: char = ']';
pub const LEFT_PAREN: char = '(';
pub const RIGHT_PAREN: char = ')';

// Other character constants
pub const COMMA_CHAR: char = ',';
pub const COLON_CHAR: char = ':';
pub const PIPE_CHAR: char = '|';
pub const SEMICOLON_CHAR: char = ';';
pub const EQUALS_CHAR: char = '=';

/// Accepts any character except '"' and control characters (like \n, \t)
pub fn is_string_char(c: char) -> bool {
    c != '"' && !c.is_control()
}

pub fn keyword<'a>(kw: &'static str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    terminated(
        delimited(multispace0, tag(kw), multispace0),
        not(peek(identifier_start_or_continue)),
    )
}

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    delimited(multispace0, parse_or, multispace0)(input)
    //parse_or(input)
}

fn parse_or(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_and(input)?;
    fold_many0(
        preceded(keyword("or"), parse_and),
        move || init.clone(),
        |acc, val| Expression::Or(Box::new(acc), Box::new(val)),
    )(input)
}

fn parse_and(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_not(input)?;
    fold_many0(
        preceded(keyword("and"), parse_not),
        move || init.clone(),
        |acc, val| Expression::And(Box::new(acc), Box::new(val)),
    )(input)
}

fn parse_not(input: &str) -> IResult<&str, Expression> {
    alt((
        map(preceded(keyword("not"), parse_not), |e| {
            Expression::Not(Box::new(e))
        }),
        parse_relational,
    ))(input)
}

fn parse_relational(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_add_sub(input)?;
    fold_many0(
        pair(
            alt((
                operator("<="),
                operator("<"),
                operator(">="),
                operator(">"),
                operator("=="),
                operator("!="),
            )),
            parse_add_sub,
        ),
        move || init.clone(),
        |acc, (op, val)| match op {
            "<" => Expression::LT(Box::new(acc), Box::new(val)),
            "<=" => Expression::LTE(Box::new(acc), Box::new(val)),
            ">" => Expression::GT(Box::new(acc), Box::new(val)),
            ">=" => Expression::GTE(Box::new(acc), Box::new(val)),
            "==" => Expression::EQ(Box::new(acc), Box::new(val)),
            "!=" => Expression::NEQ(Box::new(acc), Box::new(val)),
            _ => unreachable!(),
        },
    )(input)
}

fn parse_add_sub(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_term(input)?;
    fold_many0(
        pair(alt((operator("+"), operator("-"))), parse_term),
        move || init.clone(),
        |acc, (op, val)| match op {
            "+" => Expression::Add(Box::new(acc), Box::new(val)),
            "-" => Expression::Sub(Box::new(acc), Box::new(val)),
            _ => unreachable!(),
        },
    )(input)
}

fn parse_term(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_factor(input)?;
    fold_many0(
        pair(alt((operator("*"), operator("/"))), parse_factor),
        move || init.clone(),
        |acc, (op, val)| match op {
            "*" => Expression::Mul(Box::new(acc), Box::new(val)),
            "/" => Expression::Div(Box::new(acc), Box::new(val)),
            _ => unreachable!(),
        },
    )(input)
}

fn parse_factor(input: &str) -> IResult<&str, Expression> {
    alt((
        parse_bool,
        parse_number,
        parse_string,
        parse_list,
        parse_function_call,
        parse_var,
        delimited(
            char::<&str, Error<&str>>(LEFT_PAREN),
            parse_expression,
            char::<&str, Error<&str>>(RIGHT_PAREN),
        ),
    ))(input)
}

fn parse_bool(input: &str) -> IResult<&str, Expression> {
    alt((
        value(Expression::CTrue, keyword("True")),
        value(Expression::CFalse, keyword("False")),
    ))(input)
}

fn parse_number(input: &str) -> IResult<&str, Expression> {
    let float_parser = map_res(
        verify(
            tuple((
                opt(char::<&str, Error<&str>>('-')),
                digit1,
                char::<&str, Error<&str>>('.'),
                digit1,
            )),
            |(_, _, _, _)| true,
        ),
        |(sign, d1, _, d2)| {
            let s = match sign {
                Some(_) => format!("-{}.{}", d1, d2),
                None => format!("{}.{}", d1, d2),
            };
            f64::from_str(&s)
        },
    );

    let int_parser = map_res(
        tuple((opt(char::<&str, Error<&str>>('-')), digit1)),
        |(sign, digits)| {
            let s = match sign {
                Some(_) => format!("-{}", digits),
                None => digits.to_string(),
            };
            i32::from_str(&s)
        },
    );

    alt((
        map(float_parser, Expression::CReal),
        map(int_parser, Expression::CInt),
    ))(input)
}

fn parse_string(input: &str) -> IResult<&str, Expression> {
    map(
        delimited(
            multispace0,
            delimited(
                char::<&str, Error<&str>>('"'),
                map(take_while(is_string_char), |s: &str| s.to_string()),
                char::<&str, Error<&str>>('"'),
            ),
            multispace0,
        ),
        |s| Expression::CString(s),
    )(input)
}

fn parse_var(input: &str) -> IResult<&str, Expression> {
    map(identifier, |v| Expression::Var(*v))(input)
}

fn parse_function_call(input: &str) -> IResult<&str, Expression> {
    let (input, name) = identifier(input)?;
    let (input, args) = parse_actual_arguments(input)?;
    Ok((input, Expression::FuncCall(name.to_string(), args)))
}

pub fn parse_actual_arguments(input: &str) -> IResult<&str, Vec<Expression>> {
    map(
        tuple((
            multispace0,
            char::<&str, Error<&str>>(LEFT_PAREN),
            separated_list0(
                tuple((
                    multispace0,
                    char::<&str, Error<&str>>(COMMA_CHAR),
                    multispace0,
                )),
                parse_expression,
            ),
            multispace0,
            char::<&str, Error<&str>>(RIGHT_PAREN),
        )),
        |(_, _, args, _, _)| args,
    )(input)
}

fn parse_list(input: &str) -> IResult<&str, Expression> {
    let (input, _) = multispace0(input)?;
    let (input, _) = char(LEFT_BRACKET)(input)?;
    let (input, _) = multispace0(input)?;

    let (input, elements) = separated_list0(
        delimited(multispace0, char(COMMA_CHAR), multispace0),
        parse_expression,
    )(input)?;

    let (input, _) = multispace0(input)?;
    let (input, _) = char(RIGHT_BRACKET)(input)?;
    let (input, _) = multispace0(input)?;

    Ok((input, Expression::ListValue(elements)))
}

/// Parses an operator.
fn operator<'a>(op: &'static str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    delimited(multispace0, tag(op), multispace0)
}
