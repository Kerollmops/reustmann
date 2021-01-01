extern crate reustmann;
extern crate bstr;

use std::io::Write;
use std::{fs, io};

use reustmann::instruction::op_codes;
use reustmann::{Program, Interpreter, Statement};

const ARCH_LENGTH: usize = 100; // memory length
const ARCH_WIDTH: usize = 8; // word size
const CYCLE_LIMIT: usize = 200;

fn main() {
    let file = std::env::args().nth(1).expect("missing instructions file");
    let instructions = fs::read_to_string(file).unwrap();
    let program = Program::from_iter(instructions.into_bytes());

    let mut interpreter = Interpreter::new(ARCH_LENGTH, ARCH_WIDTH).unwrap();
    interpreter.copy_program(&program);

    let mut input = io::stdin();
    let mut output = Vec::new();
    for _ in 0..CYCLE_LIMIT {
        // each interpreter step return a statement
        // while no `HALT` statement is found, we continue
        match interpreter.step(&mut input, &mut output) {
            Statement(op_codes::HALT, _) => break,
            _ => ()
        }
    }

    io::stdout().write_all(&output).unwrap();
}
