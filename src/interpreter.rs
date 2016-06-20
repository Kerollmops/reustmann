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
        if arch_length == 0 || arch_length > u32::MAX as usize { // FIXME ugly and wrong !
            return Err("Arch length need to be in the range [1..2^64)");
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
    pub fn copy_program(&mut self, program: &Program) -> Result<(), &'static str> {
        let mnemos = program.memory();
        if mnemos.len() > self.memory.len() {
            return Err("Program len is bigger than memory len");
        }
        for i in 0..mnemos.len() {
            let mnemo = mnemos[i] as char;
            self.memory[i] = match is_valid_mnemonic(mnemo) {
                true => Into::<Instruction>::into(mnemo).into(),
                false => mnemo as u8,
            };
        }
        self.reset();
        Ok(())
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
        self.pc.wrapping_sub(1) % self.memory.len();
    }

    #[inline]
    fn increment_sp(&mut self) {
        self.sp = self.sp.wrapping_add(1) % self.memory.len()
    }

    #[inline]
    /// Truncate a number to the machine word width.
    fn trunc(&self, val: u8) -> u8 {
        val & ((1 << self.arch_width) - 1)
    }

    // FIXME use Bytes iterator ?
    fn execute<R: Read, W: Write>(&mut self, op: OpCode, input: &mut R, output: &mut W) -> Statement {
        match op {
            RESET  => self.reset(),
            HALT   => Statement(op, true),
            IN     => {
                self.execute(NOP, input, output);

                // if (mDataInIdx != mDataIn.size()) {
                //     self.decrement_sp();
                //     // let val = self.trunc((size_t)(T)mDataIn[mDataInIdx]);
                //     let val = 0;
                //     self.memory[self.sp] = val;
                //     self.set_nz(val);
                //     // don't increment the index past the last element
                //     // mDataInIdx = std::min(mDataInIdx + 1, mDataIn.size() - 1);
                //     self.increment_pc();
                // } else if (mDataInIdx == 0 && mDataIn.size() == 0) {
                //     // input zeros in lieu of an input stream
                //     self.decrement_sp();
                //     self.set_nz(mem[sp] = 0);
                //     self.increment_pc();
                // } else {
                //     // HALT if reached end of input - should never happen
                //     // because we prevent dataInIdx from exceeding the string length.
                //     return false;
                // }
                Statement(op, true)
            },
            OUT    => {
                let val = self.memory[self.sp];
                let status = match output.write(&[val]) {
                    Err(_) => false,
                    _ => true
                };
                // DEBUG
                // println!("OUT {} => '{}'", val, val as char);

                // output.flush()/* .unwrap() */; // FIXME need this ?

                // if (mExecutionTraceLevel >= 1) {
                //     std::cerr << "Output =====>(T) "
                //          << std::to_string((unsigned char)(mem[sp] & 0xff))
                //          << " = \'" << (char)(mem[sp] & 0xff) << "\'" << std::endl;
                // }

                self.set_nz(val);
                self.increment_sp();
                self.increment_pc();

                Statement(op, status)
            },
            POP    => {
                let val = self.memory[self.sp];
                self.set_nz(val);
                self.increment_sp();
                self.increment_pc();
                Statement(op, true)
            },
            DUP    => {
                let tmp = self.memory[self.sp];
                self.decrement_sp();
                self.memory[self.sp] = tmp;
                self.set_nz(tmp);
                self.increment_sp();
                Statement(op, true)
            },
            PUSHPC => { self.execute(NOP, input, output); Statement(op, true) },
            POPPC  => { self.execute(NOP, input, output); Statement(op, true) },
            POPSP  => { self.execute(NOP, input, output); Statement(op, true) },
            SPTGT  => { self.execute(NOP, input, output); Statement(op, true) },
            PUSHNZ => { self.execute(NOP, input, output); Statement(op, true) },
            SWAP   => { self.execute(NOP, input, output); Statement(op, true) },
            PUSH0  => { self.execute(NOP, input, output); Statement(op, true) },
            ADD    => { self.execute(NOP, input, output); Statement(op, true) },
            SUB    => { self.execute(NOP, input, output); Statement(op, true) },
            INC    => { self.execute(NOP, input, output); Statement(op, true) },
            DEC    => { self.execute(NOP, input, output); Statement(op, true) },
            MUL    => { self.execute(NOP, input, output); Statement(op, true) },
            DIV    => { self.execute(NOP, input, output); Statement(op, true) },
            XOR    => { self.execute(NOP, input, output); Statement(op, true) },
            AND    => { self.execute(NOP, input, output); Statement(op, true) },
            OR     => { self.execute(NOP, input, output); Statement(op, true) },
            SHL    => { self.execute(NOP, input, output); Statement(op, true) },
            SHR    => { self.execute(NOP, input, output); Statement(op, true) },
            NOT    => { self.execute(NOP, input, output); Statement(op, true) },
            BZ     => { self.execute(NOP, input, output); Statement(op, true) },
            BNZ    => { self.execute(NOP, input, output); Statement(op, true) },
            BEQ    => { self.execute(NOP, input, output); Statement(op, true) },
            BGT    => { self.execute(NOP, input, output); Statement(op, true) },
            BLT    => { self.execute(NOP, input, output); Statement(op, true) },
            BGE    => { self.execute(NOP, input, output); Statement(op, true) },
            LOOP   => { self.execute(NOP, input, output); Statement(op, true) },
            ENDL   => { self.execute(NOP, input, output); Statement(op, true) },
            BRAN   => { self.execute(NOP, input, output); Statement(op, true) },
            BRAP   => { self.execute(NOP, input, output); Statement(op, true) },
            TARGET => { self.execute(NOP, input, output); Statement(op, true) },
            SKIP1  => { self.increment_pc_n(2); Statement(op, true) },
            SKIP2  => { self.increment_pc_n(3); Statement(op, true) },
            SKIP3  => { self.increment_pc_n(4); Statement(op, true) },
            SKIP4  => { self.increment_pc_n(5); Statement(op, true) },
            SKIP5  => { self.increment_pc_n(6); Statement(op, true) },
            SKIP6  => { self.increment_pc_n(7); Statement(op, true) },
            SKIP7  => { self.increment_pc_n(8); Statement(op, true) },
            SKIP8  => { self.increment_pc_n(9); Statement(op, true) },
            SKIP9  => { self.increment_pc_n(10); Statement(op, true) },
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
