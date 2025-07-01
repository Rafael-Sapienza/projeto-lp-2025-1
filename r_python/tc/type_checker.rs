use crate::environment::environment::Environment;
use crate::ir::ast::{Expression, FormalArgument, Function, Name, Statement, Type};

type ErrorMessage = String;

pub enum ControlFlow {
    Continue(Environment<Type>),
    Return(Type),
}

pub fn check_expr(exp: Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    match exp {
        Expression::CTrue => Ok(Type::TBool),
        Expression::CFalse => Ok(Type::TBool),
        Expression::CVoid => Ok(Type::TVoid),
        Expression::CInt(_) => Ok(Type::TInteger),
        Expression::CReal(_) => Ok(Type::TReal),
        Expression::CString(_) => Ok(Type::TString),
        Expression::Add(l, r) => check_bin_arithmetic_expression(*l, *r, env),
        Expression::Sub(l, r) => check_bin_arithmetic_expression(*l, *r, env),
        Expression::Mul(l, r) => check_bin_arithmetic_expression(*l, *r, env),
        Expression::Div(l, r) => check_bin_arithmetic_expression(*l, *r, env),
        Expression::And(l, r) => check_bin_boolean_expression(*l, *r, env),
        Expression::Or(l, r) => check_bin_boolean_expression(*l, *r, env),
        Expression::Not(e) => check_not_expression(*e, env),
        Expression::EQ(l, r) => check_bin_relational_expression(*l, *r, env),
        Expression::GT(l, r) => check_bin_relational_expression(*l, *r, env),
        Expression::LT(l, r) => check_bin_relational_expression(*l, *r, env),
        Expression::GTE(l, r) => check_bin_relational_expression(*l, *r, env),
        Expression::LTE(l, r) => check_bin_relational_expression(*l, *r, env),
        Expression::Var(name) => check_var_name(name, env, false),
        Expression::COk(e) => check_result_ok(*e, env),
        Expression::CErr(e) => check_result_err(*e, env),
        Expression::CJust(e) => check_maybe_just(*e, env),
        Expression::CNothing => Ok(Type::TMaybe(Box::new(Type::TAny))),
        Expression::IsError(e) => check_iserror_type(*e, env),
        Expression::IsNothing(e) => check_isnothing_type(*e, env),
        Expression::Unwrap(e) => check_unwrap_type(*e, env),
        Expression::Propagate(e) => check_propagate_type(*e, env),
        Expression::ListValue(elements) => check_list_value(&elements, env),
        _ => Err("not implemented yet.".to_string()), //        Expression::FuncCall(name, args) => check_func_call(name, args, env),
                                                      //        Expression::ADTConstructor(adt_name, constructor_name, args) => check_adt_constructor(adt_name, constructor_name, args, env)
    }
}

pub fn check_stmt(
    stmt: Statement,
    env: &Environment<Type>,
) -> Result<Environment<Type>, ErrorMessage> {
    match stmt {
        Statement::Sequence(stmt1, stmt2) => {
            let new_env = check_stmt(*stmt1, &env)?;
            check_stmt(*stmt2, &new_env)
        }
        Statement::Assignment(name, exp) => {
            let mut new_env = env.clone();
            let var_type = new_env.lookup(&name);
            let exp_type = check_expr(*exp, &new_env)?;

            match var_type {
		Some(t) if *t == Type::TAny => {
		    new_env.map_variable(name.clone(), exp_type);
                    Ok(new_env)
		}
                Some(t) => {
                    if *t != exp_type {
                        return Err(format!(
                            "[Type Error] expected '{:?}', found '{:?}'.",
                            t, exp_type
                        ));
                    } else {
                        return Ok(new_env);
                    }
                }
                None => {
                    new_env.map_variable(name.clone(), exp_type);
                    Ok(new_env)
                }
            }
        }
        Statement::IfThenElse(cond, stmt_then, stmt_else_opt) => {
            let mut new_env = env.clone();
            let cond_type = check_expr(*cond, &new_env)?;
            if cond_type != Type::TBool {
                return Err(
                    "[Type Error] a condition in a 'if' statement must be of type boolean."
                        .to_string(),
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
        Statement::While(cond, stmt) => {
            let mut new_env = env.clone();
            let cond_type = check_expr(*cond, &new_env)?;
            if cond_type != Type::TBool {
                return Err(
                    "[Type Error] a condition in a 'while' statement must be of type boolean."
                        .to_string(),
                );
            }
            new_env = check_stmt(*stmt, &new_env)?;
            Ok(new_env)
        }
        Statement::For(var, expr, stmt) => {
            let mut new_env = env.clone();
            let var_type = env.lookup(&var);
            let expr_type = check_expr(*expr, &new_env)?;
            match expr_type {
                Type::TList(base_type) => {
                    if let Some(t) = env.lookup(&var) {
                        if *t == *base_type || *base_type == Type::TAny {
                            new_env = check_stmt(*stmt, &new_env)?;
                            return Ok(new_env);
                        }
			else {
                            return Err(format!(
                                "[TypeError] Type mismatch between {:?} and {:?}",
                                t, base_type
                            ));
                        }
                    } else {
                        new_env.map_variable(var.clone(), *base_type);
                        new_env = check_stmt(*stmt, &new_env)?;
                        return Ok(new_env);
                    }
                }
                _ => {
                    return Err(format!(
                        "[TypeError] Expecting a List type, but found a {:?}",
                        expr_type
                    ))
                }
            }
        }
        Statement::FuncDef(function) => {
            let mut new_env = env.clone();
            new_env.push();

            for formal_arg in function.params.iter() {
                new_env.map_variable(
                    formal_arg.argumentName.clone(),
                    formal_arg.argumentType.clone(),
                );
            }

            if let Some(body) = function.body.clone() {
                new_env = check_stmt(*body, &new_env)?;
            }
            new_env.pop();
            new_env.map_function(function);

            Ok(new_env)
        }
        Statement::Return(exp) => {
            let mut new_env = env.clone();

            assert!(new_env.scoped_function());

            let ret_type = check_expr(*exp, &new_env)?;

            //TODO: Use a constant RETURN, instead of the string 'return' here.
            match new_env.lookup(&"return".to_string()) {
                Some(ret_type) => Ok(new_env),
                Some(_) => Err("[Type error] Inconsistent return types.".to_string()),
                None => {
                    new_env.map_variable("return".to_string(), ret_type);
                    Ok(new_env)
                }
            }
        }
        _ => Err("Not implemented yet".to_string()), //     Statement::ADTDeclaration(name, constructors) => {
                                                     //         new_env.insert_type(name.clone(), constructors.clone());
                                                     //         Ok(ControlFlow::Continue(new_env))
                                                     //     }
                                                     //     _ => Err(String::from("not implemented yet.")),
                                                     // }
    }
}

// fn check_adt_constructor(
//     adt_name: Name,         // Name of the ADT
//     constructor_name: Name, // Name of the constructor
//     args: Vec<Box<Expression>>,
//     env: &Environment<Type>,
// ) -> Result<Type, ErrorMessage> {
//     // Retrieve the ADT definition from the environment
//     if let Some(constructors) = env.get_type(&adt_name) {
//         // Find the correct constructor by name
//         if let Some(constructor) = constructors.iter().find(|c| c.name == constructor_name) {
//             // Check if the number of arguments matches the expected number
//             if args.len() != constructor.types.len() {
//                 return Err(format!(
//                     "[Type Error in '{}'] ADT constructor '{}' expected {} arguments, found {}.",
//                     env.scope_name(),
//                     constructor_name,
//                     constructor.types.len(),
//                     args.len()
//                 ));
//             }

//             // Check if the arguments match the expected constructor types
//             for (arg, expected_type) in args.iter().zip(&constructor.types) {
//                 let arg_type = check_expr(*arg.clone(), env)?;
//                 if arg_type != *expected_type {
//                     return Err(format!(
//                         "[Type Error in '{}'] ADT constructor '{}' has mismatched argument types: expected '{:?}', found '{:?}'.",
//                         env.scope_name(),
//                         constructor_name,
//                         expected_type,
//                         arg_type
//                     ));
//                 }
//             }

//             // Return the ADT type
//             Ok(Type::Tadt(adt_name.clone(), constructors.clone()))
//         } else {
//             Err(format!(
//                 "[Type Error in '{}'] ADT constructor '{}' not found in ADT '{}'.",
//                 env.scope_name(),
//                 constructor_name,
//                 adt_name
//             ))
//         }
//     } else {
//         Err(format!(
//             "[Type Error in '{}'] ADT '{}' is not defined.",
//             env.scope_name(),
//             adt_name
//         ))
//     }
// }

fn check_var_name(name: Name, env: &Environment<Type>, scoped: bool) -> Result<Type, ErrorMessage> {
    let var_type = env.lookup(&name);
    match var_type {
        Some(t) => Ok(t.clone()),
        None => Err(format!("[Name Error] '{}' is not defined.", name)),
    }
}

fn check_bin_arithmetic_expression(
    left: Expression,
    right: Expression,
    env: &Environment<Type>,
) -> Result<Type, ErrorMessage> {
    let left_type = check_expr(left, env)?;
    let right_type = check_expr(right, env)?;

    match (left_type, right_type) {
        (Type::TInteger, Type::TInteger) => Ok(Type::TInteger),
        (Type::TInteger, Type::TReal) => Ok(Type::TReal),
        (Type::TReal, Type::TInteger) => Ok(Type::TReal),
        (Type::TReal, Type::TReal) => Ok(Type::TReal),
        _ => Err(String::from("[Type Error] expecting numeric type values.")),
    }
}

fn check_bin_boolean_expression(
    left: Expression,
    right: Expression,
    env: &Environment<Type>,
) -> Result<Type, ErrorMessage> {
    let left_type = check_expr(left, env)?;
    let right_type = check_expr(right, env)?;
    match (left_type, right_type) {
        (Type::TBool, Type::TBool) => Ok(Type::TBool),
        _ => Err(String::from("[Type Error] expecting boolean type values.")),
    }
}

fn check_not_expression(exp: Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;

    match exp_type {
        Type::TBool => Ok(Type::TBool),
        _ => Err(String::from("[Type Error] expecting a boolean type value.")),
    }
}

fn check_bin_relational_expression(
    left: Expression,
    right: Expression,
    env: &Environment<Type>,
) -> Result<Type, ErrorMessage> {
    let left_type = check_expr(left, env)?;
    let right_type = check_expr(right, env)?;

    match (left_type, right_type) {
        (Type::TInteger, Type::TInteger) => Ok(Type::TBool),
        (Type::TInteger, Type::TReal) => Ok(Type::TBool),
        (Type::TReal, Type::TInteger) => Ok(Type::TBool),
        (Type::TReal, Type::TReal) => Ok(Type::TBool),
        _ => Err(String::from("[Type Error] expecting numeric type values.")),
    }
}

fn check_result_ok(exp: Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;
    return Ok(Type::TResult(Box::new(exp_type), Box::new(Type::TAny)));
}

fn check_result_err(exp: Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;
    return Ok(Type::TResult(Box::new(Type::TAny), Box::new(exp_type)));
}

fn check_unwrap_type(exp: Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;

    match exp_type {
        Type::TMaybe(t) => Ok(*t),
        Type::TResult(tl, _) => Ok(*tl),
        _ => Err(String::from(
            "[Type Error] expecting a maybe or result type value.",
        )),
    }
}

fn check_propagate_type(exp: Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;

    match exp_type {
        Type::TMaybe(t) => Ok(*t),
        Type::TResult(tl, _) => Ok(*tl),
        _ => Err(String::from(
            "[Type Error] expecting a maybe or result type value.",
        )),
    }
}

fn check_maybe_just(exp: Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;
    Ok(Type::TMaybe(Box::new(exp_type)))
}

fn check_iserror_type(exp: Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let v = check_expr(exp, env)?;

    match v {
        Type::TResult(_, _) => Ok(Type::TBool),
        _ => Err(String::from("[Type Error] expecting a result type value.")),
    }
}

fn check_isnothing_type(exp: Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;

    match exp_type {
        Type::TMaybe(_) => Ok(Type::TBool),
        _ => Err(String::from("[Type Error] expecting a maybe type value.")),
    }
}

fn merge_environments(
    env1: &Environment<Type>,
    env2: &Environment<Type>,
) -> Result<Environment<Type>, ErrorMessage> {
    let mut merged = env1.clone();

    // Get all variables defined in either environment
    for (name, type2) in env2.get_all_variables() {
        match env1.lookup(&name) {
            Some(type1) => {
                // Variable exists in both branches - types must match
                if *type1 != type2 {
                    return Err(format!(
                        "[Type Error] Variable '{}' has inconsistent types in different branches: '{:?}' and '{:?}'",
                        name, type1, type2
                    ));
                }
            }
            None => {
                // Variable only exists in else branch - it's conditionally defined
                // For now, we'll add it to the environment but might want to mark it as conditional
                merged.map_variable(name.clone(), type2.clone());
            }
        }
    }
    Ok(merged)
}

fn check_list_value(
    elements: &[Expression],
    env: &Environment<Type>,
) -> Result<Type, ErrorMessage> {
    if elements.is_empty() {
        return Ok(Type::TList(Box::new(Type::TAny)));
    }

    // Check the type of the first element
    let first_type = check_expr(elements[0].clone(), env)?;

    // Check that all other elements have the same type
    for element in elements.iter().skip(1) {
        let element_type = check_expr(element.clone(), env)?;
        if element_type != first_type {
            return Err(format!(
                "[Type Error] List elements must have the same type. Expected '{:?}', found '{:?}'.",
                first_type, element_type
            ));
        }
    }

    Ok(Type::TList(Box::new(first_type)))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::environment::environment::Environment;
    use crate::ir::ast::Expression::*;
    use crate::ir::ast::Function;
    use crate::ir::ast::Statement::*;
    use crate::ir::ast::Type::*;

    #[test]
    fn check_tlist_comparison() {
        let t_list1 = TList(Box::new(TInteger));
        let t_list2 = TList(Box::new(TInteger));

        assert_eq!(t_list1 == t_list2, true);
    }

    #[test]
    fn check_tlist_comparison_different_types() {
        let t_list1 = TList(Box::new(TInteger));
        let t_list2 = TList(Box::new(TBool));

        assert_eq!(t_list1 == t_list2, false);
    }

    #[test]
    fn check_ttuple_comparison() {
        let t_tuple1 = TTuple(vec![TInteger, TBool]);
        let t_tuple2 = TTuple(vec![TInteger, TBool]);

        assert_eq!(t_tuple1 == t_tuple2, true);
    }

    #[test]
    fn check_ttuple_comparison_different_types() {
        let t_tuple1 = TTuple(vec![TInteger, TBool]);
        let t_tuple2 = TTuple(vec![TBool, TInteger]);

        assert_eq!(t_tuple1 == t_tuple2, false);
    }

    #[test]
    fn check_constant() {
        let env = Environment::new();
        let c10 = CInt(10);

        assert_eq!(check_expr(c10, &env), Ok(TInteger));
    }

    #[test]
    fn check_add_integers() {
        let env = Environment::new();

        let c10 = CInt(10);
        let c20 = CInt(20);
        let add = Add(Box::new(c10), Box::new(c20));

        assert_eq!(check_expr(add, &env), Ok(TInteger));
    }

    #[test]
    fn check_add_reals() {
        let env = Environment::new();

        let c10 = CReal(10.5);
        let c20 = CReal(20.3);
        let add = Add(Box::new(c10), Box::new(c20));

        assert_eq!(check_expr(add, &env), Ok(TReal));
    }

    #[test]
    fn check_add_real_and_integer() {
        let env = Environment::new();

        let c10 = CInt(10);
        let c20 = CReal(20.3);
        let add = Add(Box::new(c10), Box::new(c20));

        assert_eq!(check_expr(add, &env), Ok(TReal));
    }

    #[test]
    fn check_add_integer_and_real() {
        let env = Environment::new();

        let c10 = CReal(10.5);
        let c20 = CInt(20);
        let add = Add(Box::new(c10), Box::new(c20));

        assert_eq!(check_expr(add, &env), Ok(TReal));
    }

    #[test]
    fn check_type_error_arithmetic_expression() {
        let env = Environment::new();

        let e1 = CInt(10);
        let e2 = CFalse;
        let e3 = Add(Box::new(e1), Box::new(e2));

        assert!(
            matches!(check_expr(e3, &env), Err(_)),
            "Expecting a type error."
        );
    }

    #[test]
    fn check_type_error_not_expression() {
        let env = Environment::new();

        let e1 = CInt(10);
        let e2 = Not(Box::new(e1));

        assert!(
            matches!(check_expr(e2, &env), Err(_)),
            "Expecting a type error."
        );
    }

    #[test]
    fn check_type_error_and_expression() {
        let env = Environment::new();

        let e1 = CInt(10);
        let e2 = CTrue;
        let e3 = And(Box::new(e1), Box::new(e2));

        assert!(
            matches!(check_expr(e3, &env), Err(_)),
            "Expecting a type error."
        );
    }

    #[test]
    fn check_type_error_or_expression() {
        let env = Environment::new();

        let e1 = CInt(10);
        let e2 = CTrue;
        let e3 = Or(Box::new(e1), Box::new(e2));

        assert!(
            matches!(check_expr(e3, &env), Err(_)),
            "Expecting a type error."
        );
    }

    #[test]
    fn check_ok_result() {
        let env = Environment::new();
        let e1 = CReal(10.0);
        let e2 = COk(Box::new(e1));

        assert_eq!(
            check_expr(e2, &env),
            Ok(TResult(Box::new(TReal), Box::new(TAny)))
        );
    }

    #[test]
    fn check_err_result() {
        let env = Environment::new();
        let e1 = CInt(1);
        let e2 = CErr(Box::new(e1));

        assert_eq!(
            check_expr(e2, &env),
            Ok(TResult(Box::new(TAny), Box::new(TInteger)))
        );
    }

    #[test]
    fn check_just_integer() {
        let env = Environment::new();
        let e1 = CInt(5);
        let e2 = CJust(Box::new(e1));

        assert_eq!(check_expr(e2, &env), Ok(TMaybe(Box::new(TInteger))))
    }

    #[test]
    fn check_is_error_result_positive() {
        let env = Environment::new();
        let e1 = CTrue;
        let e2 = COk(Box::new(e1));
        let e3 = IsError(Box::new(e2));

        assert_eq!(check_expr(e3, &env), Ok(TBool));
    }

    #[test]
    fn check_is_error_result_error() {
        let env = Environment::new();
        let e1 = CTrue;
        let e2 = IsError(Box::new(e1));

        assert!(
            matches!(check_expr(e2, &env), Err(_)),
            "Expecting a result type value."
        );
    }

    #[test]
    fn check_nothing() {
        let env = Environment::new();

        assert_eq!(check_expr(CNothing, &env), Ok(TMaybe(Box::new(TAny))));
    }

    #[test]
    fn check_is_nothing_on_maybe() {
        let env = Environment::new();
        let e1 = CInt(5);
        let e2 = CJust(Box::new(e1));
        let e3 = IsNothing(Box::new(e2));

        assert_eq!(check_expr(e3, &env), Ok(TBool));
    }

    #[test]
    fn check_is_nothing_type_error() {
        let env = Environment::new();
        let e1 = CInt(5);
        let e2 = IsNothing(Box::new(e1));

        assert!(
            matches!(check_expr(e2, &env), Err(_)),
            "expecting a maybe type value."
        );
    }

    #[test]
    fn check_unwrap_maybe() {
        let env = Environment::new();
        let e1 = CInt(5);
        let e2 = CJust(Box::new(e1));
        let e3 = Unwrap(Box::new(e2));

        assert_eq!(check_expr(e3, &env), Ok(TInteger));
    }

    #[test]
    fn check_unwrap_maybe_type_error() {
        let env = Environment::new();
        let e1 = CInt(5);
        let e2 = Unwrap(Box::new(e1));

        assert!(
            matches!(check_expr(e2, &env), Err(_)),
            "expecting a maybe or result type value."
        );
    }

    #[test]
    fn check_unwrap_result() {
        let env = Environment::new();
        let e1 = CTrue;
        let e2 = COk(Box::new(e1));
        let e3 = Unwrap(Box::new(e2));

        assert_eq!(check_expr(e3, &env), Ok(TBool));
    }

    #[test]
    fn check_propagate_maybe() {
        let env = Environment::new();
        let e1 = CInt(5);
        let e2 = CJust(Box::new(e1));
        let e3 = Propagate(Box::new(e2));

        assert_eq!(check_expr(e3, &env), Ok(TInteger));
    }

    #[test]
    fn check_propagate_maybe_type_error() {
        let env = Environment::new();
        let e1 = CInt(5);
        let e2 = Propagate(Box::new(e1));

        assert!(
            matches!(check_expr(e2, &env), Err(_)),
            "expecting a maybe or result type value."
        );
    }

    #[test]
    fn check_propagate_result() {
        let env = Environment::new();
        let e1 = CTrue;
        let e2 = COk(Box::new(e1));
        let e3 = Propagate(Box::new(e2));

        assert_eq!(check_expr(e3, &env), Ok(TBool));
    }

    #[test]
    fn check_assignment() {
        let env: Environment<Type> = Environment::new();

        let assignment = Assignment("a".to_string(), Box::new(CTrue));

        match check_stmt(assignment, &env) {
            Ok(_) => assert!(true),
            Err(s) => assert!(false, "{}", s),
        }
    }

    #[test]
    fn check_assignment_error2() {
        let env: Environment<Type> = Environment::new();

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
            Ok(new_env) => assert!(true),
            Err(s) => assert!(false, "{}", s),
        }
    }

    #[test]
    fn test_if_else_consistent_types() {
        let env = Environment::new();
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
    fn test_undefined_variable() {
        let env = Environment::new();
        let exp = Expression::Var("x".to_string());

        // Should fail - x is not defined
        assert!(check_expr(exp, &env).is_err());
    }

    #[test]
    fn test_defined_variable() {
        let mut env = Environment::new();
        env.map_variable("x".to_string(), Type::TInteger);
        let exp = Expression::Var("x".to_string());

        // Should succeed and return integer type
        assert_eq!(check_expr(exp, &env), Ok(Type::TInteger));
    }

    #[test]
    fn test_variable_assignment() {
        let env = Environment::new();
        let stmt = Statement::Assignment("x".to_string(), Box::new(Expression::CInt(42)));

        // Should succeed and add x:integer to environment
        let new_env = check_stmt(stmt, &env).unwrap();
        assert_eq!(new_env.lookup(&"x".to_string()), Some(&Type::TInteger));
    }

    #[test]
    fn test_variable_reassignment_same_type() {
        let mut env = Environment::new();
        env.map_variable("x".to_string(), Type::TInteger);

        let stmt = Statement::Assignment("x".to_string(), Box::new(Expression::CInt(100)));

        // Should succeed - reassigning same type
        assert!(check_stmt(stmt, &env).is_ok());
    }

    #[test]
    fn test_variable_reassignment_different_type() {
        let mut env = Environment::new();
        env.map_variable("x".to_string(), Type::TInteger);

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

        let local_func = Function {
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
        env.map_variable("sum".to_string(), Type::TInteger);
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
        env.map_variable("x".to_string(), Type::TString); // x is defined as string in outer scope

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
