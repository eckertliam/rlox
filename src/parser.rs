use std::{fmt::format, io::{stderr, Write}};

use crate::{token::{Token, TokenType}, scanner::Scanner};

pub struct Parser {
    scanner: Scanner,
    tokens: Vec<Token>,
    current: usize,
    pub had_error: bool,
    panic_mode: bool,
}

impl Parser {
    pub fn new(scanner: Scanner) -> Self {
        Self {
            scanner,
            tokens: Vec::new(),
            current: 0,
            had_error: false,
            panic_mode: false,
        }
    }

    fn error_at_current(&mut self, message: &str) {
        let token = &self.tokens[self.current].clone();
        self.error_at(token, message);
    }

    pub fn error(&mut self, message: &str) {
        let token = &self.tokens[self.current - 1].clone();
        self.error_at(token, message);
    }

    pub fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn error_at(&mut self, token: &Token, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        let stderr = stderr();
        let mut stderr_handle = stderr.lock();
        write!(stderr_handle, "[line {}] Error", token.line).unwrap();

        match token.token_type {
            TokenType::EOF => write!(stderr_handle, " at end").unwrap(),
            TokenType::ERROR => {}
            _ => write!(stderr_handle, " at '{}'", token.lexeme).unwrap(),
        }

        writeln!(stderr_handle, ": {}", message).unwrap();
        self.had_error = true;
    }

    fn scan_token(&mut self) -> usize {
        self.tokens.push(self.scanner.scan_token());
        self.tokens.len() - 1
    }

    pub fn consume(&mut self, token_type: TokenType, message: &str) {
        let token = &self.tokens[self.current];
        if token.token_type == token_type {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }

    pub fn previous_line(&self) -> usize {
        self.tokens[self.current - 1].line
    }

    pub fn current(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn advance(&mut self) {
        loop {
            self.current = self.scan_token();
            let token = &self.tokens[self.current];
            if token.token_type != TokenType::ERROR {
                break;
            }
            self.error_at_current(&format!("Token: {}\nLine: {}", token.lexeme, token.line));
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl Precedence {
    pub fn next(&self) -> Self {
        match self {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::None,
        }
    }
}
