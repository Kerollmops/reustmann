#[macro_use] extern crate colorify;
extern crate rustyline;
extern crate reustmann;

mod command;
mod debugger;
mod debugger_error;
mod sink_debug;
mod display;

use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use command::Command;
use debugger::Debugger;

fn main() {
    let file_comp = FilenameCompleter::new();
    let mut rustyline = Editor::new();

    rustyline.set_completer(Some(&file_comp));
    if let Err(_) = rustyline.load_history("history.txt") {
        printlnc!(yellow: "No previous history.");
    }

    let mut last_command = None;
    let mut dbg = Debugger::new();

    // FIXME do this elsewhere
    // if let Ok(ref interpreter) = dbg.interpreter() {
    //     display::display_interpreter_properties(interpreter);
    // }

    // let mut input = std::io::empty();
    let mut input = "\nHello".as_bytes();

    // let mut output = sink_debug::sink_debug();
    let mut output = Vec::<u8>::new();

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
                    Ok(Command::Exit) => break,
                    Ok(Command::Repeat) => unreachable!(),
                    Ok(ref command) => dbg.execute(&command, &mut input, &mut output), // FIXME retrieve error
                    Err(ref e) => printlnc!(red: "{}", e),
                }
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
