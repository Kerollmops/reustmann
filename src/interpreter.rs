use std::io::{Read, Write};
use std::fmt;
use instruction::op_codes::*;
use memory::{Mnemonics, OpCodes};
use program::Program;
use std::u32;

/// The statement the machine cna return.
#[derive(Debug, Copy, Clone)]
pub enum Statement {
    /// If the instruction was correctly executed.
    Success,
    /// If halt instruction has been executed.
    HaltInstruction,
    // other ideas ???
}

use self::Statement::{Success, HaltInstruction};

/// A Debug structure to help debugging :)
pub struct DebugInfos {
    pub memory: Mnemonics,
    pub pc: usize,
    pub sp: usize,
    pub nz: bool,
    pub statement: Option<Statement>
}

// FIXME don't use Display !!!
// writeln!(fmtr, colorify!(red: "Number of zombies killed: {}"), zombie_kills);
impl fmt::Display for DebugInfos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO #[macro_use] extern crate colorify;
        writeln!(f, "pc: {}, sp: {}, nz: {}, statement: {:?}", self.pc, self.sp, self.nz, self.statement);
        write!(f, "{:?}", *self.memory)
    }
}

/// The main interpreter, execute instructions, read from input,
/// write to output
pub struct Interpreter {
    arch_width: u8,      // [6..32)
    memory: Vec<OpCode>, // [1..2^32)
    pc: usize,
    sp: usize,
    nz: bool,
    last_statement: Option<Statement>
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
            nz: false,
            last_statement: None
        })
    }

    /// Copy your program in the memory of the machine, a reset is done after
    /// program was loaded.
    pub fn copy_program(&mut self, program: &Program) -> Result<(), &'static str> {
        let OpCodes(op_codes) = program.memory().into();
        if op_codes.len() > self.memory.len() {
            return Err("Program len is bigger than memory len");
        }
        for i in 0..op_codes.len() {
            self.memory[i] = op_codes[i].into();
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
        let statement = Success;
        self.last_statement = Some(statement);
        statement
    }

    #[inline]
    fn increment_pc_n(&mut self, n: usize) -> Statement {
        self.pc = self.pc.wrapping_add(n) % self.memory.len();
        Success
    }

    #[inline]
    fn increment_pc(&mut self) -> Statement {
        self.increment_pc_n(1)
    }

    #[inline]
    fn set_nz(&mut self, val: u8) -> Statement {
        self.nz = val != 0;
        Success
    }

    #[inline]
    fn decrement_sp(&mut self) -> Statement {
        self.pc.wrapping_sub(1) % self.memory.len();
        Success
    }

    #[inline]
    fn increment_sp(&mut self) -> Statement {
        self.sp = self.sp.wrapping_add(1) % self.memory.len();
        Success
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
            HALT   => HaltInstruction,
            IN     => self.execute(NOP, input, output),
            OUT    => self.execute(NOP, input, output),
            POP    => {
                let val = self.memory[self.sp];
                self.set_nz(val);
                self.increment_sp();
                self.increment_sp()
            },
            DUP    => {
                let tmp = self.memory[self.sp];
                self.decrement_sp();
                self.memory[self.sp] = tmp;
                self.set_nz(tmp);
                self.increment_sp()
            },
            PUSHPC => self.execute(NOP, input, output),
            POPPC  => self.execute(NOP, input, output),
            POPSP  => self.execute(NOP, input, output),
            SPTGT  => self.execute(NOP, input, output),
            PUSHNZ => self.execute(NOP, input, output),
            SWAP   => self.execute(NOP, input, output),
            PUSH0  => self.execute(NOP, input, output),
            ADD    => self.execute(NOP, input, output),
            SUB    => self.execute(NOP, input, output),
            INC    => self.execute(NOP, input, output),
            DEC    => self.execute(NOP, input, output),
            MUL    => self.execute(NOP, input, output),
            DIV    => self.execute(NOP, input, output),
            XOR    => self.execute(NOP, input, output),
            AND    => self.execute(NOP, input, output),
            OR     => self.execute(NOP, input, output),
            SHL    => self.execute(NOP, input, output),
            SHR    => self.execute(NOP, input, output),
            NOT    => self.execute(NOP, input, output),
            BZ     => self.execute(NOP, input, output),
            BNZ    => self.execute(NOP, input, output),
            BEQ    => self.execute(NOP, input, output),
            BGT    => self.execute(NOP, input, output),
            BLT    => self.execute(NOP, input, output),
            BGE    => self.execute(NOP, input, output),
            LOOP   => self.execute(NOP, input, output),
            ENDL   => self.execute(NOP, input, output),
            BRAN   => self.execute(NOP, input, output),
            BRAP   => self.execute(NOP, input, output),
            TARGET => self.execute(NOP, input, output),
            SKIP1  => self.increment_pc_n(2),
            SKIP2  => self.increment_pc_n(3),
            SKIP3  => self.increment_pc_n(4),
            SKIP4  => self.increment_pc_n(5),
            SKIP5  => self.increment_pc_n(6),
            SKIP6  => self.increment_pc_n(7),
            SKIP7  => self.increment_pc_n(8),
            SKIP8  => self.increment_pc_n(9),
            SKIP9  => self.increment_pc_n(10),
            NOP | _ => self.increment_pc(),
        }
    }

    /// Use [Empty](https://doc.rust-lang.org/std/io/struct.Empty.html) and/or
    /// [Sink](https://doc.rust-lang.org/std/io/struct.Sink.html)
    /// if you don't want to give input and/or output.
    pub fn step<R: Read, W: Write>(&mut self, input: &mut R, output: &mut W) -> Statement {
        let instr = self.memory[self.pc];
        let statement = self.execute(instr, input, output);
        self.last_statement = Some(statement);
        statement
    }

    /// Get a debug struct that can help for debugging programs
    pub fn debug_infos(&self) -> DebugInfos {
       DebugInfos {
            memory: OpCodes(self.memory.clone()).into(),
            pc: self.pc,
            sp: self.sp,
            nz: self.nz,
            statement: self.last_statement
        }
    }
}
