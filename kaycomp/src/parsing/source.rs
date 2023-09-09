use std::path::PathBuf;

use chumsky::prelude::Rich;

use crate::parsing::{Token, Spanned, Span};

#[salsa::input]
pub struct LexedFile {
    path: PathBuf,
    #[return_ref]
    pub contents: Vec<Spanned<Token>>,
    #[return_ref]
    pub errors: Vec<Rich<'static, char, Span>>
}