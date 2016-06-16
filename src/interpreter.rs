use std::io::{Read, Write};
use super::Instruction;
use super::Instruction::*;
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
    fn move_pc_n(&mut self, n: usize) {
        self.pc = self.pc.wrapping_add(n) % self.memory.len();
    }

    #[inline]
    fn move_pc(&mut self) {
        self.move_pc_n(1); // FIXME cod it here
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

    fn execute(&mut self, instr: Instruction) -> Statement {
        match instr {
            Nop    => self.move_pc(),
            Reset  => {
                self.pc = 0;
                self.sp = 0;
                self.nz = false;
            },
            Halt   => return Statement::HaltInstruction,
            In     => { self.execute(Nop); }
            Out    => { self.execute(Nop); }
            Pop    => { self.execute(Nop); }
            Dup    => { self.execute(Nop); }
            PushPc => { self.execute(Nop); }
            PopPc  => { self.execute(Nop); }
            PopSp  => { self.execute(Nop); }
            SpTgt  => { self.execute(Nop); }
            PushNz => { self.execute(Nop); }
            Swap   => { self.execute(Nop); }
            Push0  => { self.execute(Nop); }
            Add    => { self.execute(Nop); }
            Sub    => { self.execute(Nop); }
            Inc    => { self.execute(Nop); }
            Dec    => { self.execute(Nop); }
            Mul    => { self.execute(Nop); }
            Div    => { self.execute(Nop); }
            Xor    => { self.execute(Nop); }
            And    => { self.execute(Nop); }
            Or     => { self.execute(Nop); }
            Shl    => { self.execute(Nop); }
            Shr    => { self.execute(Nop); }
            Not    => { self.execute(Nop); }
            Bz     => { self.execute(Nop); }
            Bnz    => { self.execute(Nop); }
            Beq    => { self.execute(Nop); }
            Bgt    => { self.execute(Nop); }
            Blt    => { self.execute(Nop); }
            Bge    => { self.execute(Nop); }
            Loop   => { self.execute(Nop); }
            EndL   => { self.execute(Nop); }
            BraN   => { self.execute(Nop); }
            BraP   => { self.execute(Nop); }
            Target => { self.execute(Nop); }
            Skip1  => self.move_pc_n(2),
            Skip2  => self.move_pc_n(3),
            Skip3  => self.move_pc_n(4),
            Skip4  => self.move_pc_n(5),
            Skip5  => self.move_pc_n(6),
            Skip6  => self.move_pc_n(7),
            Skip7  => self.move_pc_n(8),
            Skip8  => self.move_pc_n(9),
            Skip9  => self.move_pc_n(10),
        };
        Statement::Success
    }

    // FIXME that's a big loose of time to convert into Instruction type
    pub fn step(&mut self) -> Statement {
        let instr = self.memory[self.pc].into();
        self.execute(instr)
    }
}
