use crate::models::{Block, BlockExecutor, Value};
use crate::models::helper_functions::get_number_input;

pub fn handle_math_operations(executor: &mut dyn BlockExecutor, block: &Block) -> Option<Value> {
    let mut num1 = 0.0;
    let mut num2 = 0.0;

    if let Some(inputs) = &block.inputs {
        if let Some(input) = inputs.get("NUM1") {
            num1 = get_number_input(executor, input).unwrap_or(0.0);
        }
        if let Some(input) = inputs.get("NUM2") {
            num2 = get_number_input(executor, input).unwrap_or(0.0);
        }
    }

    let result = match block.r#type.as_str() {
        "sum"  => num1 + num2,
        "sub"  => num1 - num2,
        "mult" => num1 * num2,
        "divi" => num1 / num2,
        _ => unreachable!(),
    };

    Some(Value::Number(result))
}


pub fn handle_math_comparisons(executor: &mut dyn BlockExecutor, block: &Block) -> Option<Value> {
    let mut num1 = 0.0;
    let mut num2 = 0.0;

    if let Some(inputs) = &block.inputs {
        if let Some(input) = inputs.get("NUM1") {
            num1 = get_number_input(executor, input).unwrap_or(0.0);
        }
        if let Some(input) = inputs.get("NUM2") {
            num2 = get_number_input(executor, input).unwrap_or(0.0);
        }
    }

    let result = match block.r#type.as_str() {
        "bigger"  => num1 > num2,
        "smaller" => num1 < num2,
        "equal"   => (num1 - num2).abs() < std::f64::EPSILON,
        "less_equal"  => num1 <= num2,
        "greater_equal" => num1 >= num2,
        _ => unreachable!(),
    };

    Some(Value::Boolean(result))
}
