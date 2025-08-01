use crate::environment::environment::Environment;
use crate::ir::ast::{
    Expression, FormalArgument, FuncSignature, Function, Name, Statement, Type, ValueConstructor,
};
use crate::type_checker::check_func_call;
use crate::type_checker::expression_type_checker::check_expr;
use crate::{environment, show, show_counter};
use std::collections::HashSet;
use tokio::sync::watch::error;

type ErrorMessage = String;

pub fn check_stmt(
    stmt: Statement,
    env: &Environment<Type>,
) -> Result<Environment<Type>, ErrorMessage> {
    match stmt.clone() {
        Statement::VarDeclaration(var, expr) => {
            show_counter_tp_statement();
            show_tp_statement(format!("Check VarDeclaration: "));
            show_tp_statement(format!("Statement: {:?}", stmt));
            show_tp_statement(format!("Env: {:?}", env));
            check_var_declaration_stmt(var, expr, env)
        }
        Statement::ValDeclaration(var, expr) => {
            show_counter_tp_statement();
            show_tp_statement(format!("Check ValDeclaration: "));
            show_tp_statement(format!("Statement: {:?}", stmt));
            show_tp_statement(format!("Env: {:?}", env));
            check_val_declaration_stmt(var, expr, env)
        }
        Statement::Sequence(stmt1, stmt2) => {
            show_counter_tp_statement();
            show_tp_statement(format!("Check Sequence: "));
            show_tp_statement(format!("Statement: {:?}", stmt));
            show_tp_statement(format!("Env: {:?}", env));
            check_squence_stmt(stmt1, stmt2, env)
        }
        Statement::Assignment(name, exp) => {
            show_counter_tp_statement();
            show_tp_statement(format!("Check Assignment: "));
            show_tp_statement(format!("Statement: {:?}", stmt));
            show_tp_statement(format!("Env: {:?}", env));
            check_assignment_stmt(name, exp, env)
        }
        Statement::IfThenElse(cond, stmt_then, stmt_else_opt) => {
            show_counter_tp_statement();
            show_tp_statement(format!("Check IfThenElse: "));
            show_tp_statement(format!("Statement: {:?}", stmt));
            show_tp_statement(format!("Env: {:?}", env));
            check_if_then_else_stmt(cond, stmt_then, stmt_else_opt, env)
        }
        Statement::While(cond, stmt) => {
            show_counter_tp_statement();
            show_tp_statement(format!("Check While: "));
            show_tp_statement(format!("Statement: {:?}", stmt));
            show_tp_statement(format!("Env: {:?}", env));
            check_while_stmt(cond, stmt, env)
        }
        Statement::For(var, expr, stmt) => {
            show_counter_tp_statement();
            show_tp_statement(format!("Check For: "));
            show_tp_statement(format!("Statement: {:?}", stmt));
            show_tp_statement(format!("Env: {:?}", env));
            check_for_stmt(var, expr, stmt, env)
        }
        Statement::FuncDef(function) => {
            show_counter_tp_statement();
            show_tp_statement(format!("Check FuncDef: "));
            show_tp_statement(format!("Statement: {:?}", stmt));
            show_tp_statement(format!("Env: {:?}", env));
            check_func_def_stmt(function, env)
        }
        Statement::TypeDeclaration(name, cons) => {
            show_counter_tp_statement();
            show_tp_statement(format!("Check TypeDeclaration: "));
            show_tp_statement(format!("Statement: {:?}", stmt));
            show_tp_statement(format!("Env: {:?}", env));
            check_adt_declarations_stmt(name, cons, env)
        }
        Statement::Return(exp) => {
            show_counter_tp_statement();
            show_tp_statement(format!("Check Return: "));
            show_tp_statement(format!("Statement: {:?}", stmt));
            show_tp_statement(format!("Env: {:?}", env));
            check_return_stmt(exp, env)
        }
        Statement::Block(statements_vector) => {
            show_counter_tp_statement();
            show_tp_statement(format!("Check Block: "));
            show_tp_statement(format!("Statement: {:?}", stmt));
            show_tp_statement(format!("Env: {:?}", env));
            check_block_statement(statements_vector, env)
        }
        Statement::Print(exp) => {
            show_counter_tp_statement();
            show_tp_statement(format!("Check Print: "));
            show_tp_statement(format!("Statement: {:?}", stmt));
            show_tp_statement(format!("Env: {:?}", env));
            check_print_statement(exp, env)
        }
        Statement::SingleFuncCall(name, args) => {
            show_counter_tp_statement();
            show_tp_statement(format!("Check SingleFuncCall: "));
            show_tp_statement(format!("Statement: {:?}", stmt));
            show_tp_statement(format!("Env: {:?}", env));
            match check_func_call(name, args, env) {
                Ok(_) => {
                    return Ok(env.clone());
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        _ => Err("Not implemented yet".to_string()),
    }
}

fn check_squence_stmt(
    stmt1: Box<Statement>,
    stmt2: Box<Statement>,
    env: &Environment<Type>,
) -> Result<Environment<Type>, ErrorMessage> {
    let new_env = check_stmt(*stmt1, &env)?;
    check_stmt(*stmt2, &new_env)
}

fn check_block_statement(
    statements_vector: Vec<Statement>,
    env: &Environment<Type>,
) -> Result<Environment<Type>, ErrorMessage> {
    let mut new_env = env.clone();
    new_env.push();
    for statement in &statements_vector {
        new_env = check_stmt(statement.clone(), &new_env)?;
    }
    new_env.pop();
    return Ok(new_env);
}

fn check_print_statement(
    exp: Box<Expression>,
    env: &Environment<Type>,
) -> Result<Environment<Type>, ErrorMessage> {
    let new_env = env.clone();
    let exp_type = check_expr(*exp, &new_env)?;
    match exp_type {
        Type::TString | Type::TInteger | Type::TReal | Type::TBool => Ok(new_env),
        _ => Err("Print statement does not support values of this type".into()),
    }
}

fn check_assignment_stmt(
    name: Name,
    exp: Box<Expression>,
    env: &Environment<Type>,
) -> Result<Environment<Type>, ErrorMessage> {
    let mut new_env = env.clone();
    let exp_type = check_expr(*exp.clone(), &new_env)?;

    match *exp {
        Expression::Lambda(mut func) => {
            func.name = name;
            new_env = check_func_def_stmt(func, env)?;
            Ok(new_env)
        }
        _ => match new_env.lookup(&name) {
            Some((mutable, var_type)) => {
                if !mutable {
                    Err(format!(
                        "[Type Error] cannot reassign '{:?}' variable, since it was declared as a constant value.",
                        name
                    ))
                } else if var_type == Type::TAny {
                    new_env.change_variable_value(name.clone(), exp_type)?;
                    Ok(new_env)
                } else if var_type == exp_type {
                    Ok(new_env)
                } else {
                    Err(format!(
                        "[Type Error] expected '{:?}', found '{:?}'.",
                        var_type, exp_type
                    ))
                }
            }
            None => Err(format!("[Type Error] variable '{:?}' not declared.", name)),
        },
    }
}

fn check_var_declaration_stmt(
    name: Name,
    exp: Box<Expression>,
    env: &Environment<Type>,
) -> Result<Environment<Type>, ErrorMessage> {
    let mut new_env = env.clone();
    let var_type = new_env.lookup(&name);
    let exp_type = check_expr(*exp, &new_env)?;

    if var_type.is_none() {
        new_env.create_variable(name.clone(), true, exp_type)?;
        Ok(new_env)
    } else {
        Err(format!(
            "[Type Error] variable '{:?}' already declared",
            name
        ))
    }
}

fn check_val_declaration_stmt(
    name: Name,
    exp: Box<Expression>,
    env: &Environment<Type>,
) -> Result<Environment<Type>, ErrorMessage> {
    let mut new_env = env.clone();
    let var_type = new_env.lookup(&name);
    let exp_type = check_expr(*exp, &new_env)?;

    if var_type.is_none() {
        new_env.create_variable(name.clone(), false, exp_type)?;
        Ok(new_env)
    } else {
        Err(format!(
            "[Type Error] variable '{:?}' already declared",
            name
        ))
    }
}

fn check_if_then_else_stmt(
    cond: Box<Expression>,
    stmt_then: Box<Statement>,
    stmt_else_opt: Option<Box<Statement>>,
    env: &Environment<Type>,
) -> Result<Environment<Type>, ErrorMessage> {
    let mut new_env = env.clone();
    let cond_type = check_expr(*cond, &new_env)?;
    if cond_type != Type::TBool {
        return Err(
            "[Type Error] a condition in a 'if' statement must be of type boolean.".to_string(),
        );
    }
    let then_env = check_stmt(*stmt_then, &new_env)?;
    if let Some(stmt_else) = stmt_else_opt {
        let else_env = check_stmt(*stmt_else, &new_env)?;
        new_env = merge_environments(&then_env, &else_env)?;
    } else {
        new_env = merge_environments(&new_env, &then_env)?;
    }
    Ok(new_env)
}

fn check_while_stmt(
    cond: Box<Expression>,
    stmt: Box<Statement>,
    env: &Environment<Type>,
) -> Result<Environment<Type>, ErrorMessage> {
    let mut new_env = env.clone();
    let cond_type = check_expr(*cond, &new_env)?;
    if cond_type != Type::TBool {
        return Err(
            "[Type Error] a condition in a 'while' statement must be of type boolean.".to_string(),
        );
    }
    new_env = check_stmt(*stmt, &new_env)?;
    Ok(new_env)
}

fn check_for_stmt(
    var: Name,
    expr: Box<Expression>,
    stmt: Box<Statement>,
    env: &Environment<Type>,
) -> Result<Environment<Type>, ErrorMessage> {
    let mut new_env = env.clone();
    let _var_type = env.lookup(&var);
    let expr_type = check_expr(*expr, &new_env)?;
    match expr_type {
        Type::TList(base_type) => {
            if let Some((_, t)) = env.lookup(&var) {
                if t == *base_type || *base_type == Type::TAny {
                    new_env = check_stmt(*stmt, &new_env)?;
                    return Ok(new_env);
                } else {
                    return Err(format!(
                        "[TypeError] Type mismatch between {:?} and {:?}",
                        t, base_type
                    ));
                }
            } else {
                new_env.create_variable(var.clone(), false, *base_type)?;
                new_env = check_stmt(*stmt, &new_env)?;
                return Ok(new_env);
            }
        }
        _ => {
            return Err(format!(
                "[TypeError] Expecting a List type, but found a {:?}",
                expr_type
            ));
        }
    }
}

fn check_func_def_stmt(
    function: Function,
    env: &Environment<Type>,
) -> Result<Environment<Type>, ErrorMessage> {
    let mut new_env = Environment::new();
    let func_signature = FuncSignature::from_func(&function);
    //new_env.push(); -> Push and pop will happen in check_block_statement
    new_env.set_current_func(&func_signature);
    // Previous environment functions and the formal parameters are regarded as global
    new_env.set_global_functions(env.get_all_functions());

    // Ensure that each function is defined only once in current scope
    let current_scope = env.get_current_scope();
    if current_scope
        .functions
        .contains_key(&func_signature)
    {
        return Err(format!(
            "Function {} is defined multiple times",
            func_signature
        ));
    }

    // Ensure that no parameter names are repeated in the function's argument list
    let mut seen_names = HashSet::new();
    for arg in &function.params {
        if !seen_names.insert(arg.argument_name.clone()) {
            return Err(format!(
                "Duplicate parameter name '{}' found in function '{}'",
                arg.argument_name, func_signature
            ));
        }
    }

    for formal_arg in function.params.iter() {
        match formal_arg.argument_type.clone() {
            Type::TFunction(arg_func_ret_type, arg_func_params_type) => {
                let mut params: Vec<FormalArgument> = Vec::new();
                let mut count: u64 = 0;
                for arg_type in &arg_func_params_type {
                    params.push(FormalArgument {
                        argument_name: count.to_string(),
                        argument_type: arg_type.clone(),
                    });
                    count += 1;
                }
                new_env.map_function(Function {
                    name: formal_arg.argument_name.clone(),
                    kind: *arg_func_ret_type,
                    params: params,
                    body: None,
                });
            }
            _ => {
                new_env.create_variable(
                    formal_arg.argument_name.clone(),
                    false,
                    formal_arg.argument_type.clone(),
                )?;
            }
        }
    }

    new_env.map_function(function.clone());
    if let Some(body) = function.body.clone() {
        check_stmt(*body, &new_env)?; //new_env is only used to check function body 
    }

    let mut final_env = env.clone();
    final_env.map_function(function.clone());
    Ok(final_env) // if function body is ok, return original env with new function
}

fn check_adt_declarations_stmt(
    name: Name,
    cons: Vec<ValueConstructor>,
    env: &Environment<Type>,
) -> Result<Environment<Type>, ErrorMessage> {
    let mut new_env = env.clone();
    new_env.map_adt(name.clone(), cons);
    Ok(new_env)
}

fn check_return_stmt(
    exp: Box<Expression>,
    env: &Environment<Type>,
) -> Result<Environment<Type>, ErrorMessage> {
    let new_env = env.clone();

    assert!(new_env.scoped_function());

    let ret_type = check_expr(*exp, &new_env)?;

    let current_func = env.lookup_function(&env.current_func);

    if current_func.is_none() {
        return Err(format!("Type checker: No function to return from"));
    }

    let current_func = current_func.unwrap();

    if ret_type != current_func.kind {
        return Err(format!(
            "Error in function {}:
        Actual return type cannot be different from formal return type \n 
        Actual return type: {:?} \n
        Formal return type: {:?}",
            env.current_func, ret_type, current_func.kind
        ));
    }
    return Ok(new_env);
    /*
    match new_env.lookup(&"return".to_string()) {
        Some(_) => Ok(new_env),
        None => {
            new_env.create_variable("return".to_string(), false, ret_type)?;
            Ok(new_env)
        }
    }
    */
}

fn merge_environments(
    env1: &Environment<Type>,
    env2: &Environment<Type>,
) -> Result<Environment<Type>, ErrorMessage> {
    let mut merged = env1.clone();

    // Get all variables defined in either environment
    for (name, (mutable2, type2)) in env2.get_all_variables() {
        match env1.lookup(&name) {
            Some((mutable1, type1)) => {
                // Variable exists in both branches
                // Check mutability first - if either is constant, result must be constant
                let final_mutable = mutable1 && mutable2;

                // Then check types
                if type1 == Type::TAny {
                    // If type1 is TAny, use type2
                    merged.map_variable(name.clone(), final_mutable, type2.clone());
                } else if type2 == Type::TAny {
                    // If type2 is TAny, keep type1
                    merged.map_variable(name.clone(), final_mutable, type1.clone());
                } else if type1 != type2 {
                    return Err(format!(
                        "[Type Error] Variable '{}' has inconsistent types in different branches: '{:?}' and '{:?}'",
                        name, type1, type2
                    ));
                } else {
                    // Types match, update with combined mutability
                    merged.map_variable(name.clone(), final_mutable, type1.clone());
                }
            }
            None => {
                // Variable only exists in else branch - it's conditionally defined
                merged.map_variable(name.clone(), mutable2, type2.clone());
            }
        }
    }

    //TODO: should we merge ADTs and functions?

    Ok(merged)
}

fn show_tp_statement(texto: String) {
    show(texto, "tp_statement.txt");
}

fn show_counter_tp_statement() {
    show_counter("tp_statement.txt");
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::environment::Environment;
    use crate::ir::ast::Expression::*;
    use crate::ir::ast::FormalArgument;
    use crate::ir::ast::Function;
    use crate::ir::ast::Statement::*;
    use crate::ir::ast::Type;

    #[test]
    fn check_assignment() {
        let env: Environment<Type> = Environment::new();
        // Declare variable 'a' first
        let env = check_stmt(
            Statement::VarDeclaration("a".to_string(), Box::new(CTrue)),
            &env,
        )
        .unwrap();
        let assignment = Assignment("a".to_string(), Box::new(CTrue));

        match check_stmt(assignment, &env) {
            Ok(_) => assert!(true),
            Err(s) => assert!(false, "{}", s),
        }
    }

    #[test]
    fn check_assignment_error2() {
        let env: Environment<Type> = Environment::new();
        // Declare variable 'a' first
        let env = check_stmt(
            Statement::VarDeclaration("a".to_string(), Box::new(CTrue)),
            &env,
        )
        .unwrap();
        let assignment1 = Assignment("a".to_string(), Box::new(CTrue));
        let assignment2 = Assignment("a".to_string(), Box::new(CInt(1)));
        let program = Sequence(Box::new(assignment1), Box::new(assignment2));

        assert!(
            matches!(check_stmt(program, &env), Err(_)),
            "[Type Error on '__main__()'] 'a' has mismatched types: expected 'TBool', found 'TInteger'."
        );
    }

    #[test]
    fn check_if_then_else_error() {
        let env: Environment<Type> = Environment::new();

        let stmt = IfThenElse(
            Box::new(CInt(1)),
            Box::new(Assignment("a".to_string(), Box::new(CInt(1)))),
            Some(Box::new(Assignment("b".to_string(), Box::new(CReal(2.0))))),
        );

        assert!(
            matches!(check_stmt(stmt, &env), Err(_)),
            "[Type Error on '__main__()'] if expression must be boolean."
        );
    }

    #[test]
    fn check_while_error() {
        let env: Environment<Type> = Environment::new();

        let assignment1 = Assignment("a".to_string(), Box::new(CInt(3)));
        let assignment2 = Assignment("b".to_string(), Box::new(CInt(0)));
        let stmt = While(
            Box::new(CInt(1)),
            Box::new(Assignment(
                "b".to_string(),
                Box::new(Add(Box::new(Var("b".to_string())), Box::new(CInt(1)))),
            )),
        );
        let program = Sequence(
            Box::new(assignment1),
            Box::new(Sequence(Box::new(assignment2), Box::new(stmt))),
        );

        assert!(
            matches!(check_stmt(program, &env), Err(_)),
            "[Type Error on '__main__()'] while expression must be boolean."
        );
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn check_func_def() {
        let env: Environment<Type> = Environment::new();

        let func = FuncDef(Function {
            name: "add".to_string(),
            kind: Type::TInteger,
            params: vec![
                FormalArgument::new("a".to_string(), Type::TInteger),
                FormalArgument::new("b".to_string(), Type::TInteger),
            ],
            body: Some(Box::new(Return(Box::new(Add(
                Box::new(Var("a".to_string())),
                Box::new(Var("b".to_string())),
            ))))),
        });
        match check_stmt(func, &env) {
            Ok(_) => assert!(true),
            Err(s) => assert!(false, "{}", s),
        }
    }

    #[test]
    fn test_if_else_consistent_types() {
        let env = Environment::new();
        // Declare variable 'x' first
        let env = check_stmt(
            Statement::VarDeclaration("x".to_string(), Box::new(Expression::CInt(0))),
            &env,
        )
        .unwrap();
        let stmt = Statement::IfThenElse(
            Box::new(Expression::CTrue),
            Box::new(Statement::Assignment(
                "x".to_string(),
                Box::new(Expression::CInt(1)),
            )),
            Some(Box::new(Statement::Assignment(
                "x".to_string(),
                Box::new(Expression::CInt(2)),
            ))),
        );

        // Should succeed - x is consistently an integer in both branches
        assert!(check_stmt(stmt, &env).is_ok());
    }

    #[test]
    fn test_if_else_inconsistent_types() {
        let env = Environment::new();
        let stmt = Statement::IfThenElse(
            Box::new(Expression::CTrue),
            Box::new(Statement::Assignment(
                "x".to_string(),
                Box::new(Expression::CInt(1)),
            )),
            Some(Box::new(Statement::Assignment(
                "x".to_string(),
                Box::new(Expression::CString("hello".to_string())),
            ))),
        );

        // Should fail - x has different types in different branches
        assert!(check_stmt(stmt, &env).is_err());
    }

    #[test]
    fn test_if_else_partial_definition() {
        let env = Environment::new();
        // Declare variable 'x' first
        let env = check_stmt(
            Statement::VarDeclaration("x".to_string(), Box::new(Expression::CInt(0))),
            &env,
        )
        .unwrap();
        let stmt = Statement::Sequence(
            Box::new(Statement::IfThenElse(
                Box::new(Expression::CTrue),
                Box::new(Statement::Assignment(
                    "x".to_string(),
                    Box::new(Expression::CInt(1)),
                )),
                None,
            )),
            Box::new(Statement::Assignment(
                "x".to_string(),
                Box::new(Expression::CInt(2)),
            )),
        );

        // Should succeed - x is conditionally defined in then branch
        // and later used consistently as an integer
        assert!(check_stmt(stmt, &env).is_ok());
    }

    #[test]
    fn test_variable_assignment() {
        let env = Environment::new();
        // Declare variable 'x' first
        let env = check_stmt(
            Statement::VarDeclaration("x".to_string(), Box::new(Expression::CInt(0))),
            &env,
        )
        .unwrap();
        let stmt = Statement::Assignment("x".to_string(), Box::new(Expression::CInt(42)));

        // Should succeed and add x:integer to environment
        let new_env = check_stmt(stmt, &env).unwrap();
        assert_eq!(
            new_env.lookup(&"x".to_string()),
            Some((true, Type::TInteger))
        );
    }

    #[test]
    fn test_variable_reassignment_same_type() {
        let mut env = Environment::new();
        env.map_variable("x".to_string(), true, Type::TInteger);

        let stmt = Statement::Assignment("x".to_string(), Box::new(Expression::CInt(100)));

        // Should succeed - reassigning same type
        assert!(check_stmt(stmt, &env).is_ok());
    }

    #[test]
    fn test_variable_reassignment_different_type() {
        let mut env = Environment::new();
        env.map_variable("x".to_string(), true, Type::TInteger);

        let stmt = Statement::Assignment(
            "x".to_string(),
            Box::new(Expression::CString("hello".to_string())),
        );

        // Should fail - trying to reassign different type
        assert!(check_stmt(stmt, &env).is_err());
    }

    #[test]
    fn test_function_scoping() {
        let mut env: Environment<i32> = Environment::new();

        let global_func = Function {
            name: "global".to_string(),
            kind: Type::TVoid,
            params: Vec::new(),
            body: None,
        };

        let _local_func = Function {
            name: "local".to_string(),
            kind: Type::TVoid,
            params: Vec::new(),
            body: None,
        };

        // Test function scoping
        env.map_function(global_func.clone());
        assert!(env.lookup_function(&"global".to_string()).is_some());
    }

    #[test]
    fn test_for_valid_integer_list() {
        let mut env = Environment::new();
        env.map_variable("sum".to_string(), true, Type::TInteger);
        let stmt = Statement::For(
            "x".to_string(),
            Box::new(Expression::ListValue(vec![
                Expression::CInt(1),
                Expression::CInt(2),
                Expression::CInt(3),
            ])),
            Box::new(Statement::Assignment(
                "sum".to_string(),
                Box::new(Expression::Add(
                    Box::new(Expression::Var("sum".to_string())),
                    Box::new(Expression::Var("x".to_string())),
                )),
            )),
        );
        assert!(check_stmt(stmt, &env).is_ok());
    }

    #[test]
    fn test_for_mixed_type_list() {
        let env = Environment::new();
        let stmt = Statement::For(
            "x".to_string(),
            Box::new(Expression::ListValue(vec![
                Expression::CInt(1),
                Expression::CString("hello".to_string()),
                Expression::CInt(3),
            ])),
            Box::new(Statement::Assignment(
                "x".to_string(),
                Box::new(Expression::CInt(1)),
            )),
        );
        // Should fail - list contains mixed types (integers and strings)
        assert!(check_stmt(stmt, &env).is_err());
    }

    #[test]
    fn test_for_empty_list() {
        let env = Environment::new();
        // Declare variable 'x' first
        let env = check_stmt(
            Statement::VarDeclaration("x".to_string(), Box::new(Expression::CInt(0))),
            &env,
        )
        .unwrap();
        let stmt = Statement::For(
            "x".to_string(),
            Box::new(Expression::ListValue(vec![])),
            Box::new(Statement::Assignment(
                "x".to_string(),
                Box::new(Expression::CInt(1)),
            )),
        );
        // Should succeed - empty list is valid, though no iterations will occur
        assert!(check_stmt(stmt, &env).is_ok());
    }

    #[test]
    fn test_for_iterator_variable_reassignment() {
        let env = Environment::new();
        let stmt = Statement::For(
            "x".to_string(),
            Box::new(Expression::ListValue(vec![
                Expression::CInt(1),
                Expression::CInt(2),
            ])),
            Box::new(Statement::Assignment(
                "x".to_string(),
                Box::new(Expression::CString("invalid".to_string())),
            )),
        );
        // Should fail - trying to assign string to iterator variable when iterating over integers
        assert!(check_stmt(stmt, &env).is_err());
    }

    #[test]
    fn test_for_nested_loops() {
        let env = Environment::new();
        // Declare variable 'sum' first
        let env = check_stmt(
            Statement::VarDeclaration("sum".to_string(), Box::new(Expression::CInt(0))),
            &env,
        )
        .unwrap();
        let stmt = Statement::For(
            "i".to_string(),
            Box::new(Expression::ListValue(vec![
                Expression::CInt(1),
                Expression::CInt(2),
            ])),
            Box::new(Statement::For(
                "j".to_string(),
                Box::new(Expression::ListValue(vec![
                    Expression::CInt(3),
                    Expression::CInt(4),
                ])),
                Box::new(Statement::Assignment(
                    "sum".to_string(),
                    Box::new(Expression::Add(
                        Box::new(Expression::Var("i".to_string())),
                        Box::new(Expression::Var("j".to_string())),
                    )),
                )),
            )),
        );

        // Should succeed - nested loops with proper variable usage
        assert!(check_stmt(stmt, &env).is_ok());
    }

    #[test]
    fn test_for_variable_scope() {
        let mut env = Environment::new();
        env.map_variable("x".to_string(), true, Type::TString); // x is defined as string in outer scope

        let stmt = Statement::For(
            "x".to_string(), // reusing name x as iterator
            Box::new(Expression::ListValue(vec![
                Expression::CInt(1),
                Expression::CInt(2),
            ])),
            Box::new(Statement::Assignment(
                "y".to_string(),
                Box::new(Expression::Var("x".to_string())),
            )),
        );

        // Should not succeed - for loop creates new scope, x is temporarily an integer
        // TODO: Let discuss this case here next class.
        assert!(check_stmt(stmt, &env).is_err());
    }
}
*/
