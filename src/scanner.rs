use crate::token::{Token, TokenType};

pub struct Scanner {
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub source: String,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            start: 0,
            current: 0,
            line: 1,
            source,
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(
            token_type,
            self.source[self.start..self.current].to_string(),
            self.line,
        )
    }

    fn error_token(&self, message: &str) -> Token {
        Token::new(TokenType::ERROR, message.to_string(), self.line)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn expect(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        self.advance();
        self.make_token(TokenType::STRING)
    }

    fn check_keyword(
        &mut self,
        start: usize,
        length: usize,
        rest: &str,
        token_type: TokenType,
    ) -> TokenType {
        if self.current - self.start == start + length
            && &self.source[self.start..self.current] == rest
        {
            return token_type;
        }
        TokenType::IDENTIFIER
    }

    fn identifier_type(&mut self) -> TokenType {
        match self.peek() {
            'a' => self.check_keyword(1, 2, "nd", TokenType::AND),
            'c' => self.check_keyword(1, 4, "lass", TokenType::CLASS),
            'e' => self.check_keyword(1, 3, "lse", TokenType::ELSE),
            'i' => self.check_keyword(1, 1, "f", TokenType::IF),
            'n' => self.check_keyword(1, 2, "il", TokenType::NIL),
            'o' => self.check_keyword(1, 1, "r", TokenType::OR),
            'p' => self.check_keyword(1, 4, "rint", TokenType::PRINT),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::RETURN),
            's' => self.check_keyword(1, 4, "uper", TokenType::SUPER),
            'v' => self.check_keyword(1, 2, "ar", TokenType::VAR),
            'w' => self.check_keyword(1, 4, "hile", TokenType::WHILE),
            'f' => {
                if self.current - self.start > 1 {
                    match self.peek() {
                        'a' => self.check_keyword(2, 3, "lse", TokenType::FALSE),
                        'o' => self.check_keyword(2, 1, "r", TokenType::FOR),
                        'u' => self.check_keyword(2, 1, "n", TokenType::FUN),
                        _ => TokenType::IDENTIFIER,
                    }
                } else {
                    TokenType::IDENTIFIER
                }
            }
            't' => {
                if self.current - self.start > 1 {
                    match self.peek() {
                        'h' => self.check_keyword(2, 2, "is", TokenType::THIS),
                        'r' => self.check_keyword(2, 2, "ue", TokenType::TRUE),
                        _ => TokenType::IDENTIFIER,
                    }
                } else {
                    TokenType::IDENTIFIER
                }
            }
            'v' => self.check_keyword(1, 2, "ar", TokenType::VAR),
            _ => TokenType::IDENTIFIER,
        }
    }

    fn identifier(&mut self) -> Token {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let token_type = self.identifier_type();
        self.make_token(token_type)
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        let c = self.advance();

        match c {
            '(' => self.make_token(TokenType::LEFT_PAREN),
            ')' => self.make_token(TokenType::RIGHT_PAREN),
            '{' => self.make_token(TokenType::LEFT_BRACE),
            '}' => self.make_token(TokenType::RIGHT_BRACE),
            ';' => self.make_token(TokenType::SEMICOLON),
            ',' => self.make_token(TokenType::COMMA),
            '.' => self.make_token(TokenType::DOT),
            '-' => self.make_token(TokenType::MINUS),
            '+' => self.make_token(TokenType::PLUS),
            '/' => {
                if self.expect('/') {
                    while self.peek_next() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    self.scan_token()
                } else {
                    self.make_token(TokenType::SLASH)
                }
            }
            '*' => self.make_token(TokenType::STAR),
            '!' => {
                if self.expect('=') {
                    self.make_token(TokenType::BANG_EQUAL)
                } else {
                    self.make_token(TokenType::BANG)
                }
            }
            '=' => {
                if self.expect('=') {
                    self.make_token(TokenType::EQUAL_EQUAL)
                } else {
                    self.make_token(TokenType::EQUAL)
                }
            }
            '<' => {
                if self.expect('=') {
                    self.make_token(TokenType::LESS_EQUAL)
                } else {
                    self.make_token(TokenType::LESS)
                }
            }
            '>' => {
                if self.expect('=') {
                    self.make_token(TokenType::GREATER_EQUAL)
                } else {
                    self.make_token(TokenType::GREATER)
                }
            }
            '"' => self.string(),
            'a'..='z' | 'A'..='Z' => self.identifier(),
            '0'..='9' => {
                while self.peek().is_digit(10) {
                    self.advance();
                }

                if self.peek() == '.' && self.peek_next().is_digit(10) {
                    self.advance();
                    while self.peek().is_digit(10) {
                        self.advance();
                    }
                }

                return self.make_token(TokenType::NUMBER);
            }
            _ => self.error_token(&format!("Unexpected character: {}", c)),
        }
    }
}
