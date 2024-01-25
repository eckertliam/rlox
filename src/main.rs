mod chunk;
mod value;
mod opcode;
mod vm;
mod compiler;
mod scanner;
mod token;
mod parser;

use std::io::Write;

use vm::{VM, InterpretResult};

fn repl(vm: &mut VM) {
    loop {
        print!("> ");
        std::io::stdout().flush().expect("Error flushing stdout");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error reading line");
        let res = vm.interpret(input);
        match res {
            InterpretResult::CompileError => std::process::exit(65),
            InterpretResult::RuntimeError => std::process::exit(70),
            _ => (),
        }
    }
}

fn run_file(vm: &mut VM, path: &str) {
    let source = std::fs::read_to_string(path).expect("Error reading file");
    let res = vm.interpret(source);
    match res {
        InterpretResult::CompileError => std::process::exit(65),
        InterpretResult::RuntimeError => std::process::exit(70),
        _ => (),
    }
}

fn main() {
    let mut vm = VM::new();

    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        repl(&mut vm);
    } else if args.len() == 2 {
        run_file(&mut vm, &args[1]);
    } else {
        eprintln!("Usage: rlox [path]");
        std::process::exit(64);
    }
}
