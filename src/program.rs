use std::{fs, io};
use std::path::Path;

/// A set of instructions that can be given to an interpreter.
pub struct Program(Vec<u8>);

impl Program {
    /// Construct a new Program from a source.
    ///
    /// Make sure that you truncate the final newline if any.
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Program> {
        fs::read(path).map(Self::from_iter)
    }

    /// Construct a program from a list of instructions (mnemonic).
    pub fn from_iter<I: IntoIterator<Item=u8>>(iter: I) -> Program {
        Program(iter.into_iter().collect())
    }

    /// Get the u8 representation of the source.
    pub fn memory(&self) -> &[u8] {
        &self.0
    }
}

