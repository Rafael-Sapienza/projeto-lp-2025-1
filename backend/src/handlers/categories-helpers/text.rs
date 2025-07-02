pub fn get_string_input() {

}


/*
            let mut text = String::new();
            if let Some(inputs) = &block.inputs {
                if let Some(input) = inputs.get("TEXT") {
                    let value = if let Some(sub_block) = &input.block {
                        execute_block(sub_block)
                    } else if let Some(shadow_block) = &input.shadow {
                        execute_block(shadow_block)
                    } else {
                        None
                    };
                    
                    if let Some(found_value) = value {
                        text = match found_value {
                            Value::String(s) => s,
                            Value::Number(n) => n.to_string(),
                            Value::Boolean(b) => b.to_string(),
                        };
                    }
                }
            }
*/
