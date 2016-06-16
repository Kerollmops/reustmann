use std::io::Read;
use super::Instruction;
use std::vec::Vec;

#[derive(Debug)]
pub struct Program(Vec<Instruction>);

impl Program {
    pub fn new(input: &mut Read) -> Result<Program, &'static str> {
        let mut content = Vec::new(); // FIXME use with_capacity() ?
        if input.read_to_end(&mut content).is_err() {
            return Err("Error while tryig to read input.");
        }
        let mut instructions = Vec::with_capacity(content.len());
        for c in content {
            instructions.push(c.into());
        }
        Ok(Program(instructions))
    }
}
