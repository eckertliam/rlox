mod chunk;
mod value;
mod opcode;
mod vm;
mod compiler;
mod scanner;
mod token;
mod parser;


use compiler::Compiler;
use vm::{VM, InterpretResult};

fn repl(vm: &mut VM) {
    loop {
        print!("> ");
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("Error reading line");
        vm.interpret(line);
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
    vm.debug_interpret("1 + 2 * 3;".to_string());
}
