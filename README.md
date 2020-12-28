# Reustmann - a Von Neumann architecture

Reustmann is a Von Neumann architecture in Rust.
I was inspired by [the dave miller Iota machine](http://www.millermattson.com/dave/?p=174).

I just recreate it in Rust.

The [Reustmann Documentation](https://docs.rs/reustmann).

## How to

The `hello_world.rm` program, make sure you don't add a final newline
```text
Gp..OOOOOOOOOOOOHTFello World!
```

First create a program, from a file for example
```rust
extern crate reustmann;

use std::fs::File;
use reustmann::Program;

let program = Program::from_file("./hello_world.rm").unwrap();
```

Then use this program to fill the interpreter memory
```rust
use reustmann::Interpreter;

let arch_length = 50; // memory length
let arch_width = 8; // word size

let mut interpreter = Interpreter::new(arch_length, arch_width).unwrap();
interpreter.copy_program(&program).unwrap();
```

Then you can run it by using an interpreter
```rust
use reustmann::Statement;
use reustmann::instruction::op_codes;
use std::io::{empty, stdout};
// use std::io::sink; // for no output

let mut input = empty(); // no input data needed
let mut output = stdout(); // output on the standard output

loop {
    // each interpreter step return a statement
    // while no `HALT` statement is found, we continue
    match interpreter.step(&mut input, &mut output) {
        Statement(op_codes::HALT, _) => break,
        _ => ()
    }
}
```

You can have debug informations at any moment
```rust
// put this in the previous match, in the right position!
println!("{:?}", interpreter.debug_infos());
```
