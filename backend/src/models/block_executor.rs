use super::serialization::{Block, Input, Value};

pub trait BlockExecutor {
    fn exec_block(&mut self, block: &Block) -> Option<Value>;
    fn exec_input(&mut self, input: &Input) -> Option<Value>;
    fn exec_sequence(&mut self, block: &Block);
    fn push_output(&mut self, text: String);
    fn set_variable(&mut self, id: &str, value: Value);
    fn get_variable(&mut self, id: &str) -> Option<Value>;
}
