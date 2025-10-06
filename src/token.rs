use std::{any::Any, fmt::Display};

use crate::token_type::TokenType;

#[derive(Debug)]
pub enum LiteralType {
    Number(f64),
    String(String),
    Option(),
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: LiteralType,
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
    pub fn new(token_type: TokenType, lexeme: String, literal: LiteralType, line: usize) -> Token {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
