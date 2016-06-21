use std::default::Default;
use std::io::{Read, Write};
use reustmann::{Interpreter, DebugInfos, Program, Statement};
use reustmann::instruction::op_codes;

pub struct Debugger {
    interpreter: Option<Interpreter>
}

// FIXME create enum of errors types
const NO_INTERPRETER: &'static str = "No interpreter created (`interpreter [arch_length] [arch_width]` to create one)";

impl Debugger {
    pub fn new() -> Debugger {
        Debugger {
            interpreter: None
        }
    }

    pub fn set_interpreter(&mut self, arch_length: usize, arch_width: usize) -> Result<(), &'static str> {
        let interpreter = try!(Interpreter::new(arch_length, arch_width));
        self.interpreter = Some(interpreter);
        Ok(())
    }

    pub fn unset_interpreter(&mut self) -> Result<(), &'static str> {
        if let None = self.interpreter {
            Err(NO_INTERPRETER)
        }
        else {
            self.interpreter = None;
            Ok(())
        }
    }

    pub fn interpreter(&self) -> Result<&Interpreter, &'static str> {
        match self.interpreter {
            Some(ref interpreter) => Ok(interpreter),
            None => Err(NO_INTERPRETER),
        }
    }

    pub fn copy_program_and_reset(&mut self, program: &Program) -> Result<(), &'static str> {
        if let Some(ref mut interpreter) = self.interpreter {
            interpreter.copy_program(program);
            interpreter.reset();
            Ok(())
        }
        else { Err(NO_INTERPRETER) }
    }

    pub fn reset(&mut self) -> Result<Statement, &'static str> {
        if let Some(ref mut interpreter) = self.interpreter {
            Ok(interpreter.reset())
        }
        else { Err(NO_INTERPRETER) }
    }

    pub fn steps<R: Read, W: Write>(&mut self, steps: usize, input: &mut R, output: &mut W)
            -> Result<(usize, DebugInfos, Option<Statement>), &'static str> {

        if let Some(ref mut interpreter) = self.interpreter {
            let mut statement = None;
            let mut executed = 0;
            for i in 0..steps {
                statement = Some(interpreter.step(input, output));
                if let Some(statement) = statement {
                    match statement {
                        Statement(op_codes::HALT, _) => break,
                        _ => (),
                    }
                }
                executed = i + 1;
            }
            Ok((executed, interpreter.debug_infos(), statement))
        }
        else { Err(NO_INTERPRETER) }
    }

    pub fn debug_infos(&self) -> Result<DebugInfos, &'static str> {
        if let Some(ref interpreter) = self.interpreter {
            Ok(interpreter.debug_infos())
        }
        else { Err(NO_INTERPRETER) }
    }
}

impl Default for Debugger {
    fn default() -> Debugger {
        Debugger::new()
    }
}
