use std::io::Read;
use super::Instruction;
use super::Mnemonics;
use std::vec::Vec;

pub struct Program(Mnemonics);

impl Program {
    pub fn new(input: &mut Read) -> Result<Program, &'static str> {
        let mut content = Vec::new();
        if input.read_to_end(&mut content).is_err() {
            return Err("Error while trying to read input");
        }
        if content.is_empty() {
            return Err("Empty program is not a valid program")
        }
        let mut instructions = Vec::with_capacity(content.len());
        for c in content {
            instructions.push((c as char).into());
        }
        Ok(Program(Mnemonics(instructions)))
    }

    pub fn memory(&self) -> &Mnemonics {
        &self.0
    }
}

