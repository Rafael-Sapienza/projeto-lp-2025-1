use super::serialization::{Block, Input, Value}; use serde_json::Value as JsonValue;
use std::collections::HashMap;

pub struct EasyInterpreter {
    output: Vec<String>,
    variables: HashMap<String, Value>,
    functions: HashMap<String, Block>,
}

impl EasyInterpreter {
    pub fn new() -> Self {
        EasyInterpreter {
            output: Vec::new(),
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    /// Run all top‑level blocks that came from the frontend.
    pub fn run(&mut self, blocks: &[Block]) {
        for block in blocks {
            if block.r#type.starts_with("function_def_") {
                self.functions.insert(block.r#type.clone(), block.clone());
            }
        }

        for block in blocks {
                if block.r#type == "easy_run" {
                    self.execute_sequence(block);
                    break;
                }
            }
    }

    /// Consume the interpreter and give the accumulated output back.
    pub fn into_output(self) -> Vec<String> {
        self.output
    }


    fn execute_sequence(&mut self, top_block: &Block) {
        let mut current_block = Some(top_block);

        while let Some(sub_block) = current_block {
            if let Some(Value::String(s)) = self.execute_block(sub_block) {
                if !s.is_empty() {
                    self.output.push(s);
                }
            }

            current_block = match sub_block.next.as_ref() {
                Some(b) => Some(&*b.block),
                None => None,
            };
        }
    }

    fn execute_block(&mut self, block: &Block) -> Option<Value> {
        match block.r#type.as_str() {

            "variables_set_number" => {
                if let (Some(fields), Some(inputs)) = (&block.fields, &block.inputs) {
                    if let (Some(var_field), Some(input)) = (fields.get("VAR"), inputs.get("NUM")) {
                        if let Some(var_id) = var_field.get("id").and_then(|v| v.as_str()) {
                            if let Some(num) = self.get_number_input(input) {
                                self.variables.insert(var_id.to_string(), Value::Number(num));
                            }
                        }
                    }
                }
                Some(Value::String(String::new()))
            }

            "variables_set_string" => {
                if let (Some(fields), Some(inputs)) = (&block.fields, &block.inputs) {
                    if let (Some(var_field), Some(input)) = (fields.get("VAR"), inputs.get("TEXT")) {
                        if let Some(var_id) = var_field.get("id").and_then(|v| v.as_str()) {
                            if let Some(text) = self.get_string_input(input) {
                                self.variables.insert(var_id.to_string(), Value::String(text));
                            }
                        }
                    }
                }
                Some(Value::String(String::new()))
            }
            "variables_get_number" | "variables_get_string" => {
                if let Some(fields) = &block.fields {
                    if let Some(var_field) = fields.get("VAR") {
                        if let Some(var_id) = var_field.get("id").and_then(|v| v.as_str()) {
                            return self.variables.get(var_id).cloned();
                        }
                    }
                }
                None
            }
            other_type if other_type.starts_with("function_set_") => {
                self.execute_user_function(block)
            }
            other_type if other_type.starts_with("function_param_get_") => {
                let normalized = other_type.replace(' ', "_");
                self.variables.get(&normalized).cloned()
            }
            "print" => {
                let mut text = String::new();
                if let Some(inputs) = &block.inputs {
                    if let Some(input) = inputs.get("TEXT") {
                        if let Some(text) = self.get_string_input(input) {
                            self.output.push(text);
                        }
                    }
                }
                Some(Value::String(String::new()))
            }
            "join" => {
                let mut left_text = String::new();
                let mut right_text = String::new();

                if let Some(inputs) = &block.inputs {
                    if let Some(input) = inputs.get("TEXT1") {
                        left_text = self.get_string_input(input).unwrap_or_else(|| "".to_string());
                    }
                    if let Some(input) = inputs.get("TEXT2") {
                        right_text = self.get_string_input(input).unwrap_or_else(|| "".to_string());
                    }
                }

                Some(Value::String(format!("{}{}", left_text, right_text)))
            }
            "number_to_text" => {
                if let Some(inputs) = &block.inputs {
                    if let Some(input) = inputs.get("NUM") {
                        if let Some(num) = self.get_number_input(input) {
                            return Some(Value::String(num.to_string()));
                        }
                    }
                }
                Some(Value::String(String::new()))
            }
            "compare_texts" => {
                if let Some(inputs) = &block.inputs {
                    let left = inputs.get("TEXT1").and_then(|i| self.get_string_input(i)).unwrap_or_default();
                    let right = inputs.get("TEXT2").and_then(|i| self.get_string_input(i)).unwrap_or_default();
                    return Some(Value::Boolean(left == right));
                }
                Some(Value::Boolean(false))
            }
            "length" => {
                let mut total: f64 = 0.0;
                if let Some(inputs) = &block.inputs {
                    if let Some(input) = inputs.get("TEXT") {
                        if let Some(s) = self.get_string_input(input) {
                            total = s.len() as f64;
                        }
                    }
                }
                Some(Value::Number(total))
            }

            "sum" | "sub" | "mult" | "divi" => {
                let mut num1 = 0.0;
                let mut num2 = 0.0;

                if let Some(inputs) = &block.inputs {
                    if let Some(input) = inputs.get("NUM1") {
                        num1 = self.get_number_input(input).unwrap_or(0.0);
                    }
                    if let Some(input) = inputs.get("NUM2") {
                        num2 = self.get_number_input(input).unwrap_or(0.0);
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

            "bigger" | "smaller" | "equal" | "less_equal" | "greater_equal" => {
                let mut num1 = 0.0;
                let mut num2 = 0.0;

                if let Some(inputs) = &block.inputs {
                    if let Some(input) = inputs.get("NUM1") {
                        num1 = self.get_number_input(input).unwrap_or(0.0);
                    }
                    if let Some(input) = inputs.get("NUM2") {
                        num2 = self.get_number_input(input).unwrap_or(0.0);
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

            "if" => {
                let mut condition_met = false;

                if let Some(inputs) = &block.inputs {
                    if let Some(input) = inputs.get("CONDITION") {
                        if let Some(b) = self.get_boolean_input(input) {
                            condition_met = b;
                        }
                    }

                    if condition_met {
                        if let Some(input) = inputs.get("DO") {
                            if let Some(sub_block) = &input.block {
                                self.execute_sequence(sub_block);
                            }
                        }
                    }
                }

                Some(Value::String(String::new()))
            }
            "if_else" => {
                if let Some(inputs) = &block.inputs {
                    let mut condition_met = false;

                    if let Some(condition_input) = inputs.get("CONDITION") {
                        if let Some(b) = self.get_boolean_input(condition_input) {
                            condition_met = b;
                        }
                    }

                    let selected_branch = if condition_met {
                        inputs.get("DO")
                    } else {
                        inputs.get("ELSE")
                    };

                    if let Some(Some(branch_block)) = selected_branch.map(|input| input.block.as_ref()) {
                        self.execute_sequence(branch_block);
                    }
                }

                Some(Value::String(String::new()))
            }
            "repeat" => {
                if let Some(inputs) = &block.inputs {
                    let times = inputs.get("TIMES")
                        .and_then(|input| self.get_number_input(input))
                        .unwrap_or(0.0) as usize;

                    for _ in 0..times {
                        if let Some(input) = inputs.get("DO") {
                            if let Some(sub_block) = &input.block {
                                self.execute_sequence(sub_block);
                            }
                        }
                    }
                }
                Some(Value::String(String::new()))
            }
            "while" => {
                if let Some(inputs) = &block.inputs {
                    let mut guard = 0; // Optional: prevent infinite loops
                    while inputs.get("CONDITION")
                        .and_then(|input| self.get_boolean_input(input))
                        .unwrap_or(false)
                    {
                        if let Some(input) = inputs.get("DO") {
                            if let Some(sub_block) = &input.block {
                                self.execute_sequence(sub_block);
                            }
                        }

                        guard += 1;
                        if guard > 10000 {
                            eprintln!("Infinite loop protection triggered.");
                            break;
                        }
                    }
                }
                Some(Value::String(String::new()))
            }
            "text_shadow" => {
                Some(Value::String(self.get_text_shadow(block).unwrap_or_else(|| "".to_string())))
            }

            "number_shadow" => {
                self.get_number_shadow(block).map(Value::Number)
            }

            _ => None,
        }
    }

    /* ---------- helpers (unchanged style) ---------- */

    fn get_string_input(&mut self, input: &Input) -> Option<String> {
        let value = if let Some(sub_block) = &input.block {
            self.execute_block(sub_block)
        } else if let Some(shadow_block) = &input.shadow {
            self.execute_block(shadow_block)
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

    fn get_number_input(&mut self, input: &Input) -> Option<f64> {
        let value = if let Some(sub_block) = &input.block {
            self.execute_block(sub_block)
        } else if let Some(shadow_block) = &input.shadow {
            self.execute_block(shadow_block)
        } else {
            None
        };

        match value {
            Some(Value::Number(n)) => Some(n),
            _ => None,
        }
    }

    fn get_boolean_input(&mut self, input: &Input) -> Option<bool> {
        let value = if let Some(sub_block) = &input.block {
            self.execute_block(sub_block)
        } else {
            None
        };

        match value {
            Some(Value::Boolean(b)) => Some(b),
            _ => None,
        }
    }

    fn get_number_shadow(&self, block: &Block) -> Option<f64> {
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

    fn get_text_shadow(&self, block: &Block) -> Option<String> {
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


    fn execute_user_function(&mut self, block: &Block) -> Option<Value> {
        let function_name = block.r#type.replacen("function_set_", "function_def_", 1);
        let function_def = self.functions.get(&function_name)?.clone();

        let mut local_vars = HashMap::new();

        // 1. Pass arguments to local variables
        if let Some(input_map) = &block.inputs {
            for (input_name, input) in input_map {
                let value = self.execute_input(input);
                // Param names in definition: "function_param_get_<function_name>_<input_name>"
                let var_block_type = format!("function_param_get_{}_{}", 
                    function_name.strip_prefix("function_def_").unwrap_or(""),
                    input_name);

                if let Some(Value::String(val)) = &value {
                    local_vars.insert(var_block_type.clone(), Value::String(val.clone()));
                } else if let Some(Value::Number(val)) = &value {
                    local_vars.insert(var_block_type.clone(), Value::Number(*val));
                } else if let Some(Value::Boolean(val)) = &value {
                    local_vars.insert(var_block_type.clone(), Value::Boolean(*val));
                }
            }
        }

        // 2. Save current scope
        let old_variables = std::mem::replace(&mut self.variables, local_vars);

        // 3. Execute BODY
        if let Some(inputs) = &function_def.inputs {
            if let Some(body_input) = inputs.get("BODY") {
                if let Some(body_block) = &body_input.block {
                    self.execute_sequence(body_block);
                }
            }
        }

        // 4. Extract RETURN
        let return_value = if let Some(inputs) = &function_def.inputs {
            if let Some(ret_input) = inputs.get("RETURN") {
                self.execute_input(ret_input)
            } else {
                None
            }
        } else {
            None
        };

        // 5. Restore original scope
        self.variables = old_variables;

        // 6. Execute next block, if present
        /*if let Some(next_block) = &block.next {
            self.execute_sequence(&next_block.block);
        }*/

        return_value
    }

    fn execute_input(&mut self, input: &Input) -> Option<Value> {
        if let Some(block) = &input.block {
            self.execute_block(block)
        } else if let Some(shadow) = &input.shadow {
            self.execute_block(shadow)
        } else {
            None
        }
    }
}
