use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{char, digit1, multispace0, multispace1},
    combinator::{map, map_res, opt, value, verify},
    error::Error,
    multi::{fold_many0, separated_list0},
    sequence::{delimited, pair, preceded, tuple},
};

use std::str::FromStr;

use crate::ir::ast::Function;
use crate::ir::ast::Statement;
use crate::parser::parser_common::{
    COLON_CHAR,
    // Other character constants
    COMMA_CHAR,
    FUNCTION_ARROW,
    LAMBDA_KEYWORD,
    // Bracket and parentheses constants
    LEFT_BRACKET,
    LEFT_PAREN,
    RIGHT_BRACKET,
    RIGHT_PAREN,
    identifier,
    is_string_char,
    keyword,
};
use crate::parser::parser_stmt::{parse_formal_argument, parse_return_statement};
use crate::parser::parser_type::parse_type;
use crate::{
    ir::ast::Expression,
    parser::{parser_common::END_KEYWORD, parser_stmt::parse_block},
};

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
        parse_lambda,
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

fn parse_lambda(input: &str) -> IResult<&str, Expression> {
    map(
        tuple((
            keyword(LAMBDA_KEYWORD),
            preceded(multispace1, identifier),
            delimited(
                char::<&str, Error<&str>>(LEFT_PAREN),
                separated_list0(
                    tuple((
                        multispace0,
                        char::<&str, Error<&str>>(COMMA_CHAR),
                        multispace0,
                    )),
                    parse_formal_argument,
                ),
                char::<&str, Error<&str>>(RIGHT_PAREN),
            ),
            preceded(multispace0, tag(FUNCTION_ARROW)),
            delimited(
                multispace0,
                parse_type,
                char::<&str, Error<&str>>(COLON_CHAR),
            ),
            parse_return_statement,
            keyword(END_KEYWORD),
        )),
        |(_, name, args, _, t, return_stmt, _)| {
            Expression::Lambda(Function {
                name: name.to_string(),
                kind: t,
                params: args,
                body: Some(Box::new(Statement::Block(vec![return_stmt]))),
            })
        },
    )(input)
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
