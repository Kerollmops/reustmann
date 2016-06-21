//! ## Overview
//! The Reustmann machine is a tiny programming language and abstract processing
//! engine designed for educational use for experimentation with automatic program generation.
//! It has the following properties:
//!
//! - Any bit pattern in Reustmann memory is a legal Reustmann program – there are no instruction faults
//! - All operations address existing memory – there are no memory faults
//! - Reustmann source code consists of a string of characters, one character per instruction
//!
//! A corollary is that any string of random characters is the source code for a legal,
//! executable Reustmann program.
//! This allows, for example, a genetic algorithm to use an Reustmann program in source form as its genome,
//! evolving the source code string until the Reustmann program produces the desired output.
//!
//! ## Architecture
//! The Reustmann abstract machine is a Von Neumann architecture where all memory
//! locations are accessed through a program counter (*PC*) or stack pointer (*SP*).
//! The machine architecture can be characterized as a processor with three special-purpose registers (*PC*, *SP*, and *NZ* flag),
//! *L* memory locations ("words") *W* bits wide, and an instruction set of about 40 opcodes.
//! All possible states of memory and registers comprise legal Reustmann programs
//! – there is no possibility of an illegal instruction or a memory fault.
//!
//! Allowing all possible bit patterns to result in legal (but not necessarily interesting)
//! programs simplifies experimentation with automatic program generation.
//! AI programs, such as neural nets and genetic algorithms,
//! can regard Reustmann programs as arbitrary bit patterns, evaluate them with an Reustmann simulator,
//! and compare the results against some fitness metric, without concern for syntactic program correctness.
//!
//! The Reustmann language is similar to a rudimentary assembly language.
//! Reustmann source code can be expressed using either long or short mnemonics.
//! Here is a simple Reustmann program that copies its input to its output as long as
//! the data is nonzero, shown with long mnemonics with added comments:
//! ```ignore
//! LOOP    ; loop until the ENDL opcode
//! IN      ; read one char from stdin and push it on the stack
//! BNZ     ; skip the next instruction if nonzero
//! HALT    ; HALT
//! OUT     ; pop a word from the stack and write it to stdout
//! ENDL    ; loop forever
//!
//! All memory addressing is through the stack pointer (*SP*) and is implied in each instruction,
//! so there are no operands specified in the source code.
//! Using the short mnemonics, each Reustmann instruction can be represented by a single character,
//! and is the preferred format for entering and saving programs.
//! The sample program above is represented in source code form using short mnemonics as the string:
//!
//! `LIzHO]`.
//!
//! An Reustmann abstract machine contains *L* memory locations, *W* bits wide, where
//!
//! `1 ≤ L ≤ 2^32` and `6 ≤ W ≤ 32`.

// FIXME rename me Iota ?!?!
// but this already exist !!!
mod program;
mod interpreter;

pub mod instruction;
pub mod memory;

/// All instructions used in the Reustmann architecture.
// pub use instruction::op_codes::OpCode;
// pub use instruction::{Mnemonic, LongMnemonic};

pub use program::Program;
pub use interpreter::{Interpreter, Statement, DebugInfos};

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }
