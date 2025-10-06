use std::collections::HashMap;

use crate::{
    LOX, Lox,
    token::{LiteralType, Token},
    token_type::TokenType,
};
use lazy_static::lazy_static;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut h = HashMap::new();
        h.insert("and", TokenType::AND);
        h.insert("class", TokenType::AND);
        h.insert("else", TokenType::AND);
        h.insert("false", TokenType::AND);
        h.insert("for", TokenType::AND);
        h.insert("fun", TokenType::AND);
        h.insert("if", TokenType::AND);
        h.insert("nil", TokenType::AND);
        h.insert("or", TokenType::AND);
        h.insert("print", TokenType::AND);
        h.insert("return", TokenType::AND);
        h.insert("super", TokenType::AND);
        h.insert("this", TokenType::AND);
        h.insert("true", TokenType::AND);
        h.insert("var", TokenType::AND);
        h.insert("while", TokenType::AND);
        h
    };
}

pub struct Scanner {
    source: Vec<char>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            start: 0,
            current: 0,
            line: 1,
            source: vec![],
            tokens: vec![],
        }
    }
}

impl Scanner {
    pub fn from_str(input: &str) -> Self {
        Self {
            source: input.chars().collect(),
            ..Default::default()
        }
    }
    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            String::from(""),
            LiteralType::Option(),
            self.line,
        ));
    }
    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => self.next_match('=', TokenType::BANG_EQUAL, TokenType::BANG),
            '=' => self.next_match('=', TokenType::EQUAL_EQUAL, TokenType::EQUAL),
            '<' => self.next_match('=', TokenType::LESS_EQUAL, TokenType::LESS),
            '>' => self.next_match('=', TokenType::GREATER_EQUAL, TokenType::GREATER),
            '/' => {
                if self.source[self.current] == '/' {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            '0'..='9' => self.number(),
            _ => {
                if c.is_alphanumeric() {
                    self.identifier();
                } else {
                    LOX.lock()
                        .unwrap()
                        .error(self.line, "Unexpected character.")
                }
            }
        }
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn advance(&mut self) -> char {
        // NOTE: Doing this since rust doesn't have postfix operator
        self.current += 1;
        self.source[self.current]
    }
    fn add_token(&mut self, token: TokenType) {
        self.add_token_literal(token, LiteralType::Option());
    }
    fn add_token_literal(&mut self, token: TokenType, literal: LiteralType) {
        self.tokens.push(Token {
            token_type: token,
            lexeme: self.source[self.start..self.current].iter().collect(),
            literal,
            line: self.line,
        });
    }

    fn next_match(&mut self, matching: char, if_true: TokenType, if_false: TokenType) {
        if self.is_at_end() {
            return;
        }
        if self.source[self.current] != matching {
            self.add_token(if_false);
        } else {
            self.add_token(if_true);
        }
        self.current += 1;
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            LOX.lock().unwrap().error(self.line, "Unterminated string");
            return;
        }

        self.advance();

        let val = &self.source[self.start + 1..self.current - 1];
        self.add_token_literal(
            TokenType::STRING,
            LiteralType::String(val.iter().collect::<String>()),
        );
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        let mut is_float = false;

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            is_float = true;
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        println!("Is float? {}", is_float);
        let out = if !is_float {
            self.source[self.start..self.current]
                .iter()
                .collect::<String>()
                .trim()
                .parse::<u64>()
                .unwrap() as f64
        } else {
            self.source[self.start..self.current]
                .iter()
                .collect::<String>()
                .trim()
                .parse()
                .expect("Failed to parse float")
        };

        self.add_token_literal(TokenType::NUMBER, LiteralType::Number(out));
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text = self.source[self.start..self.current]
            .iter()
            .collect::<String>();
        let tokentype = *KEYWORDS
            .get(text.as_str())
            .unwrap_or(&TokenType::IDENTIFIER);
        self.add_token(tokentype);
    }
}
