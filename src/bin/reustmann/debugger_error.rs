#[derive(Debug)]
pub enum DebuggerError {
    NoInterpreter,
    InterpreterCreation(&'static str)
}

impl DebuggerError {
    pub fn description(&self) -> &'static str {
        match *self {
            DebuggerError::NoInterpreter => "No interpreter created",
            DebuggerError::InterpreterCreation(err) => err
        }
    }
}
