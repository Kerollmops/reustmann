//! ## Instruction Set Summary
//! These are the opcodes of the Reustmann instruction set, shown with their
//! single-character mnemonics and long mnemonics.
//!
//! ### System and stack
//!
//! ```text
//! ;       NOP     No-operation
//! R       RESET   Reset
//! H       HALT    Halt program execution
//! I       IN      Input from stdin
//! O       OUT     Output to stdout
//! p       POP     Pop the stack
//! D       DUP     Duplicate the top of the stack
//! C       PUSHPC  Push the current PC
//! c       POPPC   Pop the stack and set PC
//! Y       POPSP   Pop the stack and set SP
//! G       SPTGT   Set the SP to the next TARGET
//! P       PUSHNZ  Push the NZ flag
//! S       SWAP    Swap the top two stacked words
//! ```
//!
//! ### Math and logic
//!
//! ```text
//! 0       PUSH0   Push a zero onto the stack
//! +       ADD     Add the top two stacked words
//! -       SUB     Subract the top two stacked words
//! .       INC     Increment the top of the stack
//! ,       DEC     Decrement the top of the stack
//! *       MUL     Multiply the top two stacked words
//! /       DIV     Divide the top two stacked words
//! ^       XOR     Bitwise exclusive OR
//! &       AND     Bitwise logical AND
//! |       OR      Bitwise logical OR
//! (       SHL     Logical shift left the top stacked word
//! )       SHR     Logical shift right the top stacked word
//! ~       NOT     Bitwise invert the top stacked word
//! ```
//!
//! ### Conditionals
//!
//! ```text
//! Z       BZ      Branch on zero
//! z       BNZ     Branch on not-zero
//! =       BEQ     Branch on equal
//! >       BGT     Branch on greater than
//! {       BLT     Branch on less than
//! }       BGE     Branch on greater or equal
//! ```
//!
//! ### Unconditionals
//!
//! ```text
//! L       LOOP    Loop until the following ENDL
//! ]       ENDL    End of LOOP
//! B       BRAN    Branch to next TARGET opcode
//! b       BRAP    Branch to previous TARGET opcode
//! T       TARGET  Branch target for BRAN, BRAP
//! 1       SKIP1   Skip over the next instruction
//! 2       SKIP2   Skip over the next two instructions
//! 3       SKIP3   Skip over the next three instructions
//! 4       SKIP4   Skip over the next four instructions
//! 5       SKIP5   Skip over the next five instructions
//! 6       SKIP6   Skip over the next six instructions
//! 7       SKIP7   Skip over the next seven instructions
//! 8       SKIP8   Skip over the next eight instructions
//! 9       SKIP9   Skip over the next nine instructions
//! ```

use std::convert::From;

pub mod mnemonics;
pub mod long_mnemonics;
pub mod op_codes;

pub use self::op_codes::OpCode;
pub use self::mnemonics::Mnemonic;
pub use self::long_mnemonics::LongMnemonic;

/// These are the opcodes of the Reustmann instruction set,
/// shown with their single-character mnemonics and long mnemonics.
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    /// No-operation, do-nothing
    ///
    /// mnemonic: `;`
    ///
    /// The NOP opcode may be encoded in memory by the value zero,
    /// or by any value not assigned to another opcode. During execution,
    /// all unassigned opcode values are mapped to the NOP instruction.
    ///
    /// ```text
    /// PC = PC + 1 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Nop = op_codes::NOP as isize,

    /// Reset
    ///
    /// mnemonic: `R`
    ///
    /// ```text
    /// PC = 0
    /// SP = 0
    /// NZ = false
    /// ```
    Reset = op_codes::RESET as isize,

    /// Halt program execution
    ///
    /// mnemonic: `H`
    ///
    /// Causes program execution to stop.
    Halt = op_codes::HALT as isize,

    /// Input a char from stdin, push it onto the stack
    ///
    /// mnemonic: `I`
    ///
    /// ```text
    /// SP = SP - 1 mod L
    /// *SP = getchar() trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result stacked is nonzero, else false
    /// ```
    In = op_codes::IN as isize,

    /// Pop a word from the stack, output to stdout
    ///
    /// mnemonic: `O`
    ///
    /// If the value on the top of the stack is outside the range of a char,
    /// it will be truncated to a char as it is output.
    /// This is inconsequential for Reustmann machines of rank W ≤ 8.
    ///
    /// ```text
    /// putchar((char)*SP)
    /// SP = SP + 1 mod L
    /// PC = PC + 1 mod L
    /// NZ = true if the character output is nonzero, else false
    /// ```
    Out = op_codes::OUT as isize,

    /// Pop a word from the stack
    ///
    /// mnemonic: `p`
    ///
    /// ```text
    /// SP = SP + 1 mod L
    /// PC = PC + 1 mod L
    /// NZ = true if the item popped is nonzero, else false
    /// ```
    Pop = op_codes::POP as isize,

    /// Duplicate the last stacked value
    ///
    /// mnemonic: `D`
    ///
    /// ```text
    /// Temp = *SP
    /// SP = SP - 1 mod L
    /// *SP = Temp
    /// PC = PC + 1 mod L
    /// NZ = true if the value duplicated is nonzero, else false
    /// ```
    Dup = op_codes::DUP as isize,

    /// Push the PC onto the stack
    ///
    /// mnemonic: `C`
    ///
    /// ```text
    /// SP = SP - 1 mod L
    /// *SP = PC trunc W
    /// PC = PC + 1 mod L
    /// NZ = no change
    /// ```
    PushPc = op_codes::PUSHPC as isize,

    /// Pop the PC from the stack
    ///
    /// mnemonic: `c`
    ///
    /// ```text
    /// PC = *SP mod L
    /// SP = SP + 1 mod L
    /// NZ = no change
    /// ```
    PopPc = op_codes::POPPC as isize,

    /// Pop the SP from the stack
    ///
    /// mnemonic: `Y`
    ///
    /// ```text
    /// SP = *SP mod L
    /// PC = PC + 1 mod L
    /// NZ = no change
    /// ```
    PopSp = op_codes::POPSP as isize,

    /// Set the SP to the next TARGET opcode
    ///
    /// mnemonic: `G`
    ///
    /// A search for the subsequent TARGET opcode is done at the time the SPTGT
    /// instruction is encountered, from the SPTGT instruction to memory location L - 1.
    /// The search does not wrap around. If no TARGET opcode is found,
    /// or if the PC is already at L - 1, the SPTGT is executed as if it were a NOP instruction.
    ///
    /// ```text
    /// If a subsequent TARGET opcode is found:
    ///     SP = address of the TARGET opcode
    /// else:
    ///     SP = no change
    /// PC = PC + 1 mod L
    /// NZ = no change
    /// ```
    SpTgt = op_codes::SPTGT as isize,

    /// Push the NZ flag
    ///
    /// mnemonic: `P`
    ///
    /// ```text
    /// SP = SP - 1 mod L
    /// *SP = NZ
    /// PC = PC + 1 mod L
    /// NZ = no change
    /// ```
    PushNz = op_codes::PUSHNZ as isize,

    /// Swap the top two items on the stack
    ///
    /// mnemonic: `S`
    ///
    /// ```text
    /// Temp = *SP
    /// *SP = *(SP + 1 mod L)
    /// *(SP + 1 mod L) = Temp
    /// PC = PC + 1 mod L
    /// NZ = no change
    /// ```
    Swap = op_codes::SWAP as isize,

    /// Push a zero onto the stack
    ///
    /// mnemonic: `0`
    ///
    /// ```text
    /// SP = SP - 1 mod L
    /// *SP = 0
    /// PC = PC + 1 mod L
    /// NZ = false
    /// ```
    Push0 = op_codes::PUSH0 as isize,

    /// Add the top two stacked words, push the result
    ///
    /// mnemonic: `+`
    ///
    /// ```text
    /// SP = SP - 1 mod L
    /// *SP = (*(SP + 2 mod L) + *(SP+ 1 mod L)) trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    Add = op_codes::ADD as isize,

    /// Subtract the top two stacked words and push the result
    ///
    /// mnemonic: `-`
    ///
    /// ```text
    /// SP = SP - 1 mod L
    /// *SP = (*(SP + 2 mod L) - *(SP + 1 mod L)) trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    Sub = op_codes::SUB as isize,

    /// Increment the item at the top of the stack
    ///
    /// mnemonic: `.`
    ///
    /// ```text
    /// *SP = (*SP) + 1 trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    Inc = op_codes::INC as isize,

    /// Decrement the item on the top of the stack
    ///
    /// mnemonic: `,`
    ///
    /// ```text
    /// *SP = (*SP) - 1 trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    Dec = op_codes::DEC as isize,

    /// Multiply the top two stacked words and push the result
    ///
    /// mnemonic: `*`
    ///
    /// ```text
    /// SP = SP - 1 mod L
    /// *SP= (*(SP + 2 mod L) * (*(SP + 1 mod L)) trunc W PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    Mul = op_codes::MUL as isize,

    /// Pop two words, divide, push the quotient and remainder
    ///
    /// mnemonic: `/`
    ///
    /// If the divisor is zero, the quotient will be the maximum possible word value, and the remainder zero.
    ///
    /// ```text
    /// Op0 = *(SP + 1 mod L)
    /// Op1 = *SP
    /// if Op1 is zero, change Op0 to the maximum value and Op1 to 1 *(SP + 1 mod L) = quotient of Op0 / Op1 trunc W
    /// *SP = remainder of Op0 / Op1
    /// PC = PC + 1 mod L
    /// NZ = true if the quotient is nonzero, else false
    /// ```
    Div = op_codes::DIV as isize,

    /// Bitwise XOR the top two stacked words and push the result
    ///
    /// mnemonic: `^`
    ///
    /// ```text
    /// SP = SP - 1 mod L
    /// *SP = (*(SP + 2 mod L) XOR *(SP + 2 mod L)) trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    Xor = op_codes::XOR as isize,

    /// Bitwise AND the top two stacked words and push the result
    ///
    /// mnemonic: `&`
    ///
    /// ```text
    /// SP = SP - 1 mod L
    /// *SP = (*( SP + 2 mod L) AND *(SP + 2 mod L)) trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    And = op_codes::AND as isize,

    /// Bitwise OR the top two stacked words and push the result
    ///
    /// mnemonic: `|`
    ///
    /// ```text
    /// SP = SP - 1 mod L
    /// *SP = (*(SP + 2 mod L) OR *(SP + 2 mod L)) trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    Or = op_codes::OR as isize,

    /// Logical shift left
    ///
    /// mnemonic: `(`
    ///
    /// ```text
    /// *SP = *SP << 1 trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    Shl = op_codes::SHL as isize,

    /// Logical shift right
    ///
    /// mnemonic: `)`
    ///
    /// ```text
    /// *SP = *SP >> 1 trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    Shr = op_codes::SHR as isize,

    /// Bitwise NOT
    ///
    /// mnemonic: `~`
    ///
    /// ```text
    /// *SP = NOT *SP trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    Not = op_codes::NOT as isize,

    /// Branch if zero (NZ flag is false)
    ///
    /// mnemonic: `Z`
    ///
    /// Skips one opcode if NZ is false.
    ///
    /// ```text
    /// if NZ is false
    ///     PC = PC + 2 mod L
    /// else
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    /// ```
    Bz = op_codes::BZ as isize,

    /// Branch if nonzero (NZ flag is true)
    ///
    /// mnemonic: `z`
    ///
    /// Skips one opcode if NZ is true.
    ///
    /// ```text
    /// if NZ is true
    ///     PC = PC + 2 mod L
    /// else
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    /// ```
    Bnz = op_codes::BNZ as isize,

    /// Compare top two stacked words, branch if equal
    ///
    /// mnemonic: `=`
    ///
    /// ```text
    /// if *(SP + 1 mod L) .eq. *SP
    ///     PC = PC+ 2 mod L
    /// else
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    /// ```
    Beq = op_codes::BEQ as isize,

    /// Compare top two stacked words, branch if greater than
    ///
    /// mnemonic: `>`
    ///
    /// ```text
    /// if *(SP + 1 mod L) > *SP
    ///     PC = PC + 2 mod L
    /// else
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    /// ```
    Bgt = op_codes::BGT as isize,

    /// Compare top two stacked words, branch if less than
    ///
    /// mnemonic: `{`
    ///
    /// ```text
    /// if *(SP + 1 mod L) < *SP
    ///     PC = PC + 2 mod L
    /// else
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    /// ```
    Blt = op_codes::BLT as isize,

    /// Compare top two stacked words, branch if greater than or equal
    ///
    /// mnemonic: `}`
    ///
    /// ```text
    /// if *(SP + 1 mod L) >= * SP
    /// PC = PC + 2 mod L
    /// else
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    /// ```
    Bge = op_codes::BGE as isize,

    /// Repeat the following instructions up to the next ENDL
    ///
    /// mnemonic: `L`
    ///
    /// ```text
    /// PC = PC + 1 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Loop = op_codes::LOOP as isize,

    /// End of LOOP
    ///
    /// mnemonic: `]`
    ///
    /// Execution resumes at the instruction following the preceding LOOP opcode.
    /// A search for the preceding LOOP opcode is done at the time the ENDL instruction is encountered,
    /// from the current PC to location 0. The search does not wrap around.
    /// If no LOOP opcode is found, or if the PC is already at location 0,
    /// the ENDL is executed as if it were a NOP instruction.
    ///
    /// ```text
    /// if there is a preceding LOOP instruction:
    ///     PC = location of LOOP opcode + 1
    /// else:
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    /// ```
    EndL = op_codes::ENDL as isize,

    /// Branch to the next TARGET opcode
    ///
    /// mnemonic: `B`
    ///
    /// A search for the subsequent TARGET opcode is done at the time the BRAN instruction is encountered,
    /// from the BRAN instruction to memory location L - 1.
    /// The search does not wrap around. If no TARGET opcode is found,
    /// the BRAN is executed as if it were a NOP instruction.
    /// If the TARGET is found at memory location L - 1, execution will resume at location 0.
    ///
    /// ```text
    /// if there is a subsequent TARGET instruction:
    ///     PC = (location of TARGET opcode + 1) mod L
    /// else:
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    /// ```
    BraN = op_codes::BRAN as isize,

    /// Branch to the previous TARGET opcode
    ///
    /// mnemonic: `b`
    ///
    /// A search for the previous TARGET opcode is done at the time the BRAP instruction is encountered,
    /// from the BRAP instruction to memory location 0. The search does not wrap around.\
    /// If no TARGET opcode is found or if the PC is already at location 0,
    /// the BRAP is executed as if it were a NOP. instruction.
    ///
    /// ```text
    /// if there is a prior TARGET instruction
    ///     PC = location of TARGET opcode + 1
    /// else
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    /// ```
    BraP = op_codes::BRAP as isize,

    /// Branch target for BRAN and BRAP
    ///
    /// mnemonic: `T`
    ///
    /// See SPTGT, BRAN, and BRAP instructions for the semantics.
    /// The TARGET opcode is just a marker, and is executed as if it were a NOP.
    ///
    /// ```text
    /// PC = PC + 1 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Target = op_codes::TARGET as isize,

    /// Skip one instruction
    ///
    /// mnemonic: `1`
    ///
    /// ```text
    /// PC = PC + 2 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Skip1 = op_codes::SKIP1 as isize,

    /// Skip two instructions
    ///
    /// mnemonic: `2`
    ///
    /// ```text
    /// PC = PC + 3 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Skip2 = op_codes::SKIP2 as isize,

    /// Skip three instructions
    ///
    /// mnemonic: `3`
    ///
    /// ```text
    /// PC = PC + 4 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Skip3 = op_codes::SKIP3 as isize,

    /// Skip four instructions
    ///
    /// mnemonic: `4`
    ///
    /// ```text
    /// PC = PC + 5 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Skip4 = op_codes::SKIP4 as isize,

    /// Skip five instructions
    ///
    /// mnemonic: `5`
    ///
    /// ```text
    /// PC = PC + 6 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Skip5 = op_codes::SKIP5 as isize,

    /// Skip five instructions
    ///
    /// mnemonic: `6`
    ///
    /// ```text
    /// PC = PC + 7 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Skip6 = op_codes::SKIP6 as isize,

    /// Skip five instructions
    ///
    /// mnemonic: `7`
    ///
    /// ```text
    /// PC = PC + 8 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Skip7 = op_codes::SKIP7 as isize,

    /// Skip five instructions
    ///
    /// mnemonic: `8`
    ///
    /// ```text
    /// PC = PC + 9 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Skip8 = op_codes::SKIP8 as isize,

    /// Skip five instructions
    ///
    /// mnemonic: `9`
    ///
    /// ```text
    /// PC = PC + 10 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Skip9 = op_codes::SKIP9 as isize,
}

use self::Instruction::*;

/// Check if a op_code is a direct command
/// or will be interpreted as NOP
pub fn is_valid_op_code(op_code: OpCode) -> bool {
    match op_code {
              op_codes::NOP
            | op_codes::RESET
            | op_codes::HALT
            | op_codes::IN
            | op_codes::OUT
            | op_codes::POP
            | op_codes::DUP
            | op_codes::PUSHPC
            | op_codes::POPPC
            | op_codes::POPSP
            | op_codes::SPTGT
            | op_codes::PUSHNZ
            | op_codes::SWAP
            | op_codes::PUSH0
            | op_codes::ADD
            | op_codes::SUB
            | op_codes::INC
            | op_codes::DEC
            | op_codes::MUL
            | op_codes::DIV
            | op_codes::XOR
            | op_codes::AND
            | op_codes::OR
            | op_codes::SHL
            | op_codes::SHR
            | op_codes::NOT
            | op_codes::BZ
            | op_codes::BNZ
            | op_codes::BEQ
            | op_codes::BGT
            | op_codes::BLT
            | op_codes::BGE
            | op_codes::LOOP
            | op_codes::ENDL
            | op_codes::BRAN
            | op_codes::BRAP
            | op_codes::TARGET
            | op_codes::SKIP1
            | op_codes::SKIP2
            | op_codes::SKIP3
            | op_codes::SKIP4
            | op_codes::SKIP5
            | op_codes::SKIP6
            | op_codes::SKIP7
            | op_codes::SKIP8
            | op_codes::SKIP9 => true,
            _ => false
        }
}

/// Check if a op_code is a direct command
/// or will be interpreted as NOP
pub fn is_valid_mnemonic(mnemo: Mnemonic) -> bool {
    match mnemo {
              mnemonics::NOP
            | mnemonics::RESET
            | mnemonics::HALT
            | mnemonics::IN
            | mnemonics::OUT
            | mnemonics::POP
            | mnemonics::DUP
            | mnemonics::PUSHPC
            | mnemonics::POPPC
            | mnemonics::POPSP
            | mnemonics::SPTGT
            | mnemonics::PUSHNZ
            | mnemonics::SWAP
            | mnemonics::PUSH0
            | mnemonics::ADD
            | mnemonics::SUB
            | mnemonics::INC
            | mnemonics::DEC
            | mnemonics::MUL
            | mnemonics::DIV
            | mnemonics::XOR
            | mnemonics::AND
            | mnemonics::OR
            | mnemonics::SHL
            | mnemonics::SHR
            | mnemonics::NOT
            | mnemonics::BZ
            | mnemonics::BNZ
            | mnemonics::BEQ
            | mnemonics::BGT
            | mnemonics::BLT
            | mnemonics::BGE
            | mnemonics::LOOP
            | mnemonics::ENDL
            | mnemonics::BRAN
            | mnemonics::BRAP
            | mnemonics::TARGET
            | mnemonics::SKIP1
            | mnemonics::SKIP2
            | mnemonics::SKIP3
            | mnemonics::SKIP4
            | mnemonics::SKIP5
            | mnemonics::SKIP6
            | mnemonics::SKIP7
            | mnemonics::SKIP8
            | mnemonics::SKIP9 => true,
            _ => false
        }
}

impl From<Mnemonic> for Instruction {
    fn from(c: Mnemonic) -> Self {
        match c {
           mnemonics::RESET  => Reset,
           mnemonics::HALT   => Halt,
           mnemonics::IN     => In,
           mnemonics::OUT    => Out,
           mnemonics::POP    => Pop,
           mnemonics::DUP    => Dup,
           mnemonics::PUSHPC => PushPc,
           mnemonics::POPPC  => PopPc,
           mnemonics::POPSP  => PopSp,
           mnemonics::SPTGT  => SpTgt,
           mnemonics::PUSHNZ => PushNz,
           mnemonics::SWAP   => Swap,
           mnemonics::PUSH0  => Push0,
           mnemonics::ADD    => Add,
           mnemonics::SUB    => Sub,
           mnemonics::INC    => Inc,
           mnemonics::DEC    => Dec,
           mnemonics::MUL    => Mul,
           mnemonics::DIV    => Div,
           mnemonics::XOR    => Xor,
           mnemonics::AND    => And,
           mnemonics::OR     => Or,
           mnemonics::SHL    => Shl,
           mnemonics::SHR    => Shr,
           mnemonics::NOT    => Not,
           mnemonics::BZ     => Bz,
           mnemonics::BNZ    => Bnz,
           mnemonics::BEQ    => Beq,
           mnemonics::BGT    => Bgt,
           mnemonics::BLT    => Blt,
           mnemonics::BGE    => Bge,
           mnemonics::LOOP   => Loop,
           mnemonics::ENDL   => EndL,
           mnemonics::BRAN   => BraN,
           mnemonics::BRAP   => BraP,
           mnemonics::TARGET => Target,
           mnemonics::SKIP1  => Skip1,
           mnemonics::SKIP2  => Skip2,
           mnemonics::SKIP3  => Skip3,
           mnemonics::SKIP4  => Skip4,
           mnemonics::SKIP5  => Skip5,
           mnemonics::SKIP6  => Skip6,
           mnemonics::SKIP7  => Skip7,
           mnemonics::SKIP8  => Skip8,
           mnemonics::SKIP9  => Skip9,
           mnemonics::NOP | _ => Nop,
        }
    }
}

impl From<Instruction> for Mnemonic {
    fn from(i: Instruction) -> Self {
        match i {
            Nop    => mnemonics::NOP,
            Reset  => mnemonics::RESET,
            Halt   => mnemonics::HALT,
            In     => mnemonics::IN,
            Out    => mnemonics::OUT,
            Pop    => mnemonics::POP,
            Dup    => mnemonics::DUP,
            PushPc => mnemonics::PUSHPC,
            PopPc  => mnemonics::POPPC,
            PopSp  => mnemonics::POPSP,
            SpTgt  => mnemonics::SPTGT,
            PushNz => mnemonics::PUSHNZ,
            Swap   => mnemonics::SWAP,
            Push0  => mnemonics::PUSH0,
            Add    => mnemonics::ADD,
            Sub    => mnemonics::SUB,
            Inc    => mnemonics::INC,
            Dec    => mnemonics::DEC,
            Mul    => mnemonics::MUL,
            Div    => mnemonics::DIV,
            Xor    => mnemonics::XOR,
            And    => mnemonics::AND,
            Or     => mnemonics::OR,
            Shl    => mnemonics::SHL,
            Shr    => mnemonics::SHR,
            Not    => mnemonics::NOT,
            Bz     => mnemonics::BZ,
            Bnz    => mnemonics::BNZ,
            Beq    => mnemonics::BEQ,
            Bgt    => mnemonics::BGT,
            Blt    => mnemonics::BLT,
            Bge    => mnemonics::BGE,
            Loop   => mnemonics::LOOP,
            EndL   => mnemonics::ENDL,
            BraN   => mnemonics::BRAN,
            BraP   => mnemonics::BRAP,
            Target => mnemonics::TARGET,
            Skip1  => mnemonics::SKIP1,
            Skip2  => mnemonics::SKIP2,
            Skip3  => mnemonics::SKIP3,
            Skip4  => mnemonics::SKIP4,
            Skip5  => mnemonics::SKIP5,
            Skip6  => mnemonics::SKIP6,
            Skip7  => mnemonics::SKIP7,
            Skip8  => mnemonics::SKIP8,
            Skip9  => mnemonics::SKIP9,
        }
    }
}

impl From<OpCode> for Instruction {
    fn from(c: OpCode) -> Self {
        match c {
            op_codes::RESET  => Reset,
            op_codes::HALT   => Halt,
            op_codes::IN     => In,
            op_codes::OUT    => Out,
            op_codes::POP    => Pop,
            op_codes::DUP    => Dup,
            op_codes::PUSHPC => PushPc,
            op_codes::POPPC  => PopPc,
            op_codes::POPSP  => PopSp,
            op_codes::SPTGT  => SpTgt,
            op_codes::PUSHNZ => PushNz,
            op_codes::SWAP   => Swap,
            op_codes::PUSH0  => Push0,
            op_codes::ADD    => Add,
            op_codes::SUB    => Sub,
            op_codes::INC    => Inc,
            op_codes::DEC    => Dec,
            op_codes::MUL    => Mul,
            op_codes::DIV    => Div,
            op_codes::XOR    => Xor,
            op_codes::AND    => And,
            op_codes::OR     => Or,
            op_codes::SHL    => Shl,
            op_codes::SHR    => Shr,
            op_codes::NOT    => Not,
            op_codes::BZ     => Bz,
            op_codes::BNZ    => Bnz,
            op_codes::BEQ    => Beq,
            op_codes::BGT    => Bgt,
            op_codes::BLT    => Blt,
            op_codes::BGE    => Bge,
            op_codes::LOOP   => Loop,
            op_codes::ENDL   => EndL,
            op_codes::BRAN   => BraN,
            op_codes::BRAP   => BraP,
            op_codes::TARGET => Target,
            op_codes::SKIP1  => Skip1,
            op_codes::SKIP2  => Skip2,
            op_codes::SKIP3  => Skip3,
            op_codes::SKIP4  => Skip4,
            op_codes::SKIP5  => Skip5,
            op_codes::SKIP6  => Skip6,
            op_codes::SKIP7  => Skip7,
            op_codes::SKIP8  => Skip8,
            op_codes::SKIP9  => Skip9,
            op_codes::NOP | _ => Nop,
        }
    }
}

impl From<Instruction> for OpCode {
    fn from(i: Instruction) -> Self {
        i as OpCode
    }
}

impl From<Instruction> for &'static str {
    fn from(c: Instruction) -> Self {
        match c {
            Nop     => long_mnemonics::NOP,
            Reset   => long_mnemonics::RESET,
            Halt    => long_mnemonics::HALT,
            In      => long_mnemonics::IN,
            Out     => long_mnemonics::OUT,
            Pop     => long_mnemonics::POP,
            Dup     => long_mnemonics::DUP,
            PushPc  => long_mnemonics::PUSHPC,
            PopPc   => long_mnemonics::POPPC,
            PopSp   => long_mnemonics::POPSP,
            SpTgt   => long_mnemonics::SPTGT,
            PushNz  => long_mnemonics::PUSHNZ,
            Swap    => long_mnemonics::SWAP,
            Push0   => long_mnemonics::PUSH0,
            Add     => long_mnemonics::ADD,
            Sub     => long_mnemonics::SUB,
            Inc     => long_mnemonics::INC,
            Dec     => long_mnemonics::DEC,
            Mul     => long_mnemonics::MUL,
            Div     => long_mnemonics::DIV,
            Xor     => long_mnemonics::XOR,
            And     => long_mnemonics::AND,
            Or      => long_mnemonics::OR,
            Shl     => long_mnemonics::SHL,
            Shr     => long_mnemonics::SHR,
            Not     => long_mnemonics::NOT,
            Bz      => long_mnemonics::BZ,
            Bnz     => long_mnemonics::BNZ,
            Beq     => long_mnemonics::BEQ,
            Bgt     => long_mnemonics::BGT,
            Blt     => long_mnemonics::BLT,
            Bge     => long_mnemonics::BGE,
            Loop    => long_mnemonics::LOOP,
            EndL    => long_mnemonics::ENDL,
            BraN    => long_mnemonics::BRAN,
            BraP    => long_mnemonics::BRAP,
            Target  => long_mnemonics::TARGET,
            Skip1   => long_mnemonics::SKIP1,
            Skip2   => long_mnemonics::SKIP2,
            Skip3   => long_mnemonics::SKIP3,
            Skip4   => long_mnemonics::SKIP4,
            Skip5   => long_mnemonics::SKIP5,
            Skip6   => long_mnemonics::SKIP6,
            Skip7   => long_mnemonics::SKIP7,
            Skip8   => long_mnemonics::SKIP8,
            Skip9   => long_mnemonics::SKIP9,
        }
    }
}
