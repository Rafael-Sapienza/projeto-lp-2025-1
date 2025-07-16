use crate::models::{Block, Input, Value, Workspace};
use actix_web::{HttpResponse, Responder, web};
use serde_json::Value as JsonValue;

pub async fn execute(payload: web::Json<Workspace>) -> impl Responder {
    let mut output = Vec::<String>::new();

    for block in &payload.blocks.blocks {
        execute_sequence(block, &mut output);
    }

    //output = process_blocks(blocks_only);
    HttpResponse::Ok().json(output)
}

// TODO: Maybe here is the place to return Possible errors
// TODO: Create an interpreter struct to avoid the repetitive/unnecessary passing of the output vector
// Interpreter struct has the output vector as an attribute
// Interpreter struct has execute_sequence() and execute_block() as impl methods
fn execute_sequence(top_block: &Block, output: &mut Vec<String>) {
    let mut current_block = Some(top_block);

    while let Some(sub_block) = current_block {
        if let Some(Value::String(s)) = execute_block(sub_block, output) {
            if !s.is_empty() {
                output.push(s);
            }
        }

        current_block = match sub_block.next.as_ref() {
            Some(b) => Some(&*b.block),
            None => None,
        }
    }
}

fn execute_block(block: &Block, output: &mut Vec<String>) -> Option<Value> {
    match block.r#type.as_str() {
        "print" => {
            let mut text = String::new();
            if let Some(inputs) = &block.inputs {
                if let Some(input) = inputs.get("TEXT") {
                    text = match get_string_input(input, output) {
                        Some(s) => s,
                        None => "".to_string(),
                    }
                }
            }

            Some(Value::String(text))
        }
        "join" => {
            let mut left_text = String::new();
            let mut right_text = String::new();

            if let Some(inputs) = &block.inputs {
                if let Some(input) = inputs.get("TEXT1") {
                    left_text = match get_string_input(input, output) {
                        Some(s) => s,
                        None => "".to_string(),
                    }
                }
            }

            if let Some(inputs) = &block.inputs {
                if let Some(input) = inputs.get("TEXT2") {
                    right_text = match get_string_input(input, output) {
                        Some(s) => s,
                        None => "".to_string(),
                    }
                }
            }

            Some(Value::String(format!("{}{}", left_text, right_text)))
        }
        "length" => {
            let mut total: f64 = 0.0;
            if let Some(inputs) = &block.inputs {
                if let Some(input) = inputs.get("TEXT") {
                    if let Some(s) = get_string_input(input, output) {
                        total = s.len() as f64;
                    }
                }
            }

            Some(Value::Number(total))
        }
        "sum" => {
            let mut num1: f64 = 0.0;
            let mut num2: f64 = 0.0;

            if let Some(inputs) = &block.inputs {
                if let Some(input) = inputs.get("NUM1") {
                    num1 = match get_number_input(input, output) {
                        Some(n) => n,
                        None => 0.0,
                    }
                }
            }

            if let Some(inputs) = &block.inputs {
                if let Some(input) = inputs.get("NUM2") {
                    num2 = match get_number_input(input, output) {
                        Some(n) => n,
                        None => 0.0,
                    }
                }
            }

            Some(Value::Number(num1 + num2))
        }
        "sub" => {
            let mut num1: f64 = 0.0;
            let mut num2: f64 = 0.0;

            if let Some(inputs) = &block.inputs {
                if let Some(input) = inputs.get("NUM1") {
                    num1 = match get_number_input(input, output) {
                        Some(n) => n,
                        None => 0.0,
                    }
                }
            }

            if let Some(inputs) = &block.inputs {
                if let Some(input) = inputs.get("NUM2") {
                    num2 = match get_number_input(input, output) {
                        Some(n) => n,
                        None => 0.0,
                    }
                }
            }

            Some(Value::Number(num1 - num2))
        }
        "mult" => {
            let mut num1: f64 = 0.0;
            let mut num2: f64 = 0.0;

            if let Some(inputs) = &block.inputs {
                if let Some(input) = inputs.get("NUM1") {
                    num1 = match get_number_input(input, output) {
                        Some(n) => n,
                        None => 0.0,
                    }
                }
            }

            if let Some(inputs) = &block.inputs {
                if let Some(input) = inputs.get("NUM2") {
                    num2 = match get_number_input(input, output) {
                        Some(n) => n,
                        None => 0.0,
                    }
                }
            }

            Some(Value::Number(num1 * num2))
        }
        "divi" => {
            let mut num1: f64 = 0.0;
            let mut num2: f64 = 0.0;

            if let Some(inputs) = &block.inputs {
                if let Some(input) = inputs.get("NUM1") {
                    num1 = match get_number_input(input, output) {
                        Some(n) => n,
                        None => 0.0,
                    }
                }
            }

            if let Some(inputs) = &block.inputs {
                if let Some(input) = inputs.get("NUM2") {
                    num2 = match get_number_input(input, output) {
                        Some(n) => n,
                        None => 0.0,
                    }
                }
            }

            Some(Value::Number(num1 / num2))
        }
        "bigger" => {
            let mut num1: f64 = 0.0;
            let mut num2: f64 = 0.0;

            if let Some(inputs) = &block.inputs {
                if let Some(input) = inputs.get("NUM1") {
                    num1 = get_number_input(input, output).unwrap_or(0.0);
                }
                if let Some(input) = inputs.get("NUM2") {
                    num2 = get_number_input(input, output).unwrap_or(0.0);
                }
            }

            Some(Value::Boolean(num1 > num2))
        }
        "smaller" => {
            let mut num1: f64 = 0.0;
            let mut num2: f64 = 0.0;

            if let Some(inputs) = &block.inputs {
                if let Some(input) = inputs.get("NUM1") {
                    num1 = get_number_input(input, output).unwrap_or(0.0);
                }
                if let Some(input) = inputs.get("NUM2") {
                    num2 = get_number_input(input, output).unwrap_or(0.0);
                }
            }

            Some(Value::Boolean(num1 < num2))
        }
        "equal" => {
            let mut num1: f64 = 0.0;
            let mut num2: f64 = 0.0;

            if let Some(inputs) = &block.inputs {
                if let Some(input) = inputs.get("NUM1") {
                    num1 = get_number_input(input, output).unwrap_or(0.0);
                }
                if let Some(input) = inputs.get("NUM2") {
                    num2 = get_number_input(input, output).unwrap_or(0.0);
                }
            }

            Some(Value::Boolean((num1 - num2).abs() < std::f64::EPSILON))
        }
        "if" => {
            let mut condition_met = false;
            if let Some(inputs) = &block.inputs {
                if let Some(input) = inputs.get("CONDITION") {
                    if let Some(b) = get_boolean_input(input, output) {
                        condition_met = b;
                    }
                }

                // Call the execute_sequence function with the top block of the DO input
                if condition_met {
                    if let Some(input) = inputs.get("DO") {
                        if let Some(sub_block) = &input.block {
                            execute_sequence(&sub_block, output);
                        }
                    }
                }
            }

            Some(Value::String("".to_string())) // Here just for error handling, for now
        }
        "text_shadow" => {
            if let Some(s) = get_text_shadow(block) {
                Some(Value::String(s))
            } else {
                Some(Value::String("".to_string()))
            }
        }
        "number_shadow" => {
            if let Some(n) = get_number_shadow(block) {
                Some(Value::Number(n))
            } else {
                None // TODO: Create a Value::Error to handle the invalid cases
            }
        }
        _ => None,
    }
}

fn get_string_input(input: &Input, output: &mut Vec<String>) -> Option<String> {
    let value = if let Some(sub_block) = &input.block {
        execute_block(sub_block, output)
    } else if let Some(shadow_block) = &input.shadow {
        execute_block(shadow_block, output)
    } else {
        None
    };

    match value {
        Some(found_value) => match found_value {
            Value::String(s) => Some(s),
            Value::Number(n) => Some(n.to_string()),
            Value::Boolean(b) => Some(b.to_string()),
        },
        None => None,
    }
}

fn get_number_input(input: &Input, output: &mut Vec<String>) -> Option<f64> {
    let value = if let Some(sub_block) = &input.block {
        execute_block(sub_block, output)
    } else if let Some(shadow_block) = &input.shadow {
        execute_block(shadow_block, output)
    } else {
        None
    };

    match value {
        Some(Value::Number(n)) => Some(n as f64),
        _ => None,
    }
}

fn get_boolean_input(input: &Input, output: &mut Vec<String>) -> Option<bool> {
    let value = if let Some(sub_block) = &input.block {
        execute_block(sub_block, output)
    } else {
        None
    };

    match value {
        Some(Value::Boolean(b)) => Some(b),
        _ => None,
    }
}

fn get_number_shadow(block: &Block) -> Option<f64> {
    if let Some(fields) = &block.fields {
        if let Some(value) = fields.get("NUM") {
            return match value {
                JsonValue::String(s) => s.parse::<f64>().ok(),
                JsonValue::Number(n) => n.as_f64(),
                _ => None,
            };
        }
    }
    None
}

fn get_text_shadow(block: &Block) -> Option<String> {
    if let Some(fields) = &block.fields {
        if let Some(value) = fields.get("TEXT") {
            if let JsonValue::String(s) = value {
                if !s.is_empty() {
                    return Some(s.to_string());
                }
            }
        }
    }
    None
}
