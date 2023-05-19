use crate::scanner::Scanner;
use crate::token::{Token, TokenType};

pub fn compile(source: String) -> Chunk {
    let mut scanner = Scanner::new(source);
    loop {
        let token = scanner.scan_token();
        if token.line != scanner.line {
            print!("{} ", token.line);
            scanner.line = token.line;
        }else{
            print!("  | ");
        }
        println!("{:?}", token);
        if token.token_type == TokenType::EOF {
            break;
        }
    }
}