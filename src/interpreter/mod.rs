use std::io::{Read, Write};
use super::Program;

pub struct Interpreter<'a, R: Read + 'a, W: Write + 'a> {
    input: &'a R,
    output: &'a W,
    memory: Vec<u8>, // need to be on the stack
    pc: usize,
    sp: usize,
    nz: bool
}

impl<'a, R: Read, W: Write> Interpreter<'a, R, W> {
    /// construct a new Interpreter with an existing Program.
    /// Define the input/ouput to give to it.
    /// Please use [Empty](https://doc.rust-lang.org/std/io/struct.Empty.html) if you don't want to give input
    /// and use [Sink](https://doc.rust-lang.org/std/io/struct.Sink.html) if you don't want to output data.
    /// Define a fixed size memory area
    pub fn new(program: &Program, input: &'a mut R, output: &'a mut W) -> Interpreter<'a, R, W> {
        Interpreter {
            input: input,
            output: output,
            memory: Vec::new(),
            pc: 0,
            sp: 0,
            nz: false
        }
    }

    //
}
