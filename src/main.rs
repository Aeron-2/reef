mod vm;
mod chunk;
mod opcode;
mod lexer;
mod compiler;
mod hash;

use crate::vm::InterpretResult;
use crate::vm::VM;
use std::fs;
use crate::lexer::Lexer;
use crate::compiler::Compiler;

fn main() {
    let args:Vec<String> = std::env::args().collect();


    match args.len() {
        1 => {},
        2 => {
            if args[1].split('.').last().map(|ext| ext.to_lowercase()) != Some("rf".to_string()) {
                throw_error("The file must be .rf");
            } else {
                let bytes = fs::read(&args[1]).expect("Failed to read file");
                let lexer = Lexer::new(bytes.as_slice());
                let compiler = Compiler::new(lexer);
                if let Some(chunk) = compiler.compile() {
                    let mut vm = VM::new(chunk);
                    match vm.run() {
                        InterpretResult::RuntimeError{message, line} => eprintln!("[line: {}] Runtime Error.. {}...", line, message),
                        _ => {},
                    }
                } else {}
            }
        },
        _ => {
            throw_error("Too may arguments!");
        }
    }
}

fn throw_error(message: &str) {
    eprintln!("{}", message);
    std::process::exit(64);
}

