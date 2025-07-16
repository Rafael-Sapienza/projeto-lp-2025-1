use crate::models::helper_functions::{get_number_input, get_string_input};
use crate::models::{Block, BlockExecutor, Value};

pub fn print(executor: &mut dyn BlockExecutor, block: &Block) -> Option<Value> {
    if let Some(inputs) = &block.inputs {
        if let Some(input) = inputs.get("TEXT") {
            if let Some(text) = get_string_input(executor, input) {
                executor.push_output(text);
            }
        }
    }
    Some(Value::String(String::new()))
}

pub fn join(executor: &mut dyn BlockExecutor, block: &Block) -> Option<Value> {
    let mut left_text = String::new();
    let mut right_text = String::new();

    if let Some(inputs) = &block.inputs {
        if let Some(input) = inputs.get("TEXT1") {
            left_text = get_string_input(executor, input).unwrap_or_else(|| "".to_string());
        }
        if let Some(input) = inputs.get("TEXT2") {
            right_text = get_string_input(executor, input).unwrap_or_else(|| "".to_string());
        }
    }

    Some(Value::String(format!("{}{}", left_text, right_text)))
}

pub fn num_to_text(executor: &mut dyn BlockExecutor, block: &Block) -> Option<Value> {
    if let Some(inputs) = &block.inputs {
        if let Some(input) = inputs.get("NUM") {
            if let Some(num) = get_number_input(executor, input) {
                return Some(Value::String(num.to_string()));
            }
        }
    }
    Some(Value::String(String::new()))
}

pub fn compare_texts(executor: &mut dyn BlockExecutor, block: &Block) -> Option<Value> {
    if let Some(inputs) = &block.inputs {
        let left = inputs
            .get("TEXT1")
            .and_then(|i| get_string_input(executor, i))
            .unwrap_or_default();
        let right = inputs
            .get("TEXT2")
            .and_then(|i| get_string_input(executor, i))
            .unwrap_or_default();
        return Some(Value::Boolean(left == right));
    }
    Some(Value::Boolean(false))
}

pub fn text_length(executor: &mut dyn BlockExecutor, block: &Block) -> Option<Value> {
    let mut total: f64 = 0.0;
    if let Some(inputs) = &block.inputs {
        if let Some(input) = inputs.get("TEXT") {
            if let Some(s) = get_string_input(executor, input) {
                total = s.len() as f64;
            }
        }
    }
    Some(Value::Number(total))
}
