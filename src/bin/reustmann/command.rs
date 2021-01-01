use std::borrow::Cow;
use std::str::{self, FromStr};

#[derive(Debug, Clone)]
pub enum Command {
    UnsetInterpreter,
    SetInterpreter {
        arch_length: usize,
        arch_width: usize
    },
    InfosInterpreter,
    Infos,
    Copy(String, bool),
    Reset,
    Step(usize),
    Repeat,
    Exit,
}

impl FromStr for Command {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        match iter.next() {
            Some("unset_interpreter") => Ok(Command::UnsetInterpreter),
            Some("interpreter") => {
                let arch_length = match iter.next().map(|s| s.parse::<usize>()) {
                    Some(Ok(value)) => value,
                    Some(Err(e)) => return Err(e.to_string().into()),
                    None => return Err("missing arch length".into()),
                };

                let arch_width = match iter.next().map(|s| s.parse::<usize>()) {
                    Some(Ok(value)) => value,
                    Some(Err(e)) => return Err(e.to_string().into()),
                    None => return Err("missing arch width".into()),
                };

                Ok(Command::SetInterpreter { arch_length, arch_width })
            },
            Some("infos_interpreter") => Ok(Command::InfosInterpreter),
            Some("infos") => Ok(Command::Infos),
            Some("copy") => {
                let file_name = iter.next().ok_or("missing file name")?;
                let skip_newline = match iter.next().map(|s| s.parse::<bool>()) {
                    Some(Ok(value)) => value,
                    Some(Err(e)) => return Err(e.to_string().into()),
                    None => true,
                };
                Ok(Command::Copy(file_name.to_string(), skip_newline))
            },
            Some("reset") => Ok(Command::Reset),
            Some("step") | Some("s") | Some("next") | Some("n") => {
                let count = match iter.next() {
                    Some(s) => s.parse::<usize>().map_err(|e| e.to_string())?,
                    None => 1,
                };
                Ok(Command::Step(count))
            },
            Some("repeat") | None => Ok(Command::Repeat),
            Some("exit") | Some("quit") | Some("q") => Ok(Command::Exit),
            Some(command) => Err(format!("invalid command {:?}", command).into()),
        }
    }
}
