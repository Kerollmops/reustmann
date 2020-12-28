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
//! locations are accessed through a program counter (**PC**) or stack pointer (**SP**).
//! The machine architecture can be characterized as a processor with three special-purpose registers (**PC**, **SP**, and **NZ** flag),
//! **L** memory locations ("words") **W** bits wide, and an instruction set of about 40 opcodes.
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
//!
//! ```text
//! LOOP    // loop until the ENDL opcode
//! IN      // read one char from stdin and push it on the stack
//! BNZ     // skip the next instruction if nonzero
//! HALT    // HALT
//! OUT     // pop a word from the stack and write it to stdout
//! ENDL    // loop forever
//! ```
//!
//! All memory addressing is through the stack pointer (**SP**) and is implied in each instruction,
//! so there are no operands specified in the source code.
//! Using the short mnemonics, each Reustmann instruction can be represented by a single character,
//! and is the preferred format for entering and saving programs.
//! The sample program above is represented in source code form using short mnemonics as the string:
//!
//! `LIzHO]`
//!
//! An Reustmann abstract machine contains **L** memory locations, **W** bits wide, where
//!
//! `1 ≤ L ≤ 2^32` and `6 ≤ W ≤ 32`.
//!
//! This Reustmann architecture specifies a set of possible abstract machines,
//! where the rank of a machine is determined by unique combinations of **L** and **W**.
//! An Reustmann program runs differently on machines of different ranks.
//! For example, an Reustmann program that runs correctly on an abstract machine
//! of rank `L = 16`, `W = 16` will not necessarily produce the same results on a machine
//! with rank `L = 20`, `W = 8`.
//!
//! ## Memory and addressing model
//! The **L** words of memory are addressed as locations 0 through **L** – 1.
//! The Stack Pointer (**SP**) seamlessly wraps around, treating the entire
//! address space as a circular buffer. Address calculations relating to the
//! Program Counter (**PC**) (e.g., branch targets) are performed using unsigned 32-bit
//! arithmetic and then adjusted if necessary to fit in the range [0 .. **L**).
//! When the **PC** would have been advanced to address A where A > **L**, the address is
//! immediately readjusted to the address modulo **L**. This implies that it is
//! impossible to address a non-existent memory location.
//!
//! Each memory location can store unsigned values in the range [0 .. 2 **W**).
//! The memory locations are all writable. Instruction opcodes are stored in
//! memory by their numeric values as defined below.
//!
//! ## Execution model
//! After machine reset, the three Reustmann registers have these initial values,
//! and are described in more detail in a later section.
//!
//! `PC = 0`, `SP = 0` and `NZ = false`
//!
//! For each simulation cycle, if the value at the memory location indexed by
//! **PC** is a defined opcode number, the opcode is executed in simulation
//! as described in the tables below. If the memory location indexed by **PC**
//! is not a defined opcode number, it is executed as if it were a NOP opcode.
//! This implies there are no instruction faults.
//!
//! ## Input and Output
//! The Reustmann instruction set has one IN and one OUT instruction.
//! These are character-oriented, regardless of the
//! value of **W**. Each time an IN instruction is executed, one character is
//! consumed from a user-specified input string or stream and pushed onto the
//! Reustmann stack, narrowing or widening the value to **W** bits. Each OUT
//! instruction pops the top of the Reustmann stack, casts the value to a (char)
//! type, and sends it to the abstract output stream.
//!
//! ## Source Code Representation
//! For Reustmann machines of rank **W** ≤ 8, the preferred source code format
//! is a string of characters of length ≤ L. When loaded into Reustmann memory,
//! the source string S is interpreted as follows:
//!
//! ```text
//! for each character C in S:
//!     if C is a defined short mnemonic:
//!         store the opcode by opcode number
//!     else
//!         store the character value narrowed to W bits
//! ```
//!
//! When a short-mnemonic source code disassembly is generated by an Reustmann abstract
//! machine of any rank, the listing will render each memory value N as a
//! single character as follows:
//!
//! ```text
//! if N is an assigned opcode number:
//!     show the short opcode mnemonic
//! else if N is representable as a single character:
//!     show the literal character
//! else:
//!     show a ';' (NOP) opcode
//! ```
//!
//! ### PC – Program Counter
//! The program counter **PC** contains a value in the range [0 .. **L**).
//! Values wrap around the address space, such that when the instruction execution
//! reaches the end of memory, execution by default continues at location zero.
//! After machine reset, the **PC** starts at zero, and by default, is incremented
//! by one after each instruction is executed. If any branching or looping opcode
//! results in a calculated address outside the range [0 .. **L**), the address is changed
//! to the calculated address modulo **L** so that it always refers to a valid memory location.
//!
//! ## SP – Stack Pointer
//! The stack pointer **SP** contains a value in the range [0 .. **L**).
//! Values wrap around, so that the entire memory becomes a circular buffer for the **SP**.
//! After machine reset, the **SP** starts at zero, and always points at the last item
//! stacked (the “top” of the stack). The **SP** grows toward lower addresses.
//! If the **SP** changes to an address outside the range [0 .. **L**),
//! it is changed to the value modulo **L** so that it always refers to a valid memory location.
//! This implies that the first item pushed onto the stack after machine reset
//! will be written to the top of memory at location **L**-1, and the new **SP**
//! will contain the value **L**-1 and will grow downward from there.
//!
//! ## NZ – Non-Zero Flag
//! The non-zero flag **NZ** reflects the non-zeroness of the most recent instruction
//! that read or wrote memory, as defined in the details below. **NZ** = true
//! means that the result was nonzero. The **NZ** flag can be tested with
//! the BZ and BNZ instructions. On machine reset, **NZ** is initialized to false.
//!
//! ## Math
//! Values in memory are regarded as unsigned integers in the range [0 .. 2 **W**).
//! The arithmetic opcodes evaluate their operands as if using 32-bit unsigned
//! integers with the results truncated to the least significant **W** bits.
//! See individual opcodes below for more details.
//!
//! ## Copyright Information
//! Copyright 2012 David R. Miller.
//! Permission is granted to copy, distribute and/or modify this document under
//! the terms of the GNU Free Documentation License,
//! Version 1.3 or any later version published by the Free Software Foundation;
//! with no Invariant Sections, no Front-Cover Texts and no Back-Cover Texts.
//! A copy of the license is included in the accompanying
//! file named COPYING and online at http://www.gnu.org/licenses/fdl.txt.

// FIXME rename me Iota Machine ?!?!
// but this already exist !!!
mod program;
mod interpreter;

pub mod instruction;
pub mod memory;

// /// All instructions used in the Reustmann architecture.
// pub use instruction::op_codes::OpCode;
// pub use instruction::{Mnemonic, LongMnemonic};

pub use program::Program;
pub use interpreter::{Interpreter, Statement, DebugInfos};
