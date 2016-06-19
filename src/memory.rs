use instruction::{Instruction, Mnemonic, LongMnemonic};
use instruction::op_codes::OpCode;
use std::ops::Deref;

/// A struct that get all instruction in bytes (used in the Interpreter).
#[derive(Clone)]
pub struct OpCodes(pub Vec<OpCode>);

/// A struct containing all mnemonic names of each instruction.
#[derive(Clone)]
pub struct Mnemonics(pub Vec<Mnemonic>);

/// A struct containing all long names of each instruction.
#[derive(Clone)]
pub struct LongMnemonics(pub Vec<LongMnemonic>);

impl Deref for OpCodes {
    type Target = Vec<OpCode>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Mnemonics {
    type Target = Vec<Mnemonic>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for LongMnemonics {
    type Target = Vec<LongMnemonic>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Mnemonics> for OpCodes {
    fn from(mnemos: Mnemonics) -> Self {
        let Mnemonics(mnemos) = mnemos;
        let mut op_codes = Vec::with_capacity(mnemos.len());
        for instr in mnemos {
            op_codes.push(Into::<Instruction>::into(instr).into());
        }
        OpCodes(op_codes)
    }
}

impl<'a> From<&'a Mnemonics> for OpCodes {
    fn from(mnemos: &Mnemonics) -> Self {
        let &Mnemonics(ref mnemos) = mnemos;
        let mut op_codes = Vec::with_capacity(mnemos.len());
        for instr in mnemos.iter() {
            op_codes.push(Into::<Instruction>::into(*instr).into());
        }
        OpCodes(op_codes)
    }
}

impl From<OpCodes> for Mnemonics {
    fn from(op_codes: OpCodes) -> Self {
        let OpCodes(op_codes) = op_codes;
        let mut mnemos = Vec::with_capacity(op_codes.len());
        for instr in op_codes {
            mnemos.push(Into::<Instruction>::into(instr).into());
        }
        Mnemonics(mnemos)
    }
}

impl<'a> From<&'a OpCodes> for Mnemonics {
    fn from(op_codes: &OpCodes) -> Self {
        let &OpCodes(ref op_codes) = op_codes;
        let mut mnemos = Vec::with_capacity(op_codes.len());
        for instr in op_codes {
            let instr: Instruction = (*instr).into();
            mnemos.push(instr.into());
        }
        Mnemonics(mnemos)
    }
}

impl From<OpCodes> for LongMnemonics {
    fn from(op_codes: OpCodes) -> Self {
        let OpCodes(op_codes) = op_codes;
        let mut lmnemos = Vec::with_capacity(op_codes.len());
        for instr in op_codes {
            lmnemos.push(Into::<Instruction>::into(instr).into());
        }
        LongMnemonics(lmnemos)
    }
}

impl<'a> From<&'a OpCodes> for LongMnemonics {
    fn from(op_codes: &OpCodes) -> Self {
        let &OpCodes(ref op_codes) = op_codes;
        let mut lmnemos = Vec::with_capacity(op_codes.len());
        for instr in op_codes {
            lmnemos.push(Into::<Instruction>::into(*instr).into());
        }
        LongMnemonics(lmnemos)
    }
}

impl From<Mnemonics> for LongMnemonics {
    fn from(mnemos: Mnemonics) -> Self {
        let Mnemonics(mnemos) = mnemos;
        let mut lmnemos = Vec::with_capacity(mnemos.len());
        for instr in mnemos {
            lmnemos.push(Into::<Instruction>::into(instr).into());
        }
        LongMnemonics(lmnemos)
    }
}

impl<'a> From<&'a Mnemonics> for LongMnemonics {
    fn from(mnemos: &Mnemonics) -> Self {
        let &Mnemonics(ref mnemos) = mnemos;
        let mut lmnemos = Vec::with_capacity(mnemos.len());
        for instr in mnemos {
            lmnemos.push(Into::<Instruction>::into(*instr).into());
        }
        LongMnemonics(lmnemos)
    }
}
