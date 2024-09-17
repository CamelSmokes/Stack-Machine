use std::fs;

use stack_machine::interpreter::InterpreterEvent;
use stack_machine::{
    assembler::assemble_string_to_bytes, interpreter::Interpreter,
    parser::parse_bytes_to_instructions,
};

fn main() {
    let assembly_txt = fs::read_to_string("prog.xasm").expect("Error loading assembly file");
    let bytecode = assemble_string_to_bytes(&assembly_txt);
    let instructions = parse_bytes_to_instructions(&bytecode);
    let mut interpreter = Interpreter::new(instructions);
    loop {
        let res = interpreter.next_instruction();
        match res {
            Ok(InterpreterEvent::ProgramEnd) => break,
            Ok(r) => {
                // println!("Result: {r:?}");
                // println!("Stack: {:?}", interpreter.debug_get_stack());
            }
            Err(e) => panic!("{e:?}"),
        }
    }
}
