#[macro_use] extern crate colorify;
#[macro_use] extern crate nom;
extern crate rustyline;
extern crate reustmann;

mod command;

use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use command::Command;
use reustmann::Interpreter;

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
    let mut interpreter = Interpreter::new(arch_length, arch_width).unwrap();

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
                    Ok(Command::Step(count)) => {
                        // let mut n = 0; // FIXME a Rust way can be better !
                        // let n = for i in 0..count {
                        //     // n = i;
                        //     break i;
                        // };
                        // let debug = interpreter.debug_step();

                        println!("executed {} steps", count);
                        // println!("debug: {}", debug)
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
