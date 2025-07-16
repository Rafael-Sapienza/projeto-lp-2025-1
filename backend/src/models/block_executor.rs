use super::serialization::{Block, Input, Value};

pub trait BlockExecutor {
    fn exec_block(&mut self, block: &Block) -> Option<Value>;
    fn exec_input(&mut self, input: &Input) -> Option<Value>;
}
