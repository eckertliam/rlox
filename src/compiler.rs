use crate::scanner::Scanner;
use crate::token::{Token, TokenType};
use crate::chunk::Chunk;
use crate::parser::{Parser, Precedence};
use crate::opcode::OpCode;
use crate::value::Value;

pub struct Compiler {
    parser: Parser,
    pub current_chunk: Chunk,
}

impl Compiler {
    pub fn new(source: String) -> Self {
        Self {
            parser: Parser::new(Scanner::new(source)),
            current_chunk: Chunk::new(),
        }
    }

    fn advance(&mut self) {
        self.parser.advance();
    }

    fn emit_byte(&mut self, byte: u8) {
        self.current_chunk.write_chunk(byte, self.parser.previous_line());
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OP_RETURN as u8);
    }

    fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_constant(&mut self, value: f64) {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::OP_CONSTANT as u8, constant);
    }

    fn make_constant(&mut self, value: f64) -> u8 {
        let constant = self.current_chunk.add_constant(Value::Number(value));
        if constant > u8::MAX as usize {
            self.parser.error("Too many constants in one chunk.");
            return 0;
        }
        constant as u8
    }

    fn grouping(&mut self) {
        self.expression();
        self.parser.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.");
    }

    fn unary(&mut self) {
        let op_type = self.parser.previous().token_type;
        self.parse_precedence(Precedence::Unary);

        match op_type {
            TokenType::MINUS => self.emit_byte(OpCode::OP_NEGATE as u8),
            _ => {}
        }
    }

    fn get_rule(&mut self, token_type: TokenType) -> ParseRule {
        match token_type {
            TokenType::LEFT_PAREN => ParseRule::new(Some(Compiler::grouping), None, Precedence::None),
            TokenType::RIGHT_PAREN => ParseRule::new(None, None, Precedence::None),
            TokenType::LEFT_BRACE => ParseRule::new(None, None, Precedence::None),
            TokenType::RIGHT_BRACE => ParseRule::new(None, None, Precedence::None),
            TokenType::COMMA => ParseRule::new(None, None, Precedence::None),
            TokenType::DOT => ParseRule::new(None, None, Precedence::None),
            TokenType::MINUS => ParseRule::new(Some(Compiler::unary), Some(Compiler::binary), Precedence::Term),
            TokenType::PLUS => ParseRule::new(None, Some(Compiler::binary), Precedence::Term),
            TokenType::SEMICOLON => ParseRule::new(None, None, Precedence::None),
            TokenType::SLASH => ParseRule::new(None, Some(Compiler::binary), Precedence::Factor),
            TokenType::STAR => ParseRule::new(None, Some(Compiler::binary), Precedence::Factor),
            TokenType::BANG => ParseRule::new(None, None, Precedence::None),
            TokenType::BANG_EQUAL => ParseRule::new(None, None, Precedence::None),
            TokenType::EQUAL => ParseRule::new(None, None, Precedence::None),
            TokenType::EQUAL_EQUAL => ParseRule::new(None, None, Precedence::None),
            TokenType::GREATER => ParseRule::new(None, None, Precedence::None),
            TokenType::GREATER_EQUAL => ParseRule::new(None, None, Precedence::None),
            TokenType::LESS => ParseRule::new(None, None, Precedence::None),
            TokenType::LESS_EQUAL => ParseRule::new(None, None, Precedence::None),
            TokenType::IDENTIFIER => ParseRule::new(None, None, Precedence::None),
            TokenType::STRING => ParseRule::new(None, None, Precedence::None),
            TokenType::NUMBER => ParseRule::new(Some(Compiler::number), None, Precedence::None),
            TokenType::AND => ParseRule::new(None, None, Precedence::None),
            TokenType::CLASS => ParseRule::new(None, None, Precedence::None),
            TokenType::ELSE => ParseRule::new(None, None, Precedence::None),
            TokenType::FALSE => ParseRule::new(None, None, Precedence::None),
            TokenType::FUN => ParseRule::new(None, None, Precedence::None),
            TokenType::FOR => ParseRule::new(None, None, Precedence::None),
            TokenType::IF => ParseRule::new(None, None, Precedence::None),
            TokenType::NIL => ParseRule::new(None, None, Precedence::None),
            TokenType::OR => ParseRule::new(None, None, Precedence::None),
            TokenType::PRINT => ParseRule::new(None, None, Precedence::None),
            TokenType::RETURN => ParseRule::new(None, None, Precedence::None),
            TokenType::SUPER => ParseRule::new(None, None, Precedence::None),
            TokenType::THIS => ParseRule::new(None, None, Precedence::None),
            TokenType::TRUE => ParseRule::new(None, None, Precedence::None),
            TokenType::VAR => ParseRule::new(None, None, Precedence::None),
            TokenType::WHILE => ParseRule::new(None, None, Precedence::None),
            TokenType::EOF => ParseRule::new(None, None, Precedence::None),
            TokenType::ERROR => ParseRule::new(None, None, Precedence::None),
        }
    }

    fn binary(&mut self) {
        let op_type = self.parser.previous().token_type;
        let rule = self.get_rule(op_type);
        self.parse_precedence(rule.precedence.next());

        match op_type {
            TokenType::PLUS => self.emit_byte(OpCode::OP_ADD as u8),
            TokenType::MINUS => self.emit_byte(OpCode::OP_SUBTRACT as u8),
            TokenType::STAR => self.emit_byte(OpCode::OP_MULTIPLY as u8),
            TokenType::SLASH => self.emit_byte(OpCode::OP_DIVIDE as u8),
            _ => {}
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let prefix_rule = self.get_rule(self.parser.previous().token_type).prefix;
        if let None = prefix_rule {
            self.parser.error("Expect expression.");
            return;
        }

        prefix_rule.unwrap()(self);

        while precedence <= self.get_rule(self.parser.current().token_type).precedence {
            self.advance();
            let infix_rule = self.get_rule(self.parser.previous().token_type).infix;
            infix_rule.unwrap()(self);
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn number(&mut self) {
        let value = self.parser.previous().lexeme.parse::<f64>().unwrap();
        self.emit_constant(value);
    }

    pub fn compile(&mut self) -> bool {
        self.advance();
        self.expression();
        self.parser.consume(TokenType::EOF, "Expect end of expression.");
        self.end_compiler();
        !self.parser.had_error
    }

    pub fn get_chunk(&mut self) -> Chunk {
        self.current_chunk.clone()
    }
}


struct ParseRule {
    prefix: Option<fn(&mut Compiler)>,
    infix: Option<fn(&mut Compiler)>,
    precedence: Precedence,
}

impl ParseRule {
    fn new(prefix: Option<fn(&mut Compiler)>, infix: Option<fn(&mut Compiler)>, precedence: Precedence) -> Self {
        Self {
            prefix,
            infix,
            precedence,
        }
    }
}

