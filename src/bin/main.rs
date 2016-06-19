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
use command::Command;
use debugger::Debugger;
use reustmann::Program;

fn create_program_from_file(filename: &String) -> Result<Program, String> {
    let mut file = match File::open(filename) {
        Err(err) => return Err(err.description().into()),
        Ok(file) => file,
    };
    let program = match Program::new(&mut file, true) { // TODO option for this
        Err(err) => return Err(err.into()),
        Ok(program) => program,
    };
    Ok(program)
}

fn main() {
    let file_comp = FilenameCompleter::new();
    let mut rustyline = Editor::new();

    rustyline.set_completer(Some(&file_comp));
    if let Err(_) = rustyline.load_history("history.txt") {
        printlnc!(yellow: "No previous history.");
    }

    let mut last_command = None;

    let arch_length = 15; // TODO get input source length by default
    let arch_width = 8;
    // FIXME don't unwrap
    let mut debugger = Debugger::new(arch_length, arch_width).unwrap();
    let mut empty_input = empty();
    let mut sink_output = sink();

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
                    Ok(Command::Copy(ref filename)) => {
                        match create_program_from_file(&filename) {
                            Err(err) => printlnc!(red: "{}", err),
                            Ok(program) => {
                                if let Err(err) = debugger.copy_program_and_reset(&program) {
                                    printlnc!(red: "{}", err)
                                }
                            },
                        }
                    },
                    Ok(Command::Reset) => {
                        debugger.reset();
                        printlnc!(yellow: "reset.");
                        println!("{}", debugger.debug_infos())
                    },
                    Ok(Command::Step(to_execute)) => {
                        let (executed, debug) = debugger.steps(to_execute, &mut empty_input, &mut sink_output);
                        if executed == to_execute {
                            printlnc!(yellow: "{} steps executed.", executed);
                        }
                        else {
                            printlnc!(yellow: "{} steps executed on {}.", executed, to_execute);
                        }
                        println!("{}", debug);
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
