use std::{any::Any, fmt::Display};

use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    // NOTE: I really don't like this any.
    pub literal: Box<dyn Any>,
    pub line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?} {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Box<dyn Any>, line: usize) -> Token {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
