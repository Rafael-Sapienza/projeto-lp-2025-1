use crate::models::helper_functions::{get_boolean_input, get_number_input};
use crate::models::{Block, BlockExecutor, Value};

pub fn check_if(executor: &mut dyn BlockExecutor, block: &Block) -> Option<Value> {
    let mut condition_met = false;

    if let Some(inputs) = &block.inputs {
        if let Some(input) = inputs.get("CONDITION") {
            if let Some(b) = get_boolean_input(executor, input) {
                condition_met = b;
            }
        }

        if condition_met {
            if let Some(input) = inputs.get("DO") {
                if let Some(sub_block) = &input.block {
                    executor.exec_sequence(sub_block);
                }
            }
        }
    }

    Some(Value::String(String::new()))
}

pub fn check_if_else(executor: &mut dyn BlockExecutor, block: &Block) -> Option<Value> {
    if let Some(inputs) = &block.inputs {
        let mut condition_met = false;

        if let Some(condition_input) = inputs.get("CONDITION") {
            if let Some(b) = get_boolean_input(executor, condition_input) {
                condition_met = b;
            }
        }

        let selected_branch = if condition_met {
            inputs.get("DO")
        } else {
            inputs.get("ELSE")
        };

        if let Some(Some(branch_block)) = selected_branch.map(|input| input.block.as_ref()) {
            executor.exec_sequence(branch_block);
        }
    }

    Some(Value::String(String::new()))
}

pub fn repeat(executor: &mut dyn BlockExecutor, block: &Block) -> Option<Value> {
    if let Some(inputs) = &block.inputs {
        let times = inputs
            .get("TIMES")
            .and_then(|input| get_number_input(executor, input))
            .unwrap_or(0.0) as usize;

        for _ in 0..times {
            if let Some(input) = inputs.get("DO") {
                if let Some(sub_block) = &input.block {
                    executor.exec_sequence(sub_block);
                }
            }
        }
    }
    Some(Value::String(String::new()))
}

pub fn repeat_while(executor: &mut dyn BlockExecutor, block: &Block) -> Option<Value> {
    if let Some(inputs) = &block.inputs {
        let mut guard = 0; // Optional: prevent infinite loops
        while inputs
            .get("CONDITION")
            .and_then(|input| get_boolean_input(executor, input))
            .unwrap_or(false)
        {
            if let Some(input) = inputs.get("DO") {
                if let Some(sub_block) = &input.block {
                    executor.exec_sequence(sub_block);
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
