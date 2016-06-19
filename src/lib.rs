//! Reustmann

extern crate itertools;
mod program;
mod interpreter;

mod instruction;
pub mod memory;

/// All instructions used in the Reustmann architecture.
pub use instruction::op_codes::OpCode;
pub use instruction::{Mnemonic, LongMnemonic};

pub use program::Program;
pub use interpreter::{Interpreter, Statement, DebugInfos};

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }
