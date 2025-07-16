use crate::models::helper_functions::{get_number_input, get_string_input};
use crate::models::{Block, BlockExecutor, Value};

pub fn set_variable(executor: &mut dyn BlockExecutor, block: &Block) -> Option<Value> {
    if let (Some(fields), Some(inputs)) = (&block.fields, &block.inputs) {
        if let (Some(var_field), Some(input)) =
            (fields.get("VAR"), inputs.get("NUM").or(inputs.get("TEXT")))
        {
            if let Some(var_id) = var_field.get("id").and_then(|v| v.as_str()) {
                if let Some(num) = get_number_input(executor, input) {
                    executor.set_variable(var_id, Value::Number(num));
                } else if let Some(text) = get_string_input(executor, input) {
                    executor.set_variable(var_id, Value::String(text));
                }
            }
        }
    }

    Some(Value::String(String::new()))
}

pub fn get_variable(executor: &mut dyn BlockExecutor, block: &Block) -> Option<Value> {
    if let Some(fields) = &block.fields {
        if let Some(var_field) = fields.get("VAR") {
            if let Some(var_id) = var_field.get("id").and_then(|v| v.as_str()) {
                //return self.variables.get(var_id).cloned();
                return executor.get_variable(var_id);
            }
        }
    }
    None
}
