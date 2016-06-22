use std::fmt::Debug;
use std::default::Default;
use std::io::{Read, Write};
use std::fs::File;
use std::error::Error;
use reustmann::{Interpreter, DebugInfos, Program, Statement};
use reustmann::instruction::op_codes;
use debugger_error::DebuggerError;
use command::Command;
use display;
use sink_debug::DebugWrite;

const DEFAULT_ARCH_WIDTH: usize = 8;

fn create_program_from_file(filename: &String, ignore_nl: bool) -> Result<Program, String> {
    let mut file = match File::open(filename) {
        Err(err) => return Err(err.description().into()),
        Ok(file) => file,
    };
    let program = match Program::new(&mut file, ignore_nl) {
        Err(err) => return Err(err.into()),
        Ok(program) => program,
    };
    Ok(program)
}

fn display_debugger_error(dbg_err: &DebuggerError) {
    match *dbg_err {
        DebuggerError::NoInterpreter => {
            printlnc!(red: "{}", dbg_err.description());
            printlnc!(yellow: "{}", "`interpreter [arch_length] [arch_width]` to create one")
        },
        DebuggerError::InterpreterCreation(_) => printlnc!(red: "{}", dbg_err.description()),
    }
}

pub struct Debugger {
    interpreter: Option<Interpreter>,
    number_of_cycles: usize,
    program_name: Option<String>,
    statement: Option<Statement>,
    pc_lines: usize,
    sp_lines: usize
}

impl Default for Debugger {
    fn default() -> Self {
        Debugger::new()
    }
}

impl Debugger {
    pub fn new() -> Debugger {
        Debugger {
            interpreter: None,
            number_of_cycles: 0,
            program_name: None,
            statement: None,
            pc_lines: 10, // FIXME pc_lines need to be always >= sp_lines
            sp_lines: 5
        }
    }

    pub fn execute<R: ?Sized + Read, W: ?Sized + DebugWrite>(&mut self, command: &Command, input: &mut R, output: &mut W) /*-> Result<x, y>*/ {
        match *command {
            Command::UnsetInterpreter => {
                match self.unset_interpreter() {
                    Ok(_) => printlnc!(yellow: "Interpreter correctly unset."),
                    Err(err) => display_debugger_error(&err),
                }
            }
            Command::InfosInterpreter => {
                match self.interpreter() {
                    Ok(interpreter) => display::display_interpreter_properties(interpreter),
                    Err(err) => display_debugger_error(&err),
                }
            },
            Command::SetInterpreter{ arch_length, arch_width } => {
                match self.set_interpreter(arch_length, arch_width) {
                    Ok(_) => {
                        printlnc!(yellow: "Interpreter created.");
                        if let Ok(ref interpreter) = self.interpreter() {
                            display::display_interpreter_properties(interpreter);
                        }
                    },
                    Err(err) => display_debugger_error(&err),
                }
            }
            Command::Infos => {
                if let Some(ref filename) = self.program_name {
                    println!("Program in execution: '{}'.", filename);
                }
                match self.debug_infos() {
                    Ok(debug) => self.display_infos(&debug, output),
                    Err(err) => display_debugger_error(&err),
                }
            },
            Command::Copy(ref filename, ignore_nl) => {
                self.program_name = Some(filename.clone());
                match create_program_from_file(&filename, ignore_nl) {
                    Err(err) => printlnc!(red: "{}", err),
                    Ok(program) => {
                        match self.copy_program_and_reset(&program) {
                            Err(_) => { // FIXME if another error than no_interpreter ?!?!
                                let arch_length = program.memory().len();
                                match self.set_interpreter(arch_length, DEFAULT_ARCH_WIDTH) {
                                    Ok(_) => {
                                        printlnc!(yellow: "Interpreter created.");
                                        if let Ok(ref interpreter) = self.interpreter() {
                                            display::display_interpreter_properties(interpreter);
                                        }
                                    },
                                    Err(err) => display_debugger_error(&err),
                                }
                                self.copy_program_and_reset(&program).unwrap();
                                match self.debug_infos() {
                                    Ok(debug) => self.display_infos(&debug, output),
                                    Err(err) => display_debugger_error(&err),
                                }
                            },
                            Ok(_) => {
                                printlnc!(yellow: "Program correctly loaded.");
                                match self.debug_infos() {
                                    Ok(debug) => self.display_infos(&debug, output),
                                    Err(err) => display_debugger_error(&err),
                                }
                            },
                        }
                    },
                }
            },
            Command::Reset => {
                match self.reset() {
                    Ok(stat) => {
                        printlnc!(yellow: "Reset.");
                        self.statement = Some(stat);
                        match self.debug_infos() {
                            Ok(debug) => self.display_infos(&debug, output),
                            Err(err) => display_debugger_error(&err),
                        }
                    },
                    Err(err) => display_debugger_error(&err),
                }
            },
            Command::Step(to_execute) => {
                match self.steps(to_execute, input, output) {
                    Ok((executed, debug, stat)) => {
                        self.statement = stat;
                        match executed == to_execute {
                            true => printlnc!(yellow: "{} steps executed.", executed),
                            false => printlnc!(yellow: "{}/{} steps executed.", executed, to_execute),
                        }
                        self.display_infos(&debug, output)
                    },
                    Err(err) => display_debugger_error(&err),
                }
            },
            Command::Exit | Command::Repeat => unreachable!(),
        };
    }

    fn set_interpreter(&mut self, arch_length: usize, arch_width: usize) -> Result<(), DebuggerError> {
        let interpreter = match Interpreter::new(arch_length, arch_width) {
            Err(err) => return Err(DebuggerError::InterpreterCreation(err)),
            Ok(interpreter) => interpreter
        };
        self.interpreter = Some(interpreter);
        Ok(())
    }

    fn unset_interpreter(&mut self) -> Result<(), DebuggerError> {
        if let None = self.interpreter {
            Err(DebuggerError::NoInterpreter)
        }
        else {
            self.interpreter = None;
            Ok(())
        }
    }

    fn interpreter(&self) -> Result<&Interpreter, DebuggerError> {
        match self.interpreter {
            Some(ref interpreter) => Ok(interpreter),
            None => Err(DebuggerError::NoInterpreter),
        }
    }

    fn copy_program_and_reset(&mut self, program: &Program) -> Result<(), DebuggerError> {
        if let Some(ref mut interpreter) = self.interpreter {
            interpreter.copy_program(program);
            interpreter.reset();
            Ok(())
        }
        else { Err(DebuggerError::NoInterpreter) }
    }

    fn reset(&mut self) -> Result<Statement, DebuggerError> {
        if let Some(ref mut interpreter) = self.interpreter {
            Ok(interpreter.reset())
        }
        else { Err(DebuggerError::NoInterpreter) }
    }

    fn steps<R: ?Sized + Read, W: ?Sized + Write>(&mut self, steps: usize, input: &mut R, output: &mut W)
        -> Result<(usize, DebugInfos, Option<Statement>), DebuggerError> {

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
                self.number_of_cycles += executed;
            }
            Ok((executed, interpreter.debug_infos(), statement))
        }
        else { Err(DebuggerError::NoInterpreter) }
    }

    fn display_infos<D: ?Sized + Debug>(&self, debug_infos: &DebugInfos, output: &D) {
        display::display_infos(debug_infos,
                               self.number_of_cycles,
                               self.statement,
                               output,
                               self.pc_lines,
                               self.sp_lines)
    }

    fn debug_infos(&self) -> Result<DebugInfos, DebuggerError> {
        if let Some(ref interpreter) = self.interpreter {
            Ok(interpreter.debug_infos())
        }
        else { Err(DebuggerError::NoInterpreter) }
    }
}
