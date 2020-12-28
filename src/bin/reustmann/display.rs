use std::fmt::Debug;
use reustmann::{DebugInfos, Statement, Interpreter};
use reustmann::instruction::{Instruction, LongMnemonic, Mnemonic, OpCode, is_valid_op_code};

fn is_visible(c: u8) -> bool {
    c >= 32 && c <= 126
}

pub fn display_statement(statement: Option<Statement>) {
    if let Some(statement) = statement {
        let Statement(op_code, is_success) = statement;
        let name: LongMnemonic = Into::<Instruction>::into(op_code).into();
        println!("Last instruction was '{}' and return '{}'.", name, is_success);
    }
}

pub fn format_program_counter(mem_addr: usize, offset: usize, op_code: OpCode) -> String {
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

pub fn format_stack_pointer(mem_addr: usize, value: u8) -> String {
    let mem_addr = format!(colorify!(blue: "{:>#06x}"), mem_addr);
    if is_visible(value) == true {
        let preview = value as char;
        format!("{} ({:#04x}, '{}')", mem_addr, value, preview)
    }
    else {
        format!("{} ({:#04x})", mem_addr, value)
    }
}

fn display_sides(instr: Option<(usize, (usize, &u8))>,
                 stack: Option<(usize, &u8)>,
                 indicators: bool) {

    let pc_side = if let Some((idx, (pc_addr, op_code))) = instr {
        let pc_side = format_program_counter(pc_addr, idx, *op_code);
        if indicators == true { format!("{} {}", colorify!(red: "pc"), pc_side) }
        else { format!("   {}", pc_side) }
    } else {
        format!("")
    };
    let sp_side = if let Some((sp_addr, value)) = stack {
        let sp_side = format_stack_pointer(sp_addr, *value);
        if indicators == true { format!("{} {}", colorify!(red: "sp"), sp_side) }
        else { format!("   {}", sp_side) }
    } else {
        format!("")
    };
    println!("{}    {}", pc_side, sp_side);
}

// FIXME ugly really !!!
pub fn display_infos<D: ?Sized + Debug>(debug_infos: &DebugInfos,
                                        number_of_cycles: usize,
                                        statement: Option<Statement>,
                                        output: &D,
                                        pc_lines: usize,
                                        sp_lines: usize) {

    // if let Some(output) = output {
        // let output = String::from_utf8_lossy(&output);
        println!("Output: {:?}", output);
    // }

    let &DebugInfos{ ref memory, pc, sp, nz } = debug_infos;
    println!("cycles: {}, pc: {}, sp: {}, nz: {}", number_of_cycles, pc, sp, nz);
    display_statement(statement);

    let mut instrs = (*memory).iter().enumerate().cycle().skip(pc).take(pc_lines).enumerate();
    let mut stack = (*memory).iter().enumerate().cycle().skip(sp).take(sp_lines);

    display_sides(instrs.next(), stack.next(), true);
    loop {
        match (instrs.next(), stack.next()) {
            (None, None) => break,
            (instr, stack) => display_sides(instr, stack, false),
        }
    }
}

pub fn display_interpreter_properties(interpreter: &Interpreter) {
    println!("Interpreter as an arch width of {} and an arch length of {}.",
        format!(colorify!(yellow: "{}"), interpreter.arch_width()),
        format!(colorify!(yellow: "{}"), interpreter.arch_length())
    );
}
