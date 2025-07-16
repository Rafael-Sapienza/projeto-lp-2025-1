use super::{ BlockExecutor, Input, Value, Block};
use serde_json::Value as JsonValue;

pub fn get_string_input(executor: &mut dyn BlockExecutor, input: &Input) -> Option<String> {
    let value = if let Some(sub_block) = &input.block {
        executor.exec_block(sub_block)
    } else if let Some(shadow_block) = &input.shadow {
        executor.exec_block(shadow_block)
    } else {
        None
    };

    match value {
        Some(Value::String(s))  => Some(s),
        Some(Value::Number(n))  => Some(n.to_string()),
        Some(Value::Boolean(b)) => Some(b.to_string()),
        _ => None,
    }
}

pub fn get_number_input(executor: &mut dyn BlockExecutor, input: &Input) -> Option<f64> {
    let value = if let Some(sub_block) = &input.block {
        executor.exec_block(sub_block)
    } else if let Some(shadow_block) = &input.shadow {
        executor.exec_block(shadow_block)
    } else {
        None
    };

    match value {
        Some(Value::Number(n)) => Some(n),
        _ => None,
    }
}

pub fn get_boolean_input(executor: &mut dyn BlockExecutor, input: &Input) -> Option<bool> {
    let value = if let Some(sub_block) = &input.block {
        executor.exec_block(sub_block)
    } else {
        None
    };

    match value {
        Some(Value::Boolean(b)) => Some(b),
        _ => None,
    }
}

pub fn get_number_shadow(block: &Block) -> Option<f64> {
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

pub fn get_text_shadow(block: &Block) -> Option<String> {
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
