use instruction::{Instruction, OpCode, Mnemonic, LongMnemonic};

/// a struct that get all instruction in bytes (used in the interpreter)
#[derive(Clone)]
pub struct OpCodes(pub Vec<OpCode>);

/// a struct containing all char names of each instruction
#[derive(Clone)]
pub struct Mnemonics(pub Vec<Mnemonic>);

/// a struct containing all long names of each instruction
#[derive(Clone)]
pub struct LongMnemonics(pub Vec<LongMnemonic>);

// FIXME accept &Mnemonics...
// Mnemonics to OpCodes
impl From<Mnemonics> for OpCodes {
    fn from(mnemos: Mnemonics) -> Self {
        let Mnemonics(mnemos) = mnemos;
        let mut op_codes = Vec::with_capacity(mnemos.len());
        for instr in mnemos {
            // FIXME do this in one line
            let instr: Instruction = instr.into();
            op_codes.push(instr.into());
        }
        OpCodes(op_codes)
    }
}

// FIXME not authorized ?
// LongMnemonics to OpCodes
// impl From<LongMnemonics> for OpCodes {
//     fn from(long_mnemos: LongMnemonics) -> Self {
//         let LongMnemonics(long_mnemos) = long_mnemos;
//         let mut op_codes = Vec::with_capacity(long_mnemos.len());
//         for instr in long_mnemos {
//             // FIXME do this in one line
//             let instr: Instruction = instr.into();
//             op_codes.push(instr.into());
//         }
//         OpCodes(op_codes)
//     }
// }

// OpCodes to Mnemonics
impl From<OpCodes> for Mnemonics {
    fn from(op_codes: OpCodes) -> Self {
        let OpCodes(op_codes) = op_codes;
        let mut mnemos = Vec::with_capacity(op_codes.len());
        for instr in op_codes {
            let instr: Instruction = instr.into();
            mnemos.push(instr.into());
        }
        Mnemonics(mnemos)
    }
}

// FIXME not authorized ?
// LongMnemonics to Mnemonics
// impl From<LongMnemonics> for Mnemonics {
//     fn from(lmnemos: LongMnemonics) -> Self {
//         let LongMnemonics(lmnemos) = lmnemos;
//         let mut lmnemos = Vec::with_capacity(lmnemos.len());
//         for instr in lmnemos {
//             let instr: Instruction = instr.into();
//             lmnemos.push(instr.into());
//         }
//         Mnemonics(lmnemos)
//     }
// }

// OpCodes to LongMnemonics
impl From<OpCodes> for LongMnemonics {
    fn from(op_codes: OpCodes) -> Self {
        let OpCodes(op_codes) = op_codes;
        let mut lmnemos = Vec::with_capacity(op_codes.len());
        for instr in op_codes {
            let instr: Instruction = instr.into();
            lmnemos.push(instr.into());
        }
        LongMnemonics(lmnemos)
    }
}

// Mnemonics to LongMnemonics
impl From<Mnemonics> for LongMnemonics {
    fn from(mnemos: Mnemonics) -> Self {
        let Mnemonics(mnemos) = mnemos;
        let mut lmnemos = Vec::with_capacity(mnemos.len());
        for instr in mnemos {
            let instr: Instruction = instr.into();
            lmnemos.push(instr.into());
        }
        LongMnemonics(lmnemos)
    }
}
