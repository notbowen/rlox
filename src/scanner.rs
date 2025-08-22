use crate::{
    errors::ParsingError,
    token::{Token, TokenType},
};

#[derive(Default)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            line: 1,
            ..Default::default()
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, ParsingError> {
        while !self.is_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        let eof_token = Token::new(TokenType::Eof, "".into(), self.line);
        self.tokens.push(eof_token);

        Ok(&self.tokens)
    }

    fn scan_token(&mut self) -> Result<(), ParsingError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            _ => return Err(ParsingError),
        }

        Ok(())
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        let token = Token::new(token_type, text.into(), self.line);
        self.tokens.push(token);
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source
            .chars()
            .nth(self.current)
            .expect("Overflow advance")
    }

    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
