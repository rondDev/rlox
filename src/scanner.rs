use crate::{LOX, Lox, token::Token, token_type::TokenType};

struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
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
    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            String::from(""),
            Box::<Option<String>>::from(None),
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
            _ => LOX
                .lock()
                .unwrap()
                .error(self.line, "Unexpected character."),
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
        self.add_token_literal(token, None);
    }
    fn add_token_literal(&mut self, token: TokenType, literal: Option<String>) {
        self.tokens.push(Token {
            token_type: token,
            lexeme: self.source[self.start..self.current].iter().collect(),
            literal: Box::from(literal),
            line: self.line,
        });
    }
}
