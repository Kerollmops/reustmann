use memory::OpCodes;
use std::io::Read;
use std::vec::Vec;

/// Give it to the Interpreter !
pub struct Program(Vec<u8>);

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
            instructions.push(c);
        }

        // FIXME use not_line_ending from nom
        if ignore_last_newline == true {
            let endl = '\n' as u8;
            if let Some(&endl) = instructions.last() { // FIXME '\r\n' for windows
                instructions.pop();
            }
        }
        Ok(Program(instructions))
    }

    /// Get the u8 representation of the source.
    pub fn memory(&self) -> &[u8] {
        &self.0
    }
}

