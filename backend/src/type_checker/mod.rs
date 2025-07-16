pub mod expression_type_checker;
pub mod statement_type_checker;

pub use expression_type_checker::{check_expr, check_func_call};
pub use statement_type_checker::check_stmt;
