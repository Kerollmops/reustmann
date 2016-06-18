#[macro_use] extern crate colorify;
#[macro_use] extern crate nom;
extern crate rustyline;

use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use std::borrow::Cow;
use std::str::{self, FromStr};

use nom::{IResult, eof, space, digit};

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Steps(usize),
    Repeat,
    Exit,
}

named!(
    command<Command>,
    chain!(
        c: alt_complete!(
            exit |
            steps/* |
            repeat*/) ~
            eof,
    || c));

named!(
    steps<Command>,
    map!(
        alt_complete!(tag!("steps") | tag!("step") | tag!("s")),
        |_| Command::Steps(1)));

named!(
    exit<Command>,
    map!(
        alt_complete!(tag!("exit") | tag!("quit") | tag!("e") | tag!("q")),
        |_| Command::Exit));

impl FromStr for Command {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match command(s.as_bytes()) {
            IResult::Done(_, c) => Ok(c),
            err => Err(format!("Unable to parse command: {:?}", err).into())
        }
    }
}

fn main() {
    let file_comp = FilenameCompleter::new();
    let mut rustyline = Editor::new();

    rustyline.set_completer(Some(&file_comp));
    if let Err(_) = rustyline.load_history("history.txt") {
        println!("No previous history.");
    }

    loop {
        let prompt = format!(colorify!(dark_grey: "({}) "), "rmdb");
        let readline = rustyline.readline(&prompt);
        match readline {
            Ok(line) => {
                rustyline.add_history_entry(&line);
                match line.parse() {
                    Ok(Command::Exit) => break,
                    Ok(Command::Steps(n)) => println!("execute {}, steps", n),
                    Ok(Command::Repeat) => println!("repeat"),
                    Err(ref e) => println!("{}", e),
                };
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
