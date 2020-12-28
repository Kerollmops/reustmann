pub type Mnemonic = char;

pub const ALL_MNEMONICS: [char; 46] = [
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

pub const NOP: Mnemonic    = ';';
pub const RESET: Mnemonic  = 'R';
pub const HALT: Mnemonic   = 'H';
pub const IN: Mnemonic     = 'I';
pub const OUT: Mnemonic    = 'O';
pub const POP: Mnemonic    = 'p';
pub const DUP: Mnemonic    = 'D';
pub const PUSHPC: Mnemonic = 'C';
pub const POPPC: Mnemonic  = 'c';
pub const POPSP: Mnemonic  = 'Y';
pub const SPTGT: Mnemonic  = 'G';
pub const PUSHNZ: Mnemonic = 'P';
pub const SWAP: Mnemonic   = 'S';
pub const PUSH0: Mnemonic  = '0';
pub const ADD: Mnemonic    = '+';
pub const SUB: Mnemonic    = '-';
pub const INC: Mnemonic    = '.';
pub const DEC: Mnemonic    = ',';
pub const MUL: Mnemonic    = '*';
pub const DIV: Mnemonic    = '/';
pub const XOR: Mnemonic    = '^';
pub const AND: Mnemonic    = '&';
pub const OR: Mnemonic     = '|';
pub const SHL: Mnemonic    = '(';
pub const SHR: Mnemonic    = ')';
pub const NOT: Mnemonic    = '~';
pub const BZ: Mnemonic     = 'Z';
pub const BNZ: Mnemonic    = 'z';
pub const BEQ: Mnemonic    = '=';
pub const BGT: Mnemonic    = '>';
pub const BLT: Mnemonic    = '{';
pub const BGE: Mnemonic    = '}';
pub const LOOP: Mnemonic   = 'L';
pub const ENDL: Mnemonic   = ']';
pub const BRAN: Mnemonic   = 'B';
pub const BRAP: Mnemonic   = 'b';
pub const TARGET: Mnemonic = 'T';
pub const SKIP1: Mnemonic  = '1';
pub const SKIP2: Mnemonic  = '2';
pub const SKIP3: Mnemonic  = '3';
pub const SKIP4: Mnemonic  = '4';
pub const SKIP5: Mnemonic  = '5';
pub const SKIP6: Mnemonic  = '6';
pub const SKIP7: Mnemonic  = '7';
pub const SKIP8: Mnemonic  = '8';
pub const SKIP9: Mnemonic  = '9';
