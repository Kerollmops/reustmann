use std::io::{Read, Write};
// use instruction::Instruction;
// use instruction::OpCode;
use instruction::op_codes::*;
use super::Program;

/// the statement the machine cna return
pub enum Statement {
    /// if the instruction was correctly executed
    Success,
    /// if halt instruction has been executed
    HaltInstruction,
    // other ideas ???
}

/// The main interpreter, execute instructions, read from input,
/// write to output
pub struct Interpreter<'a, R: Read + 'a, W: Write + 'a> {
    input: &'a R,
    output: &'a W,
    memory: Vec<u8>, // need to be on the stack
    word_size: usize,
    pc: usize,
    sp: usize,
    nz: bool
}

impl<'a, R: Read, W: Write> Interpreter<'a, R, W> {
    /// Construct a new Interpreter with an existing Program.
    ///
    /// Use [Empty](https://doc.rust-lang.org/std/io/struct.Empty.html) if you don't want to give input.
    ///
    /// Use [Sink](https://doc.rust-lang.org/std/io/struct.Sink.html) if you don't want to output data.
    pub fn new(program: &Program, input: &'a mut R, output: &'a mut W) -> Interpreter<'a, R, W> {
        let mut memory = Vec::with_capacity(program.instructions().len());
        for i in program.instructions() {
            memory.push((*i).into());
        }
        Interpreter {
            input: input,
            output: output,
            memory: memory,
            word_size: program.word_size(),
            pc: 0,
            sp: 0,
            nz: false
        }
    }

    /// get the memory of the machine at any step
    pub fn memory(&self) -> &[u8] {
        &self.memory
    }

    #[inline]
    fn increment_pc_n(&mut self, n: usize) {
        self.pc = self.pc.wrapping_add(n) % self.memory.len();
    }

    #[inline]
    fn inscrement_pc(&mut self) {
        self.increment_pc_n(1); // FIXME code it here
    }

    // fn push_on_stack(&mut self) {
    //     // write data
    //     self.sp = (self.sp + 1) % self.memory.len();
    // }

    fn pop_stack(&mut self) -> u8 {
        let word = self.memory[self.sp];
        self.sp = self.sp.wrapping_sub(1) % self.memory.len();
        word
    }

    fn execute(&mut self, op_code: OpCode) -> Statement {
        match op_code {
            RESET  => {
                self.pc = 0;
                self.sp = 0;
                self.nz = false;
            },
            HALT   => return Statement::HaltInstruction,
            IN     => { self.execute(NOP); }
            OUT    => { self.execute(NOP); }
            POP    => { self.execute(NOP); }
            DUP    => { self.execute(NOP); }
            PUSHPC => { self.execute(NOP); }
            POPPC  => { self.execute(NOP); }
            POPSP  => { self.execute(NOP); }
            SPTGT  => { self.execute(NOP); }
            PUSHNZ => { self.execute(NOP); }
            SWAP   => { self.execute(NOP); }
            PUSH0  => { self.execute(NOP); }
            ADD    => { self.execute(NOP); }
            SUB    => { self.execute(NOP); }
            INC    => { self.execute(NOP); }
            DEC    => { self.execute(NOP); }
            MUL    => { self.execute(NOP); }
            DIV    => { self.execute(NOP); }
            XOR    => { self.execute(NOP); }
            AND    => { self.execute(NOP); }
            OR     => { self.execute(NOP); }
            SHL    => { self.execute(NOP); }
            SHR    => { self.execute(NOP); }
            NOT    => { self.execute(NOP); }
            BZ     => { self.execute(NOP); }
            BNZ    => { self.execute(NOP); }
            BEQ    => { self.execute(NOP); }
            BGT    => { self.execute(NOP); }
            BLT    => { self.execute(NOP); }
            BGE    => { self.execute(NOP); }
            LOOP   => { self.execute(NOP); }
            ENDL   => { self.execute(NOP); }
            BRAN   => { self.execute(NOP); }
            BRAP   => { self.execute(NOP); }
            TARGET => { self.execute(NOP); }
            SKIP1  => self.increment_pc_n(2),
            SKIP2  => self.increment_pc_n(3),
            SKIP3  => self.increment_pc_n(4),
            SKIP4  => self.increment_pc_n(5),
            SKIP5  => self.increment_pc_n(6),
            SKIP6  => self.increment_pc_n(7),
            SKIP7  => self.increment_pc_n(8),
            SKIP8  => self.increment_pc_n(9),
            SKIP9  => self.increment_pc_n(10),
            NOP | _ => self.inscrement_pc(),
        };
        Statement::Success
    }

    // FIXME that's a big loose of time to convert into Instruction type
    pub fn step(&mut self) -> Statement {
        let instr = self.memory[self.pc].into();
        self.execute(instr)
    }
}
