#[derive(Debug)]
enum Instruction {
    /// No-operation, do-nothing
    /// mnemonic `;`
    /// numeric `0`
    /// The NOP opcode may be encoded in memory by the value zero,
    /// or by any value not assigned to another opcode. During execution,
    /// all unassigned opcode values are mapped to the NOP instruction.
    ///
    /// PC = PC + 1 mod L
    /// SP = no change
    /// NZ = no change
    Nope,
    /// Reset
    /// mnemonic `R`
    /// numeric `1`
    ///
    /// PC = 0
    /// SP = 0
    /// NZ = false
    Reset,
    /// Halt program execution
    /// mnemonic `H`
    /// numeric `2`
    /// Causes program execution to stop.
    Halt,
    /// Input a char from stdin, push it onto the stack
    /// mnemonic `I`
    /// numeric `3`
    ///
    /// SP = SP – 1 mod L
    /// *SP = getchar() trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result stacked is nonzero, else false
    In,
    /// Pop a word from the stack, output to stdout
    /// mnemonic `O`
    /// numeric `4`
    /// If the value on the top of the stack is outside the range of a char,
    /// it will be truncated to a char as it is output.
    /// This is inconsequential for Iota machines of rank W ≤ 8.
    ///
    /// putchar((char)*SP)
    /// SP = SP + 1 mod L
    /// PC = PC + 1 mod L
    /// NZ = true if the character output is nonzero, else false
    Out,
    /// Pop a word from the stack
    /// mnemonic `p`
    ///
    /// SP=SP+ 1 mod L
    /// PC=PC+ 1 mod L
    /// NZ = true if the item popped is nonzero, else false
    Pop,
    /// Duplicate the last stacked value
    /// mnemonic `D`
    /// numeric `6`
    ///
    /// Temp = *SP
    /// SP = SP – 1 mod L
    /// *SP = Temp
    /// PC = PC + 1 mod L
    /// NZ = true if the value duplicated is nonzero, else false
    Dup,
    /// Push the PC onto the stack
    /// mnemonic `C`
    /// numeric `7`
    ///
    /// SP = SP – 1 mod L
    /// *SP = PC trunc W
    /// PC = PC + 1 mod L
    /// NZ = no change
    PushPc,
    /// Pop the PC from the stack
    /// mnemonic `c`
    /// numeric `8`
    ///
    /// PC = *SP mod L
    /// SP = SP + 1 mod L
    /// NZ = no change
    PopPc,
    /// Pop the SP from the stack
    /// mnemonic `Y`
    /// numeric `9`
    ///
    /// SP= *SP mod L
    /// PC = PC + 1 mod L
    /// NZ = no change
    PopSp,
    /// Set the SP to the next TARGET opcode
    /// mnemonic `G`
    /// numeric `10`
    /// A search for the subsequent TARGET opcode is done at the time the SPTGT
    /// instruction is encountered, from the SPTGT instruction to memory location L - 1.
    /// The search does not wrap around. If no TARGET opcode is found,
    /// or if the PC is already at L - 1, the SPTGT is executed as if it were a NOP instruction.
    ///
    /// If a subsequent TARGET opcode is found:
    ///     SP = address of the TARGET opcode
    /// else:
    ///     SP = no change
    /// PC = PC + 1 mod L
    /// NZ = no change
    SpTgt,
    /// Push the NZ flag
    /// mnemonic `P`
    /// numeric `11`
    ///
    /// SP = SP – 1 mod L
    /// *SP = NZ
    /// PC = PC + 1 mod L
    /// NZ = no change
    PushNz,
    /// Swap the top two items on the stack
    /// mnemonic `S`
    /// numeric `12`
    ///
    /// Temp = *SP
    /// *SP = *(SP + 1 mod L)
    /// *(SP + 1 mod L) = Temp
    /// PC = PC + 1 mod L
    /// NZ = no change
    Swap,
    /// Push a zero onto the stack
    /// mnemonic `0`
    /// numeric `13`
    ///
    /// SP = SP – 1 mod L
    // *SP = 0
    /// PC = PC + 1 mod L
    /// NZ = false
    Push0,
    /// Add the top two stacked words, push the result
    /// mnemonic `+`
    /// numeric `14`
    ///
    /// SP = SP – 1 mod L
    /// *SP = (*(SP + 2 mod L) + *(SP+ 1 mod L)) trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    Add,
    /// Subtract the top two stacked words and push the result
    /// mnemonic `-`
    /// numeric `15`
    ///
    /// SP = SP – 1 mod L
    /// *SP = (*(SP + 2 mod L) - *(SP + 1 mod L)) trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    Sub,
    /// Increment the item at the top of the stack
    /// mnemonic `.`
    /// numeric `16`
    ///
    /// *SP = (*SP) + 1 trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    Inc,
    /// Decrement the item on the top of the stack
    /// mnemonic `,`
    /// numeric `17`
    ///
    /// *SP = (*SP) – 1 trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    Dec,
    /// Multiply the top two stacked words and push the result
    /// mnemonic `*`
    /// numeric `18`
    ///
    /// SP = SP – 1 mod L
    /// *SP= (*(SP + 2 mod L) * (*(SP + 1 mod L)) trunc W PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    Mul,
    /// Pop two words, divide, push the quotient and remainder
    /// mnemonic `/`
    /// numeric `19`
    /// If the divisor is zero, the quotient will be the maximum possible word value, and the remainder zero.
    ///
    /// Op0 = *(SP + 1 mod L)
    /// Op1 = *SP
    /// if Op1 is zero, change Op0 to the maximum value and Op1 to 1 *(SP + 1 mod L) = quotient of Op0 / Op1 trunc W
    /// *SP = remainder of Op0 / Op1
    /// PC = PC + 1 mod L
    /// NZ = true if the quotient is nonzero, else false
    Div,
    /// Bitwise XOR the top two stacked words and push the result
    /// mnemonic `^`
    /// numeric `20`
    ///
    /// SP=SP- 1 mod L
    /// *SP = (*(SP + 2 mod L) XOR *(SP + 2 mod L)) trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    Xor,
    /// Bitwise AND the top two stacked words and push the result
    /// mnemonic `&`
    /// numeric `21`
    ///
    /// SP = SP - 1 mod L
    /// *SP = (*( SP + 2 mod L) AND *(SP + 2 mod L)) trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    And,
    /// Bitwise OR the top two stacked words and push the result
    /// mnemonic `|`
    /// numeric `22`
    ///
    /// SP = SP - 1 mod L
    /// *SP = (*(SP + 2 mod L) OR *(SP + 2 mod L)) trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    Or,
    /// Logical shift left
    /// mnemonic `(`
    /// numeric `23`
    ///
    /// *SP = *SP<< 1 trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    Shl,
    /// Logical shift right
    /// mnemonic `)`
    /// numeric `24`
    ///
    /// *SP= *SP >> 1 trunc W
    /// PC = PC + 1 mod L
    /// NZ = true if the result is nonzero, else false
    Shr,
    /// Bitwise NOT
    /// mnemonic `~`
    /// numeric `25`
    ///
    /// *SP = NOT *SP trunc W
    /// PC = PC+ 1 mod L
    /// NZ = true if the result is nonzero, else false
    Not,
    /// Branch if zero (NZ flag is false)
    /// mnemonic `Z`
    /// numeric `26`
    /// Skips one opcode if NZ is false.
    ///
    /// if NZ is false:
    ///     PC = PC + 2 mod L
    /// else:
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    Bz,
    /// Branch if nonzero (NZ flag is true)
    /// mnemonic `z`
    /// numeric `27`
    /// Skips one opcode if NZ is true.
    ///
    /// if NZ is true:
    ///     PC = PC + 2 mod L
    /// else:
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    Bnz,
    /// Compare top two stacked words, branch if equal
    /// mnemonic `=`
    /// numeric `28`
    ///
    /// if *(SP + 1 mod L) .eq. *SP
    ///     PC = PC+ 2 mod L
    /// else
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    Beq,
    /// Compare top two stacked words, branch if greater than
    /// mnemonic `>`
    /// numeric `29`
    ///
    /// if *(SP + 1 mod L) > *SP
    ///     PC = PC + 2 mod L
    /// else
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    Bgt,
    /// Compare top two stacked words, branch if less than
    /// mnemonic `{`
    /// numeric `30`
    ///
    /// if *(SP + 1 mod L) < *SP
    ///     PC = PC + 2 mod L
    /// else
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    Blt,
    /// Compare top two stacked words, branch if greater than or equal
    /// mnemonic `}`
    /// numeric `31`
    ///
    /// if *(SP + 1 mod L) ≥ * SP
    /// PC = PC + 2 mod L
    /// else
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    Bge,
    /// Repeat the following instructions up to the next ENDL
    /// mnemonic `L`
    /// numeric `32`
    ///
    /// PC = PC + 1 mod L
    /// SP = no change
    /// NZ = no change
    Loop,
    /// End of LOOP
    /// mnemonic `]`
    /// numeric `33`
    /// Execution resumes at the instruction following the preceding LOOP opcode.
    /// A search for the preceding LOOP opcode is done at the time the ENDL instruction is encountered,
    /// from the current PC to location 0. The search does not wrap around.
    /// If no LOOP opcode is found, or if the PC is already at location 0,
    /// the ENDL is executed as if it were a NOP instruction.
    ///
    /// If there is a preceding LOOP instruction:
    ///     PC = location of LOOP opcode + 1
    /// else:
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    EndL,
    /// Branch to the next TARGET opcode
    /// mnemonic `B`
    /// numeric `34`
    /// A search for the subsequent TARGET opcode is done at the time the BRAN instruction is encountered,
    /// from the BRAN instruction to memory location L - 1.
    /// The search does not wrap around. If no TARGET opcode is found,
    /// the BRAN is executed as if it were a NOP instruction.
    /// If the TARGET is found at memory location L – 1, execution will resume at location 0.
    ///
    /// If there is a subsequent TARGET instruction:
    ///     PC = (location of TARGET opcode + 1) mod L
    /// else:
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    BraN,
    /// Branch to the previous TARGET opcode
    /// mnemonic `b`
    /// numeric `35`
    /// A search for the previous TARGET opcode is done at the time the BRAP instruction is encountered,
    /// from the BRAP instruction to memory location 0. The search does not wrap around.\
    /// If no TARGET opcode is found or if the PC is already at location 0,
    /// the BRAP is executed as if it were a NOP. instruction.
    ///
    /// If there is a prior TARGET instruction:
    ///     PC = location of TARGET opcode + 1
    /// else:
    ///     PC = PC + 1 mod L
    /// SP = no change NZ = no change
    BraP,
    /// Branch target for BRAN and BRAP
    /// mnemonic `T`
    /// numeric `36`
    /// See SPTGT, BRAN, and BRAP instructions for the semantics.
    /// The TARGET opcode is just a marker, and is executed as if it were a NOP.
    ///
    /// PC = PC + 1 mod L
    /// SP = no change
    /// NZ = no change
    Target,
    /// Skip one instruction
    /// mnemonic `1`
    /// numeric `37`
    ///
    /// PC = PC + 2 mod L
    /// SP = no change
    /// NZ = no change
    Skip1,
    /// Skip two instructions
    /// mnemonic `2`
    /// numeric `38`
    ///
    /// PC = PC + 3 mod L
    /// SP = no change
    /// NZ = no change
    Skip2,
    /// Skip three instructions
    /// mnemonic `3`
    /// numeric `39`
    ///
    /// PC = PC + 4 mod L
    /// SP = no change
    /// NZ = no change
    Skip3,
    /// Skip four instructions
    /// mnemonic `4`
    /// numeric `40`
    ///
    /// PC = PC + 5 mod L
    /// SP = no change
    /// NZ = no change
    Skip4,
    /// Skip five instructions
    /// mnemonic `5`
    /// numeric `41`
    ///
    /// PC = PC + 6 mod L
    /// SP = no change
    /// NZ = no change
    Skip5,
}
