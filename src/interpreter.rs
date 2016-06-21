use std::io::{Read, Write};
use instruction::Instruction;
use instruction::op_codes::*;
use instruction::is_valid_mnemonic;
use memory::{Mnemonics, OpCodes};
use program::Program;
use std::u32;

/// Type used to return the execution status of a command
pub type ExecutionSucceeded = bool;

/// Type used to return the opcode executed with its execution status
#[derive(Debug, Copy, Clone)]
pub struct Statement(pub OpCode, pub ExecutionSucceeded);

/// A Debug structure to help debugging :)
// #[derive(Debug)] // TODO !!!
pub struct DebugInfos {
    pub memory: OpCodes,
    pub pc: usize,
    pub sp: usize,
    pub nz: bool
}

/// The main interpreter, execute instructions, read from input,
/// write to output
pub struct Interpreter {
    arch_width: u8,      // [6..32)
    memory: Vec<OpCode>, // [1..2^32)
    pc: usize,
    sp: usize,
    nz: bool
}

impl Interpreter {
    /// Construct a new Interpreter with an existing Program.
    ///
    /// `arch_length` need to be in the range `[1..2^32)`
    /// and `arch_width` in `[6..32)`.
    pub fn new(arch_length: usize, arch_width: usize) -> Result<Interpreter, &'static str> {
        if arch_length == 0 || arch_length > u32::MAX as usize {
            return Err("Arch length need to be in the range [1..2^32)");
        }
        if arch_width < 6 || arch_width > 32 {
            return Err("Arch width need to be in the range [6..32)");
        }
        let mut memory = Vec::with_capacity(arch_length);
        for _ in 0..arch_length {
            memory.push(NOP);
        }
        Ok(Interpreter {
            arch_width: arch_width as u8,
            memory: memory,
            pc: 0,
            sp: 0,
            nz: false
        })
    }

    /// Copy your program in the memory of the machine, a reset is done after
    /// program was loaded.
    pub fn copy_program(&mut self, program: &Program) {
        let mnemos = program.memory();
        for i in 0..mnemos.len() {
            let mnemo = mnemos[i] as char;
            self.memory[i] = if is_valid_mnemonic(mnemo) {
                Into::<Instruction>::into(mnemo).into()
            }
            else { mnemo as u8 };
        }
        self.reset();
    }

    /// return the interpreter arch length
    pub fn arch_length(&self) -> usize {
        self.memory.len()
    }

    /// return the interpreter arch width
    pub fn arch_width(&self) -> usize {
        self.arch_width as usize
    }

    /// Reset `pc`, `sp` and `nz` to `0`, `0` and `false` respectively.
    #[inline]
    pub fn reset(&mut self) -> Statement {
        self.pc = 0;
        self.sp = 0;
        self.nz = false;
        Statement(RESET, true)
    }

    #[inline]
    fn increment_pc_n(&mut self, n: usize) {
        self.pc = self.pc.wrapping_add(n) % self.memory.len();
    }

    #[inline]
    fn increment_pc(&mut self) {
        self.increment_pc_n(1);
    }

    #[inline]
    fn set_nz(&mut self, val: u8) {
        self.nz = val != 0;
    }

    #[inline]
    fn decrement_sp(&mut self) {
        self.sp = if self.sp == 0 { self.memory.len() - 1 } else { self.sp - 1 };
    }

    #[inline]
    fn increment_sp(&mut self) {
        self.sp = self.sp.wrapping_add(1) % self.memory.len();
    }

    #[inline]
    /// Truncate a number to the machine word width.
    fn trunc(&self, val: u8) -> u8 {
        val & ((1 << self.arch_width) - 1)
    }

    // FIXME use Bytes iterator ?
    fn execute<R: Read, W: Write>(&mut self, op: OpCode, input: &mut R, output: &mut W) -> Statement {
        match op {
            RESET => self.reset(),
            HALT => Statement(op, true),
            IN => {
                let mut status = true;
                self.decrement_sp();
                let mut buffer = [0; 1];
                if let Err(_) = input.read(&mut buffer) { // FIXME save/return error ???
                    status = false;
                }
                self.memory[self.sp] = buffer[0];
                self.set_nz(buffer[0]);
                self.increment_pc();
                Statement(op, status)
            },
            OUT => {
                let mut status = true;
                let val = self.memory[self.sp];
                if let Err(_) = output.write(&[val]) { // FIXME save/return error ???
                    status = false;
                }
                self.set_nz(val);
                self.increment_sp();
                self.increment_pc();
                Statement(op, status)
            },
            POP => {
                let val = self.memory[self.sp];
                self.set_nz(val);
                self.increment_sp();
                self.increment_pc();
                Statement(op, true)
            },
            DUP => {
                let tmp = self.memory[self.sp];
                self.decrement_sp();
                self.memory[self.sp] = tmp;
                self.set_nz(tmp);
                self.increment_pc();
                Statement(op, true)
            },
            PUSHPC => {
                // let val = self.trunc(self.pc);
                let val = self.pc as u8; // FIXME use trunc
                self.decrement_sp();
                self.memory[self.sp] = val;
                self.set_nz(val);
                self.increment_pc();
                Statement(op, true)
            },
            POPPC => {
                self.pc = (self.memory[self.sp] as usize) % self.memory.len();
                self.increment_sp();
                Statement(op, true)
            },
            POPSP => {
                self.sp = (self.memory[self.sp] as usize) % self.memory.len();
                self.increment_pc();
                Statement(op, true)
            },
            SPTGT => {
                // find the next TARGET
                if self.pc < self.memory.len() - 1 {
                    for i in self.pc + 1..self.memory.len() {
                        if self.memory[i] == TARGET {
                            self.sp = i;
                            break;
                        }
                    }
                }
                self.increment_pc();
                Statement(op, true)
            },
            PUSHNZ => {
                let val = self.nz as u8;
                self.decrement_sp();
                self.memory[self.sp] = val;
                self.set_nz(val);
                self.increment_pc();
                Statement(op, true)
            },
            SWAP => {
                let tmp = self.memory[self.sp];
                let arch_len = self.memory.len();
                self.memory[self.sp] = self.memory[(self.sp + 1) % arch_len];
                self.memory[(self.sp + 1) % arch_len] = tmp;
                self.increment_pc();
                Statement(op, true)
            },
            PUSH0 => {
                self.decrement_sp();
                let val = 0;
                self.memory[self.sp] = val;
                self.set_nz(val);
                self.increment_pc();
                Statement(op, true)
            },
            ADD => {
                self.decrement_sp();
                let a = self.memory[(self.sp + 2) % self.memory.len()];
                let b = self.memory[(self.sp + 1) % self.memory.len()];
                let val = a + b;
                self.memory[self.sp] = val;
                self.set_nz(val);
                self.increment_pc();
                Statement(op, true)
            },
            SUB => {
                self.decrement_sp();
                let a = self.memory[(self.sp + 2) % self.memory.len()];
                let b = self.memory[(self.sp + 1) % self.memory.len()];
                let val = a - b;
                self.memory[self.sp] = val;
                self.set_nz(val);
                self.increment_pc();
                Statement(op, true)
            },
            INC => {
                let val = self.memory[self.sp].wrapping_add(1);
                self.memory[self.sp] = val;
                self.set_nz(val);
                self.increment_pc();
                Statement(op, true)
            },
            DEC => {
                let val = self.memory[self.sp].wrapping_sub(1);
                self.memory[self.sp] = val;
                self.set_nz(val);
                self.increment_pc();
                Statement(op, true)
            },
            MUL => {
                self.decrement_sp();
                let a = self.memory[(self.sp + 2) % self.memory.len()];
                let b = self.memory[(self.sp + 1) % self.memory.len()];
                let val = a * b;
                self.memory[self.sp] = val;
                self.set_nz(val);
                self.increment_pc();
                Statement(op, true)
            },
            DIV => {
                self.decrement_sp();
                let a = self.memory[(self.sp + 2) % self.memory.len()];
                let b = self.memory[(self.sp + 1) % self.memory.len()];
                let val = if b != 0 { a / b } else { u8::max_value() };
                self.memory[self.sp] = val;
                self.set_nz(val);
                self.increment_pc();
                Statement(op, true)
            },
            XOR => {
                self.decrement_sp();
                let a = self.memory[(self.sp + 2) % self.memory.len()];
                let b = self.memory[(self.sp + 1) % self.memory.len()];
                let val = a ^ b;
                self.memory[self.sp] = val;
                self.set_nz(val);
                self.increment_pc();
                Statement(op, true)
            },
            AND => {
                self.decrement_sp();
                let a = self.memory[(self.sp + 2) % self.memory.len()];
                let b = self.memory[(self.sp + 1) % self.memory.len()];
                let val = a & b;
                self.memory[self.sp] = val;
                self.set_nz(val);
                self.increment_pc();
                Statement(op, true)
            },
            OR => {
                self.decrement_sp();
                let a = self.memory[(self.sp + 2) % self.memory.len()];
                let b = self.memory[(self.sp + 1) % self.memory.len()];
                let val = a | b;
                self.memory[self.sp] = val;
                self.set_nz(val);
                self.increment_pc();
                Statement(op, true)
            },
            SHL => {
                let val = self.memory[self.sp] << 1;
                self.memory[self.sp] = val;
                self.set_nz(val);
                self.increment_pc();
                Statement(op, true)
            },
            SHR => {
                let val = self.memory[self.sp] >> 1;
                self.memory[self.sp] = val;
                self.set_nz(val);
                self.increment_pc();
                Statement(op, true)
            },
            NOT => {
                let val = !self.memory[self.sp];
                self.memory[self.sp] = val;
                self.set_nz(val);
                self.increment_pc();
                Statement(op, true)
            },
            BZ => {
                self.increment_pc();
                if self.nz == false {
                    self.increment_pc();
                }
                Statement(op, true)
            },
            BNZ => {
                self.increment_pc();
                if self.nz == true {
                    self.increment_pc();
                }
                Statement(op, true)
            },
            BEQ => {
                self.increment_pc();
                let a = self.memory[(self.sp + 1) % self.memory.len()];
                let b = self.memory[self.sp];
                if a == b {
                    self.increment_pc();
                }
                Statement(op, true)
            },
            BGT => {
                self.increment_pc();
                let a = self.memory[(self.sp + 1) % self.memory.len()];
                let b = self.memory[self.sp];
                if a > b {
                    self.increment_pc();
                }
                Statement(op, true)
            },
            BLT => {
                self.increment_pc();
                let a = self.memory[(self.sp + 1) % self.memory.len()];
                let b = self.memory[self.sp];
                if a < b {
                    self.increment_pc();
                }
                Statement(op, true)
            },
            BGE => { // FIXME add BLE
                self.increment_pc();
                let a = self.memory[(self.sp + 1) % self.memory.len()];
                let b = self.memory[self.sp];
                if a >= b {
                    self.increment_pc();
                }
                Statement(op, true)
            },
            LOOP => {
                // logic is in the EndL opcode
                self.increment_pc();
                Statement(op, true)
            },
            ENDL => {
                // find the preceding LOOP
                let mut found = false;
                for i in (0..self.pc).rev() {
                    if self.memory[i] == LOOP {
                        self.pc = (i + 1) % self.memory.len();
                        found = true;
                        break;
                    }
                }
                if found == false {
                    self.increment_pc();
                }
                Statement(op, true)
            },
            BRAN => {
                // find the next TARGET
                let mut found = false;
                if self.pc < self.memory.len() - 1 {
                    for i in self.pc + 1..self.memory.len() {
                        if self.memory[i] == TARGET {
                            self.sp = i;
                            found = true;
                            break;
                        }
                    }
                }
                if found == false {
                    self.increment_pc();
                }
                Statement(op, true)
            },
            BRAP => {
                // find the preceding TARGET
                let mut found = false;
                for i in (0..self.pc).rev() {
                    if self.memory[i] == TARGET {
                        self.pc = (i + 1) % self.memory.len();
                        found = true;
                        break;
                    }
                }
                if found == false {
                    self.increment_pc();
                }
                Statement(op, true)
            },
            TARGET => { self.increment_pc(); Statement(op, true) },
            SKIP1 => { self.increment_pc_n(2); Statement(op, true) },
            SKIP2 => { self.increment_pc_n(3); Statement(op, true) },
            SKIP3 => { self.increment_pc_n(4); Statement(op, true) },
            SKIP4 => { self.increment_pc_n(5); Statement(op, true) },
            SKIP5 => { self.increment_pc_n(6); Statement(op, true) },
            SKIP6 => { self.increment_pc_n(7); Statement(op, true) },
            SKIP7 => { self.increment_pc_n(8); Statement(op, true) },
            SKIP8 => { self.increment_pc_n(9); Statement(op, true) },
            SKIP9 => { self.increment_pc_n(10); Statement(op, true) },
            NOP | _ => { self.increment_pc(); Statement(op, true) }, // FIXME return false if not NOP directly ?
        }
    }

    /// Use [Empty](https://doc.rust-lang.org/std/io/struct.Empty.html) and/or
    /// [Sink](https://doc.rust-lang.org/std/io/struct.Sink.html)
    /// if you don't want to give input and/or output.
    pub fn step<R: Read, W: Write>(&mut self, input: &mut R, output: &mut W) -> Statement {
        let instr = self.memory[self.pc];
        self.execute(instr, input, output)
    }

    /// Get a debug struct that can help for debugging programs
    pub fn debug_infos(&self) -> DebugInfos {
       DebugInfos {
            memory: OpCodes(self.memory.clone()),
            pc: self.pc,
            sp: self.sp,
            nz: self.nz
        }
    }
}
