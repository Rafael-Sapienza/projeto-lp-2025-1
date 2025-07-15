//! models/easy_interpreter.rs
use super::serialization::{Block, Input, Value};
use serde_json::Value as JsonValue;

pub struct EasyInterpreter {
    output: Vec<String>,
    // Later: variables: HashMap<String, Value>,
    // Later: functions: HashMap<String, Vec<Block>>,
}

impl EasyInterpreter {
    pub fn new() -> Self {
        EasyInterpreter {
            output: Vec::new(),
        }
    }

    /// Run all top‑level blocks that came from the frontend.
    pub fn run(&mut self, blocks: &[Block]) {
        for block in blocks {
            self.execute_sequence(block);
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
            "print" => {
                let mut text = String::new();
                if let Some(inputs) = &block.inputs {
                    if let Some(input) = inputs.get("TEXT") {
                        text = self.get_string_input(input).unwrap_or_else(|| "".to_string());
                    }
                }
                Some(Value::String(text))
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

            "bigger" | "smaller" | "equal" => {
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
}
