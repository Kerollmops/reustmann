#[macro_use] extern crate colorify;
#[macro_use] extern crate nom;
extern crate rustyline;
extern crate reustmann;

mod command;
mod debugger;

use std::io::{empty, sink};
use std::error::Error;
use std::fs::File;
use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use reustmann::{DebugInfos, Statement, Interpreter}; // FIXME move this elsewhere
use reustmann::instruction::{ Instruction, LongMnemonic, Mnemonic, OpCode, is_valid_op_code};
use command::Command;
use debugger::Debugger;
use reustmann::Program;

const DEFAULT_ARCH_WIDTH: usize = 8;

// FIXME found something better ???
fn is_visible(c: u8) -> bool {
    c >= 32 && c <= 126
}

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

// FIXME move this elsewhere
fn display_statement(statement: Option<Statement>) {
    if let Some(statement) = statement {
        let Statement(op_code, is_success) = statement;
        let name: LongMnemonic = Into::<Instruction>::into(op_code).into();
        println!("Last instruction was '{}' and return '{}'.", name, is_success);
    }
}

fn format_program_counter(mem_addr: usize, offset: usize, op_code: OpCode) -> String {
    let instr: Instruction = op_code.into();
    let longmnemo: LongMnemonic = instr.into();
    let mem_addr = format!(colorify!(blue: "{:>#06x}"), mem_addr);

    let (op_code, longmnemo) = if is_valid_op_code(op_code) {
        let op = format!("{:#04x},  {} ", op_code, Into::<Mnemonic>::into(instr));
        let name = format!(colorify!(green: "{:<6}"), longmnemo);
        (op, name)
    } else {
        let op = format!("{:#04x}, '{}'", op_code, op_code as char);
        let name = format!(colorify!(red: "{:<6}"), longmnemo);
        (op, name)
    };

    format!("{} <{:+}>: {} ({})", mem_addr, offset, longmnemo, op_code)
}

fn format_stack_pointer(mem_addr: usize, value: u8) -> String {
    let mem_addr = format!(colorify!(blue: "{:>#06x}"), mem_addr);
    if is_visible(value) == true {
        let preview = value as char;
        format!("{} ({:#04x}, '{}')", mem_addr, value, preview)
    }
    else {
        format!("{} ({:#04x})", mem_addr, value)
    }
}

// FIXME move this elsewhere
fn display_infos(debug_infos: &DebugInfos, statement: Option<Statement>, output: &Vec<u8>) {

    // if let Some(output) = output {
        let output = String::from_utf8_lossy(&output);
        println!("Output({}): '{}'", output.len(), output);
    // }

    let &DebugInfos{ ref memory, pc, sp, nz } = debug_infos;
    println!("pc: {}, sp: {}, nz: {}", pc, sp, nz);
    display_statement(statement);

    // FIXME don't zip, display different number of stack/instructions
    let lines = 10;

    let instrs = (*memory).iter().enumerate().cycle().skip(pc).take(lines).enumerate();
    let stack = (*memory).iter().enumerate().cycle().skip(sp).take(lines);
    let mut pc_sp = instrs.zip(stack);

    if let Some(((idx, (pc_addr, op_code)), (sp_addr, value))) = pc_sp.next() {
        let pc_side = format_program_counter(pc_addr, idx, *op_code);
        let pc_side = format!("{} {}", colorify!(red: "pc"), pc_side);
        let sp_side = format_stack_pointer(sp_addr, *value);
        let sp_side = format!("{} {}", colorify!(red: "sp"), sp_side);
        println!("{}    {}", pc_side, sp_side);
    }

    for ((idx, (pc_addr, op_code)), (sp_addr, value)) in pc_sp {
        let pc_side = format_program_counter(pc_addr, idx, *op_code);
        let pc_side = format!("   {}", pc_side);
        let sp_side = format_stack_pointer(sp_addr, *value);
        let sp_side = format!("   {}", sp_side);
        println!("{}    {}", pc_side, sp_side);
    }
}

fn display_interpreter_properties(interpreter: &Interpreter) {
    println!("Interpreter as an arch width of {} and an arch length of {}.",
        format!(colorify!(yellow: "{}"), interpreter.arch_width()),
        format!(colorify!(yellow: "{}"), interpreter.arch_length())
    );
}

fn main() {
    let file_comp = FilenameCompleter::new();
    let mut rustyline = Editor::new();

    rustyline.set_completer(Some(&file_comp));
    if let Err(_) = rustyline.load_history("history.txt") {
        printlnc!(yellow: "No previous history.");
    }

    let mut program_in_execution = None;
    let mut last_command = None;
    let mut dbg = Debugger::new(); // FIXME use default

    if let Ok(ref interpreter) = dbg.interpreter() {
        display_interpreter_properties(interpreter);
    }

    // let mut input = empty();
    let mut input = "\x02Hello".as_bytes();
    // let mut output = sink();
    let mut output = Vec::new();

    let mut statement = None;

    loop {
        let prompt = format!(colorify!(dark_grey: "({}) "), "rmdb");
        let readline = rustyline.readline(&prompt);
        match readline {
            Ok(line) => {
                rustyline.add_history_entry(&line);

                let command = match (line.parse(), last_command) {
                    (Ok(Command::Repeat), Some(c)) => Ok(c),
                    (Ok(Command::Repeat), None) => Err("No last command.".into()),
                    (Ok(c), _) => Ok(c),
                    (Err(e), _) => Err(e),
                };

                match command {
                    Ok(Command::UnsetInterpreter) => {
                        match dbg.unset_interpreter() {
                            Ok(_) => printlnc!(yellow: "Interpreter correctly unset."),
                            Err(err) => printlnc!(red: "{}", err),
                        }
                    }
                    Ok(Command::InfosInterpreter) => {
                        match dbg.interpreter() {
                            Ok(interpreter) => display_interpreter_properties(interpreter),
                            Err(err) => printlnc!(red: "{}", err),
                        }
                    },
                    Ok(Command::SetInterpreter{ arch_length, arch_width }) => {
                        match dbg.set_interpreter(arch_length, arch_width) {
                            Ok(_) => {
                                printlnc!(yellow: "Interpreter created.");
                                if let Ok(ref interpreter) = dbg.interpreter() {
                                    display_interpreter_properties(interpreter);
                                }
                            },
                            Err(err) => printlnc!(red: "{}", err),
                        }
                    }
                    Ok(Command::Infos) => {
                        if let Some(ref filename) = program_in_execution {
                            println!("Program in execution: '{}'.", filename);
                        }
                        match dbg.debug_infos() {
                            Ok(debug) => display_infos(&debug, statement, &output),
                            Err(err) => printlnc!(red: "{}", err),
                        }
                    },
                    Ok(Command::Copy(ref filename, ignore_nl)) => {
                        program_in_execution = Some(filename.clone());
                        match create_program_from_file(&filename, ignore_nl) {
                            Err(err) => printlnc!(red: "{}", err),
                            Ok(program) => {
                                match dbg.copy_program_and_reset(&program) {
                                    Err(_) => { // FIXME if another error than no_interpreter ?!?!
                                        let arch_length = program.memory().len();
                                        match dbg.set_interpreter(arch_length, DEFAULT_ARCH_WIDTH) {
                                            Ok(_) => {
                                                printlnc!(yellow: "Interpreter created.");
                                                if let Ok(ref interpreter) = dbg.interpreter() {
                                                    display_interpreter_properties(interpreter);
                                                }
                                            },
                                            Err(err) => printlnc!(red: "{}", err),
                                        }
                                        dbg.copy_program_and_reset(&program).unwrap();
                                        match dbg.debug_infos() {
                                            Ok(debug) => display_infos(&debug, statement, &output),
                                            Err(err) => printlnc!(red: "{}", err),
                                        }
                                    },
                                    Ok(_) => {
                                        printlnc!(yellow: "Program correctly loaded.");
                                        match dbg.debug_infos() {
                                            Ok(debug) => display_infos(&debug, statement, &output),
                                            Err(err) => printlnc!(red: "{}", err),
                                        }
                                    },
                                }
                            },
                        }
                    },
                    Ok(Command::Reset) => {
                        match dbg.reset() {
                            Ok(stat) => {
                                printlnc!(yellow: "Reset.");
                                statement = Some(stat);
                                match dbg.debug_infos() {
                                    Ok(debug) => display_infos(&debug, statement, &output),
                                    Err(err) => printlnc!(red: "{}", err),
                                }
                            },
                            Err(err) => printlnc!(red: "{}", err),
                        }
                    },
                    Ok(Command::Step(to_execute)) => {
                        match dbg.steps(to_execute, &mut input, &mut output) {
                            Ok((executed, debug, stat)) => {
                                statement = stat;
                                match executed == to_execute {
                                    true => printlnc!(yellow: "{} steps executed.", executed),
                                    false => printlnc!(yellow: "{}/{} steps executed.", executed, to_execute),
                                }
                                display_infos(&debug, statement, &output)
                            },
                            Err(err) => printlnc!(red: "{}", err),
                        }
                    },
                    Ok(Command::Exit) => break,
                    Ok(Command::Repeat) => unreachable!(),
                    Err(ref e) => printlnc!(red: "{}", e),
                    // Err(_) => printlnc!(red: "Unrecognized command '{}'.", command),
                };
                last_command = command.ok();
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rustyline.save_history("history.txt").unwrap();
}
