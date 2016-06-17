/// Reustmann

mod instruction;
mod program;
mod interpreter;
mod memory;

pub use instruction::Instruction;
pub use program::Program;
pub use interpreter::Interpreter;
pub use memory::{OpCodes, Mnemonics, LongMnemonics};

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }
