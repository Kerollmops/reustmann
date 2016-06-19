use std::io::{Read, Write};
use reustmann::{Interpreter, DebugInfos, Program, Statement};

pub struct Debugger {
    interpreter: Interpreter
}

impl Debugger {
    pub fn new(arch_length: usize, arch_width: usize) -> Result<Debugger, &'static str> {
        let interpreter = try!(Interpreter::new(arch_length, arch_width));
        Ok(Debugger {
            interpreter: interpreter
        })
    }

    pub fn copy_program_and_reset(&mut self, program: &Program) -> Result<(), &'static str> {
        try!(self.interpreter.copy_program(program));
        self.interpreter.reset();
        Ok(())
    }

    pub fn reset(&mut self) -> Statement {
        self.interpreter.reset()
    }

    pub fn steps<R: Read, W: Write>(&mut self, steps: usize, input: &mut R, output: &mut W) -> (usize, DebugInfos) {
        let mut executed = 0;
        for i in 0..steps {
            match self.interpreter.step(input, output) {
                Statement::Success => (),
                _ => break,
            }
            executed = i + 1;
        }
        (executed, self.interpreter.debug_infos())
    }

    pub fn debug_infos(&self) -> DebugInfos {
        self.interpreter.debug_infos()
    }
}
