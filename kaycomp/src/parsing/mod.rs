use chumsky::prelude::SimpleSpan;

pub type Span = SimpleSpan<usize>;

mod tokens;
mod lexer;
pub use tokens::*;
pub use lexer::*;