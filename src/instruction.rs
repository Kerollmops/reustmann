use std::convert::From;

/// These are the opcodes of the Reustmann instruction set,
/// shown with their single-character mnemonics and long mnemonics.
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    /// No-operation, do-nothing, `;`
    ///
    /// The NOP opcode may be encoded in memory by the value zero,
    /// or by any value not assigned to another opcode. During execution,
    /// all unassigned opcode values are mapped to the NOP instruction.
    ///
    /// ```ignore
    /// PC = PC + 1 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Nop,
    /// Reset, `R`
    ///
    /// ```ignore
    /// PC = 0
    /// SP = 0
    /// NZ = false
    /// ```
    Reset,
    /// Halt program execution, `H`
    ///
    /// Causes program execution to stop.
    Halt,
    /// Input a char from stdin, push it onto the stack, `I`
    ///
    /// ```ignore
    /// SP = SP - 1 mod L
    /// *SP = getchar() trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result stacked is nonzero, else false
    /// ```
    In,
    /// Pop a word from the stack, output to stdout, `O`
    ///
    /// If the value on the top of the stack is outside the range of a char,
    /// it will be truncated to a char as it is output.
    /// This is inconsequential for Iota machines of rank W ≤ 8.
    ///
    /// ```ignore
    /// putchar((char)*SP)
    /// SP = SP + 1 mod L
    /// PC = PC + 1 mod L
    /// NZ = true if the character output is nonzero, else false
    /// ```
    Out,
    /// Pop a word from the stack, `p`
    ///
    /// ```ignore
    /// SP = SP + 1 mod L
    /// PC = PC + 1 mod L
    /// NZ = true if the item popped is nonzero, else false
    /// ```
    Pop,
    /// Duplicate the last stacked value, `D`
    ///
    /// ```ignore
    /// Temp = *SP
    /// SP = SP - 1 mod L
    /// *SP = Temp
    /// PC = PC + 1 mod L
    /// NZ = true if the value duplicated is nonzero, else false
    /// ```
    Dup,
    /// Push the PC onto the stack, `C`
    ///
    /// ```ignore
    /// SP = SP - 1 mod L
    /// *SP = PC trunc W
    /// PC = PC + 1 mod L
    /// NZ = no change
    /// ```
    PushPc,
    /// Pop the PC from the stack, `c`
    ///
    /// ```ignore
    /// PC = *SP mod L
    /// SP = SP + 1 mod L
    /// NZ = no change
    /// ```
    PopPc,
    /// Pop the SP from the stack, `Y`
    ///
    /// ```ignore
    /// SP = *SP mod L
    /// PC = PC + 1 mod L
    /// NZ = no change
    /// ```
    PopSp,
    /// Set the SP to the next TARGET opcode, `G`
    ///
    /// A search for the subsequent TARGET opcode is done at the time the SPTGT
    /// instruction is encountered, from the SPTGT instruction to memory location L - 1.
    /// The search does not wrap around. If no TARGET opcode is found,
    /// or if the PC is already at L - 1, the SPTGT is executed as if it were a NOP instruction.
    ///
    /// ```ignore
    /// If a subsequent TARGET opcode is found:
    ///     SP = address of the TARGET opcode
    /// else:
    ///     SP = no change
    /// PC = PC + 1 mod L
    /// NZ = no change
    /// ```
    SpTgt,
    /// Push the NZ flag, `P`
    ///
    /// ```ignore
    /// SP = SP - 1 mod L
    /// *SP = NZ
    /// PC = PC + 1 mod L
    /// NZ = no change
    /// ```
    PushNz,
    /// Swap the top two items on the stack, `S`
    ///
    /// ```ignore
    /// Temp = *SP
    /// *SP = *(SP + 1 mod L)
    /// *(SP + 1 mod L) = Temp
    /// PC = PC + 1 mod L
    /// NZ = no change
    /// ```
    Swap,
    /// Push a zero onto the stack, `0`
    ///
    /// ```ignore
    /// SP = SP - 1 mod L
    /// *SP = 0
    /// PC = PC + 1 mod L
    /// NZ = false
    /// ```
    Push0,
    /// Add the top two stacked words, push the result, `+`
    ///
    /// ```ignore
    /// SP = SP - 1 mod L
    /// *SP = (*(SP + 2 mod L) + *(SP+ 1 mod L)) trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    Add,
    /// Subtract the top two stacked words and push the result, `-`
    ///
    /// ```ignore
    /// SP = SP - 1 mod L
    /// *SP = (*(SP + 2 mod L) - *(SP + 1 mod L)) trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    Sub,
    /// Increment the item at the top of the stack, `.`
    ///
    /// ```ignore
    /// *SP = (*SP) + 1 trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    Inc,
    /// Decrement the item on the top of the stack, `,`
    ///
    /// ```ignore
    /// *SP = (*SP) - 1 trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    Dec,
    /// Multiply the top two stacked words and push the result, `*`
    ///
    /// ```ignore
    /// SP = SP - 1 mod L
    /// *SP= (*(SP + 2 mod L) * (*(SP + 1 mod L)) trunc W PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    Mul,
    /// Pop two words, divide, push the quotient and remainder, `/`
    ///
    /// If the divisor is zero, the quotient will be the maximum possible word value, and the remainder zero.
    ///
    /// ```ignore
    /// Op0 = *(SP + 1 mod L)
    /// Op1 = *SP
    /// if Op1 is zero, change Op0 to the maximum value and Op1 to 1 *(SP + 1 mod L) = quotient of Op0 / Op1 trunc W
    /// *SP = remainder of Op0 / Op1
    /// PC = PC + 1 mod L
    /// NZ = true if the quotient is nonzero, else false
    /// ```
    Div,
    /// Bitwise XOR the top two stacked words and push the result, `^`
    ///
    /// ```ignore
    /// SP = SP - 1 mod L
    /// *SP = (*(SP + 2 mod L) XOR *(SP + 2 mod L)) trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    Xor,
    /// Bitwise AND the top two stacked words and push the result, `&`
    ///
    /// ```ignore
    /// SP = SP - 1 mod L
    /// *SP = (*( SP + 2 mod L) AND *(SP + 2 mod L)) trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    And,
    /// Bitwise OR the top two stacked words and push the result, `|`
    ///
    /// ```ignore
    /// SP = SP - 1 mod L
    /// *SP = (*(SP + 2 mod L) OR *(SP + 2 mod L)) trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    Or,
    /// Logical shift left, `(`
    ///
    /// ```ignore
    /// *SP = *SP << 1 trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    Shl,
    /// Logical shift right, `)`
    ///
    /// ```ignore
    /// *SP = *SP >> 1 trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    Shr,
    /// Bitwise NOT, `~`
    ///
    /// ```ignore
    /// *SP = NOT *SP trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    /// ```
    Not,
    /// Branch if zero (NZ flag is false), `Z`
    ///
    /// Skips one opcode if NZ is false.
    ///
    /// ```ignore
    /// if NZ is false
    ///     PC = PC + 2 mod L
    /// else
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    /// ```
    Bz,
    /// Branch if nonzero (NZ flag is true), `z`
    ///
    /// Skips one opcode if NZ is true.
    ///
    /// ```ignore
    /// if NZ is true
    ///     PC = PC + 2 mod L
    /// else
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    /// ```
    Bnz,
    /// Compare top two stacked words, branch if equal, `=`
    ///
    /// ```ignore
    /// if *(SP + 1 mod L) .eq. *SP
    ///     PC = PC+ 2 mod L
    /// else
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    /// ```
    Beq,
    /// Compare top two stacked words, branch if greater than, `>`
    ///
    /// ```ignore
    /// if *(SP + 1 mod L) > *SP
    ///     PC = PC + 2 mod L
    /// else
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    /// ```
    Bgt,
    /// Compare top two stacked words, branch if less than, `{`
    ///
    /// ```ignore
    /// if *(SP + 1 mod L) < *SP
    ///     PC = PC + 2 mod L
    /// else
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    /// ```
    Blt,
    /// Compare top two stacked words, branch if greater than or equal, `}`
    ///
    /// ```ignore
    /// if *(SP + 1 mod L) >= * SP
    /// PC = PC + 2 mod L
    /// else
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    /// ```
    Bge,
    /// Repeat the following instructions up to the next ENDL, `L`
    ///
    /// ```ignore
    /// PC = PC + 1 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Loop,
    /// End of LOOP, `]`
    ///
    /// Execution resumes at the instruction following the preceding LOOP opcode.
    /// A search for the preceding LOOP opcode is done at the time the ENDL instruction is encountered,
    /// from the current PC to location 0. The search does not wrap around.
    /// If no LOOP opcode is found, or if the PC is already at location 0,
    /// the ENDL is executed as if it were a NOP instruction.
    ///
    /// ```ignore
    /// if there is a preceding LOOP instruction:
    ///     PC = location of LOOP opcode + 1
    /// else:
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    /// ```
    EndL,
    /// Branch to the next TARGET opcode, `B`
    ///
    /// A search for the subsequent TARGET opcode is done at the time the BRAN instruction is encountered,
    /// from the BRAN instruction to memory location L - 1.
    /// The search does not wrap around. If no TARGET opcode is found,
    /// the BRAN is executed as if it were a NOP instruction.
    /// If the TARGET is found at memory location L - 1, execution will resume at location 0.
    ///
    /// ```ignore
    /// if there is a subsequent TARGET instruction:
    ///     PC = (location of TARGET opcode + 1) mod L
    /// else:
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    /// ```
    BraN,
    /// Branch to the previous TARGET opcode, `b`
    ///
    /// A search for the previous TARGET opcode is done at the time the BRAP instruction is encountered,
    /// from the BRAP instruction to memory location 0. The search does not wrap around.\
    /// If no TARGET opcode is found or if the PC is already at location 0,
    /// the BRAP is executed as if it were a NOP. instruction.
    ///
    /// ```ignore
    /// if there is a prior TARGET instruction
    ///     PC = location of TARGET opcode + 1
    /// else
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    /// ```
    BraP,
    /// Branch target for BRAN and BRAP, `T`
    ///
    /// See SPTGT, BRAN, and BRAP instructions for the semantics.
    /// The TARGET opcode is just a marker, and is executed as if it were a NOP.
    ///
    /// ```ignore
    /// PC = PC + 1 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Target,
    /// Skip one instruction, `1`
    ///
    /// ```ignore
    /// PC = PC + 2 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Skip1,
    /// Skip two instructions, `2`
    ///
    /// ```ignore
    /// PC = PC + 3 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Skip2,
    /// Skip three instructions, `3`
    ///
    /// ```ignore
    /// PC = PC + 4 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Skip3,
    /// Skip four instructions, `4`
    ///
    /// ```ignore
    /// PC = PC + 5 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Skip4,
    /// Skip five instructions, `5`
    ///
    /// ```ignore
    /// PC = PC + 6 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Skip5,
    /// Skip five instructions, `6`
    ///
    /// ```ignore
    /// PC = PC + 7 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Skip6,
    /// Skip five instructions, `7`
    ///
    /// ```ignore
    /// PC = PC + 8 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Skip7,
    /// Skip five instructions, `8`
    ///
    /// ```ignore
    /// PC = PC + 9 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Skip8,
    /// Skip five instructions, `9`
    ///
    /// ```ignore
    /// PC = PC + 10 mod L
    /// SP = no change
    /// NZ = no change
    /// ```
    Skip9,
}

use self::Instruction::*;

// impl From<char> for Instruction {
//     fn from(c: char) -> Self {
//         match c { // FIXME: Really fucking ugly
//             c if c == Reset.into()  => Reset,
//             c if c == Halt.into()   => Halt,
//             c if c == In.into()     => In,
//             c if c == Out.into()    => Out,
//             c if c == Pop.into()    => Pop,
//             c if c == Dup.into()    => Dup,
//             c if c == PushPc.into() => PushPc,
//             c if c == PopPc.into()  => PopPc,
//             c if c == PopSp.into()  => PopSp,
//             c if c == SpTgt.into()  => SpTgt,
//             c if c == PushNz.into() => PushNz,
//             c if c == Swap.into()   => Swap,
//             c if c == Push0.into()  => Push0,
//             c if c == Add.into()    => Add,
//             c if c == Sub.into()    => Sub,
//             c if c == Inc.into()    => Inc,
//             c if c == Dec.into()    => Dec,
//             c if c == Mul.into()    => Mul,
//             c if c == Div.into()    => Div,
//             c if c == Xor.into()    => Xor,
//             c if c == And.into()    => And,
//             c if c == Or.into()     => Or,
//             c if c == Shl.into()    => Shl,
//             c if c == Shr.into()    => Shr,
//             c if c == Not.into()    => Not,
//             c if c == Bz.into()     => Bz,
//             c if c == Bnz.into()    => Bnz,
//             c if c == Beq.into()    => Beq,
//             c if c == Bgt.into()    => Bgt,
//             c if c == Blt.into()    => Blt,
//             c if c == Bge.into()    => Bge,
//             c if c == Loop.into()   => Loop,
//             c if c == EndL.into()   => EndL,
//             c if c == BraN.into()   => BraN,
//             c if c == BraP.into()   => BraP,
//             c if c == Target.into() => Target,
//             c if c == Skip1.into()  => Skip1,
//             c if c == Skip2.into()  => Skip2,
//             c if c == Skip3.into()  => Skip3,
//             c if c == Skip4.into()  => Skip4,
//             c if c == Skip5.into()  => Skip5,
//             c if c == Skip6.into()  => Skip6,
//             c if c == Skip7.into()  => Skip7,
//             c if c == Skip8.into()  => Skip8,
//             c if c == Skip9.into()  => Skip9,
//             _                       => Nop,
//         }
//     }
// }

impl From<char> for Instruction {
    fn from(c: char) -> Self {
        match c {
           'R' => Reset,
           'H' => Halt,
           'I' => In,
           'O' => Out,
           'p' => Pop,
           'D' => Dup,
           'C' => PushPc,
           'c' => PopPc,
           'Y' => PopSp,
           'G' => SpTgt,
           'P' => PushNz,
           'S' => Swap,
           '0' => Push0,
           '+' => Add,
           '-' => Sub,
           '.' => Inc,
           ',' => Dec,
           '*' => Mul,
           '/' => Div,
           '^' => Xor,
           '&' => And,
           '|' => Or,
           '(' => Shl,
           ')' => Shr,
           '~' => Not,
           'Z' => Bz,
           'z' => Bnz,
           '=' => Beq,
           '>' => Bgt,
           '{' => Blt,
           '}' => Bge,
           'L' => Loop,
           ']' => EndL,
           'B' => BraN,
           'b' => BraP,
           'T' => Target,
           '1' => Skip1,
           '2' => Skip2,
           '3' => Skip3,
           '4' => Skip4,
           '5' => Skip5,
           '6' => Skip6,
           '7' => Skip7,
           '8' => Skip8,
           '9' => Skip9,
           ';' | _ => Nop,
        }
    }
}

impl From<Instruction> for char {
    fn from(i: Instruction) -> Self {
        match i {
            Nop    => ';',
            Reset  => 'R',
            Halt   => 'H',
            In     => 'I',
            Out    => 'O',
            Pop    => 'p',
            Dup    => 'D',
            PushPc => 'C',
            PopPc  => 'c',
            PopSp  => 'Y',
            SpTgt  => 'G',
            PushNz => 'P',
            Swap   => 'S',
            Push0  => '0',
            Add    => '+',
            Sub    => '-',
            Inc    => '.',
            Dec    => ',',
            Mul    => '*',
            Div    => '/',
            Xor    => '^',
            And    => '&',
            Or     => '|',
            Shl    => '(',
            Shr    => ')',
            Not    => '~',
            Bz     => 'Z',
            Bnz    => 'z',
            Beq    => '=',
            Bgt    => '>',
            Blt    => '{',
            Bge    => '}',
            Loop   => 'L',
            EndL   => ']',
            BraN   => 'B',
            BraP   => 'b',
            Target => 'T',
            Skip1  => '1',
            Skip2  => '2',
            Skip3  => '3',
            Skip4  => '4',
            Skip5  => '5',
            Skip6  => '6',
            Skip7  => '7',
            Skip8  => '8',
            Skip9  => '9',
        }
    }
}

// impl From<u8> for Instruction {
//     fn from(c: u8) -> Self {
//         match c { // FIXME: Really fucking ugly
//             c if c == Reset.into()  => Reset,
//             c if c == Halt.into()   => Halt,
//             c if c == In.into()     => In,
//             c if c == Out.into()    => Out,
//             c if c == Pop.into()    => Pop,
//             c if c == Dup.into()    => Dup,
//             c if c == PushPc.into() => PushPc,
//             c if c == PopPc.into()  => PopPc,
//             c if c == PopSp.into()  => PopSp,
//             c if c == SpTgt.into()  => SpTgt,
//             c if c == PushNz.into() => PushNz,
//             c if c == Swap.into()   => Swap,
//             c if c == Push0.into()  => Push0,
//             c if c == Add.into()    => Add,
//             c if c == Sub.into()    => Sub,
//             c if c == Inc.into()    => Inc,
//             c if c == Dec.into()    => Dec,
//             c if c == Mul.into()    => Mul,
//             c if c == Div.into()    => Div,
//             c if c == Xor.into()    => Xor,
//             c if c == And.into()    => And,
//             c if c == Or.into()     => Or,
//             c if c == Shl.into()    => Shl,
//             c if c == Shr.into()    => Shr,
//             c if c == Not.into()    => Not,
//             c if c == Bz.into()     => Bz,
//             c if c == Bnz.into()    => Bnz,
//             c if c == Beq.into()    => Beq,
//             c if c == Bgt.into()    => Bgt,
//             c if c == Blt.into()    => Blt,
//             c if c == Bge.into()    => Bge,
//             c if c == Loop.into()   => Loop,
//             c if c == EndL.into()   => EndL,
//             c if c == BraN.into()   => BraN,
//             c if c == BraP.into()   => BraP,
//             c if c == Target.into() => Target,
//             c if c == Skip1.into()  => Skip1,
//             c if c == Skip2.into()  => Skip2,
//             c if c == Skip3.into()  => Skip3,
//             c if c == Skip4.into()  => Skip4,
//             c if c == Skip5.into()  => Skip5,
//             c if c == Skip6.into()  => Skip6,
//             c if c == Skip7.into()  => Skip7,
//             c if c == Skip8.into()  => Skip8,
//             c if c == Skip9.into()  => Skip9,
//             _                       => Nop,
//         }
//     }
// }

impl From<u8> for Instruction {
    fn from(c: u8) -> Self {
        match c {
            1 => Reset,
            2 => Halt,
            3 => In,
            4 => Out,
            5 => Pop,
            6 => Dup,
            7 => PushPc,
            8 => PopPc,
            9 => PopSp,
            10 => SpTgt,
            11 => PushNz,
            12 => Swap,
            13 => Push0,
            14 => Add,
            15 => Sub,
            16 => Inc,
            17 => Dec,
            18 => Mul,
            19 => Div,
            20 => Xor,
            21 => And,
            22 => Or,
            23 => Shl,
            24 => Shr,
            25 => Not,
            26 => Bz,
            27 => Bnz,
            28 => Beq,
            29 => Bgt,
            30 => Blt,
            31 => Bge,
            32 => Loop,
            33 => EndL,
            34 => BraN,
            35 => BraP,
            36 => Target,
            37 => Skip1,
            38 => Skip2,
            39 => Skip3,
            40 => Skip4,
            41 => Skip5,
            42 => Skip6,
            43 => Skip7,
            44 => Skip8,
            45 => Skip9,
            0 | _ => Nop,
        }
    }
}

impl From<Instruction> for u8 {
    fn from(i: Instruction) -> Self {
        i as u8
    }
}
