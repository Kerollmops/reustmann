pub type LongMnemonic = &'static str;

pub const ALL_LONG_MNEMONICS: [&str; 46] = [
    NOP,
    RESET,
    HALT,
    IN,
    OUT,
    POP,
    DUP,
    PUSHPC,
    POPPC,
    POPSP,
    SPTGT,
    PUSHNZ,
    SWAP,
    PUSH0,
    ADD,
    SUB,
    INC,
    DEC,
    MUL,
    DIV,
    XOR,
    AND,
    OR,
    SHL,
    SHR,
    NOT,
    BZ,
    BNZ,
    BEQ,
    BGT,
    BLT,
    BGE,
    LOOP,
    ENDL,
    BRAN,
    BRAP,
    TARGET,
    SKIP1,
    SKIP2,
    SKIP3,
    SKIP4,
    SKIP5,
    SKIP6,
    SKIP7,
    SKIP8,
    SKIP9,
];

pub const NOP: LongMnemonic    = "Nop";
pub const RESET: LongMnemonic  = "Reset";
pub const HALT: LongMnemonic   = "Halt";
pub const IN: LongMnemonic     = "In";
pub const OUT: LongMnemonic    = "Out";
pub const POP: LongMnemonic    = "Pop";
pub const DUP: LongMnemonic    = "Dup";
pub const PUSHPC: LongMnemonic = "PushPc";
pub const POPPC: LongMnemonic  = "PopPc";
pub const POPSP: LongMnemonic  = "PopSp";
pub const SPTGT: LongMnemonic  = "SpTgt";
pub const PUSHNZ: LongMnemonic = "PushNz";
pub const SWAP: LongMnemonic   = "Swap";
pub const PUSH0: LongMnemonic  = "Push0";
pub const ADD: LongMnemonic    = "Add";
pub const SUB: LongMnemonic    = "Sub";
pub const INC: LongMnemonic    = "Inc";
pub const DEC: LongMnemonic    = "Dec";
pub const MUL: LongMnemonic    = "Mul";
pub const DIV: LongMnemonic    = "Div";
pub const XOR: LongMnemonic    = "Xor";
pub const AND: LongMnemonic    = "And";
pub const OR: LongMnemonic     = "Or";
pub const SHL: LongMnemonic    = "Shl";
pub const SHR: LongMnemonic    = "Shr";
pub const NOT: LongMnemonic    = "Not";
pub const BZ: LongMnemonic     = "Bz";
pub const BNZ: LongMnemonic    = "Bnz";
pub const BEQ: LongMnemonic    = "Beq";
pub const BGT: LongMnemonic    = "Bgt";
pub const BLT: LongMnemonic    = "Blt";
pub const BGE: LongMnemonic    = "Bge";
pub const LOOP: LongMnemonic   = "Loop";
pub const ENDL: LongMnemonic   = "EndL";
pub const BRAN: LongMnemonic   = "BraN";
pub const BRAP: LongMnemonic   = "BraP";
pub const TARGET: LongMnemonic = "Target";
pub const SKIP1: LongMnemonic  = "Skip1";
pub const SKIP2: LongMnemonic  = "Skip2";
pub const SKIP3: LongMnemonic  = "Skip3";
pub const SKIP4: LongMnemonic  = "Skip4";
pub const SKIP5: LongMnemonic  = "Skip5";
pub const SKIP6: LongMnemonic  = "Skip6";
pub const SKIP7: LongMnemonic  = "Skip7";
pub const SKIP8: LongMnemonic  = "Skip8";
pub const SKIP9: LongMnemonic  = "Skip9";
