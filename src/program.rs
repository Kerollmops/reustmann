use std::io::Read;
use super::Instruction;
use std::vec::Vec;

#[derive(Debug)]
pub struct Program {
    word_size: usize,
    instructions: Vec<Instruction> // FIXME use memory object
}

impl Program {
    pub fn new(input: &mut Read, word_size: usize) -> Result<Program, &'static str> {
        let mut content = Vec::new();
        if input.read_to_end(&mut content).is_err() {
            return Err("Error while trying to read input");
        }
        if content.is_empty() {
            return Err("Empty program is not a valid program")
        }
        if word_size == 0 {
            return Err("Word size equal to zero is invalid")
        }
        let mut instructions = Vec::with_capacity(content.len());
        for c in content {
            instructions.push((c as char).into());
        }
        Ok(Program{
            word_size: word_size,
            instructions: instructions
        })
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn word_size(&self) -> usize {
        self.word_size
    }
}

