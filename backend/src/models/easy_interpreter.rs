use super::BlockExecutor;
use super::{Block, Input, Value};
use super::{
    get_boolean_input, get_number_input, get_number_shadow, get_string_input, get_text_shadow,
};
use crate::models::sub_interpreters::control::{check_if, check_if_else, repeat, repeat_while};
use crate::models::sub_interpreters::math::{handle_math_comparisons, handle_math_operations};
use crate::models::sub_interpreters::text::{compare_texts, join, num_to_text, print, text_length};
use crate::models::sub_interpreters::variables::{get_variable, set_variable};
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

    fn exec_sequence(&mut self, block: &Block) {
        self.execute_sequence(block);
    }

    fn push_output(&mut self, text: String) {
        self.output.push(text);
    }

    fn set_variable(&mut self, id: &str, value: Value) {
        self.variables.insert(id.to_string(), value);
    }

    fn get_variable(&mut self, id: &str) -> Option<Value> {
        self.variables.get(id).cloned()
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
            "variables_set_number" | "variables_set_string" => set_variable(self, block),

            "variables_get_number" | "variables_get_string" => get_variable(self, block),

            function_set if function_set.starts_with("function_set_") => {
                self.execute_user_function(block)
            }

            function_param_get if function_param_get.starts_with("function_param_get_") => {
                let normalized = function_param_get.replace(' ', "_");
                self.variables.get(&normalized).cloned()
            }

            "print" => print(self, block),

            "join" => join(self, block),

            "number_to_text" => num_to_text(self, block),

            "compare_texts" => compare_texts(self, block),

            "length" => text_length(self, block),

            "sum" | "sub" | "mult" | "divi" => handle_math_operations(self, block),

            "bigger" | "smaller" | "equal" | "less_equal" | "greater_equal" => {
                handle_math_comparisons(self, block)
            }

            "if" => check_if(self, block),

            "if_else" => check_if_else(self, block),

            "repeat" => repeat(self, block),

            "while" => repeat_while(self, block),

            "text_shadow" => Some(Value::String(
                get_text_shadow(block).unwrap_or_else(|| "".to_string()),
            )),

            "number_shadow" => get_number_shadow(block).map(Value::Number),

            _ => None,
        }
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
                let var_block_type = format!(
                    "function_param_get_{}_{}",
                    function_name.strip_prefix("function_def_").unwrap_or(""),
                    input_name
                );

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
