pub mod serialization;
pub mod easy_interpreter;

// Re-export useful types so you can use them like `models::Workspace` instead of `models::serialization::Workspace`
pub use serialization::{Workspace, Blocks, Block, Input, Value};
pub use serialization::{Workspace2, Blocks2, Block2, Input2, NextBlock2};
pub use easy_interpreter::EasyInterpreter;
