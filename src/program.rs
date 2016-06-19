use memory::Mnemonics;
use std::io::Read;
use std::vec::Vec;

/// Give it to the Interpreter !
pub struct Program(Mnemonics);

impl Program {
    /// Construct a new Program from a source.
    pub fn new(input: &mut Read, ignore_last_newline: bool) -> Result<Program, &'static str> {
        let mut content = Vec::new();

        if input.read_to_end(&mut content).is_err() {
            return Err("Error while trying to read input");
        }
        if content.is_empty() {
            return Err("Empty program is not a valid program")
        }

        let mut instructions = Vec::with_capacity(content.len());
        for c in content {
            instructions.push(c as char);
        }

        // FIXME use not_line_ending from nom
        if ignore_last_newline == true {
            if let Some(&'\n') = instructions.last() { // FIXME '\r\n' for windows
                instructions.pop();
            }
        }
        Ok(Program(Mnemonics(instructions)))
    }

    /// Get the Mnemonic representation of the source.
    pub fn memory(&self) -> &Mnemonics {
        &self.0
    }
}

