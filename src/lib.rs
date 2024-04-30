mod ast;
mod env;
mod error;
mod eval;
mod parser;

pub use ast::Expr;
pub use env::Environment;
pub use error::{Error, Result};
pub use eval::Eval;
pub use parser::parse;
pub mod utils;
