use std::borrow::Cow;
use std::str::{self, FromStr};
use nom::{IResult, space, digit, is_space};

#[derive(Debug, Clone)]
pub enum Command {
    UnsetInterpreter,
    SetInterpreter {
        arch_length: usize,
        arch_width: usize
    },
    InfosInterpreter,
    Infos,
    Copy(String, bool), // FIXME use Cow like in rustendo64
    Reset,
    Step(usize),
    Repeat,
    Exit,
}

named!(
    command<Command>,
    terminated!(
        alt_complete!(
            exit |
            set_interpreter |
            unset_interpreter |
            infos_interpreter |
            infos |
            copy |
            reset |
            step |
            repeat
        ),
        eof!() // force eof after matching command
    )
);

named!(
    infos_interpreter<Command>,
    map!(
        alt_complete!(tag!("interpreter") | tag!("inter")),
        |_|  Command::InfosInterpreter
    )
);

named!(
    unset_interpreter<Command>,
    map!(
        alt_complete!(tag!("unset_interpreter") | tag!("unsetinter") | tag!("unsetint")),
        |_|  Command::UnsetInterpreter
    )
);

named!(
    set_interpreter<Command>,
    chain!(
        alt_complete!(tag!("set_interpreter") | tag!("interpreter") | tag!("setint")) ~
            length: preceded!(space, usize_parser) ~
            width: preceded!(space, usize_parser),
        ||  Command::SetInterpreter {
                arch_length: length,
                arch_width: width,
            }
    )
);

named!(
    infos<Command>,
    map!(
        alt_complete!(tag!("infos") | tag!("info") | tag!("i")),
        |_| Command::Infos
    )
);

named!(
    copy<Command>,
    chain!(
        alt_complete!(tag!("load") | tag!("copy")) ~
            filename: preceded!(space, literal_string),
        || {
            let string = unsafe{ String::from_utf8_unchecked(filename.into()) };
            Command::Copy(string, true) // TODO get last loaded file by default, use Option
        }
    )
);

named!(
    reset<Command>,
    map!(
        alt_complete!(tag!("reset") | tag!("r")),
        |_| Command::Reset
    )
);

named!(
    step<Command>,
    chain!(
        alt_complete!(tag!("step") | tag!("next") | tag!("s") | tag!("n")) ~
            count: opt!(preceded!(space, usize_parser)),
        || Command::Step(count.unwrap_or(1))
    )
);

named!(
    exit<Command>,
    map!(
        alt_complete!(tag!("exit") | tag!("quit") | tag!("e") | tag!("q")),
        |_| Command::Exit
    )
);

named!(
    repeat<Command>,
    value!(Command::Repeat)
);

named!(
    usize_parser<usize>,
    map_res!(
        map_res!(digit, str::from_utf8),
        FromStr::from_str
    )
);

named!(double_quote,
    delimited!(
        char!('"'),
        is_not!("\""),
        char!('"')
    )
);

named!(literal_string,
    chain!(
        c: alt_complete!(
            double_quote |
            take_while!(call!(|c| !is_space(c)))
            // escaped!(call!(alpha), '\\', is_not!(space)) // TODO !!!
        ),
        || c
    )
);

impl FromStr for Command {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match command(s.as_bytes()) {
            IResult::Done(_, c) => Ok(c),
            err => Err(format!("Unable to parse command: {:?}", err).into())
        }
    }
}
