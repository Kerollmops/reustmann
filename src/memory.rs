use instruction::{Instruction, OpCode, Mnemonic};

pub enum Memory {
    OpCode(Vec<OpCode>),
    ShortMnemonic(Vec<Mnemonic>),
    LongMnemonic(Vec<&'static str>),
}

impl Memory {
    pub fn short_mnemonic(&self) -> Vec<Mnemonic> {
        match *self {
            Memory::OpCode(mem) => {
                let mut short_mnemonic = Vec::with_capacity(mem.len());
                for i in mem {
                    short_mnemonic.push(i as Instruction);
                }
                short_mnemonic
            },
            Memory::LongMnemonic(mem) => {
                let mut short_mnemonic = Vec::with_capacity(mem.len());
                for i in mem {
                    short_mnemonic.push(i.into());
                }
                short_mnemonic
            },
            Memory::ShortMnemonic(mem) => mem.clone(),
        }
    }

    pub fn long_mnemonic(&self) -> Vec<&'static str> {
        match *self {
            Memory::OpCode(mem) => {
                let mut long_mnemonics = Vec::with_capacity(mem.len());
                for i in mem {
                    long_mnemonics.push(i.into());
                }
                long_mnemonics
            },
            Memory::ShortMnemonic(mem) => {
                let mut long_mnemonics = Vec::with_capacity(mem.len());
                for i in mem {
                    long_mnemonics.push(i.into());
                }
                long_mnemonics
            },
            Memory::LongMnemonic(mem) => mem.clone(),
        }
    }

    pub fn op_codes(&self) -> Vec<OpCode> {
        match *self {
            Memory::ShortMnemonic(mem) => {
                let mut op_codes = Vec::with_capacity(mem.len());
                for i in mem {
                    op_codes.push(i.into());
                }
                op_codes
            },
            Memory::LongMnemonic(mem) => {
                let mut op_codes = Vec::with_capacity(mem.len());
                for i in mem {
                    op_codes.push(i.into());
                }
                op_codes
            },
            Memory::OpCode(mem) => mem.clone(),
        }
    }
}
