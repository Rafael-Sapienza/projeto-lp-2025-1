use super::{Block, Input, Value}; 
use super::BlockExecutor; 
use super::{get_string_input, get_number_input, get_boolean_input, get_number_shadow, get_text_shadow};
use std::collections::HashMap;

pub struct EasyInterpreter {
    output: Vec<String>,
    variables: HashMap<String, Value>,
    functions: HashMap<String, Block>,
}

impl BlockExecutor for EasyInterpreter {
    fn exec_block(&mut self, block: &Block) -> Option<Value> { 
        self.execute_block(block)
    }

    fn exec_input(&mut self, input: &Input) -> Option<Value> {
        self.execute_input(input)
    }
}

impl EasyInterpreter {
    pub fn new() -> Self {
        EasyInterpreter {
            output: Vec::new(),
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    // Run all top‑level blocks that came from the frontend.
    pub fn run(&mut self, blocks: &[Block]) {
        // Store function definition blocks
        for block in blocks {
            if block.r#type.starts_with("function_def_") {
                self.functions.insert(block.r#type.clone(), block.clone());
            }
        }
        // Find/execute the main function
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
                            if let Some(num) = get_number_input(self, input) {
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
                            if let Some(text) = get_string_input(self, input) {
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
                        if let Some(text) = get_string_input(self, input) {
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
                        left_text = get_string_input(self, input).unwrap_or_else(|| "".to_string());
                    }
                    if let Some(input) = inputs.get("TEXT2") {
                        right_text = get_string_input(self, input).unwrap_or_else(|| "".to_string());
                    }
                }

                Some(Value::String(format!("{}{}", left_text, right_text)))
            }
            "number_to_text" => {
                if let Some(inputs) = &block.inputs {
                    if let Some(input) = inputs.get("NUM") {
                        if let Some(num) = get_number_input(self, input) {
                            return Some(Value::String(num.to_string()));
                        }
                    }
                }
                Some(Value::String(String::new()))
            }
            "compare_texts" => {
                if let Some(inputs) = &block.inputs {
                    let left = inputs.get("TEXT1").and_then(|i| get_string_input(self, i)).unwrap_or_default();
                    let right = inputs.get("TEXT2").and_then(|i| get_string_input(self, i)).unwrap_or_default();
                    return Some(Value::Boolean(left == right));
                }
                Some(Value::Boolean(false))
            }
            "length" => {
                let mut total: f64 = 0.0;
                if let Some(inputs) = &block.inputs {
                    if let Some(input) = inputs.get("TEXT") {
                        if let Some(s) = get_string_input(self, input) {
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
                        num1 = get_number_input(self, input).unwrap_or(0.0);
                    }
                    if let Some(input) = inputs.get("NUM2") {
                        num2 = get_number_input(self, input).unwrap_or(0.0);
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
                        num1 = get_number_input(self, input).unwrap_or(0.0);
                    }
                    if let Some(input) = inputs.get("NUM2") {
                        num2 = get_number_input(self, input).unwrap_or(0.0);
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
                        if let Some(b) = get_boolean_input(self, input) {
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
                        if let Some(b) = get_boolean_input(self, condition_input) {
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
                        .and_then(|input| get_number_input(self, input))
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
                        .and_then(|input| get_boolean_input(self, input))
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
                Some(Value::String(get_text_shadow(block).unwrap_or_else(|| "".to_string())))
            }

            "number_shadow" => {
                get_number_shadow(block).map(Value::Number)
            }

            _ => None,
        }
    }

    /* ---------- helpers ---------- */



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
