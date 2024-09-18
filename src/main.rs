use std::fs;

use stack_machine::interpreter::InterpreterEvent;
use stack_machine::{
    assembler::assemble_string_to_bytes, interpreter::Interpreter,
    parser::parse_bytes_to_instructions,
};

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();
    let file_path = args.next().unwrap();
    let assembly_txt = fs::read_to_string(file_path).expect("Error loading assembly file");
    let bytecode = assemble_string_to_bytes(&assembly_txt);
    let start_time = std::time::Instant::now();
    let instructions = parse_bytes_to_instructions(&bytecode);
    let mut interpreter = Interpreter::new(instructions);
    let mut silent_toggle = false;
    loop {
        let res = interpreter.next_instruction();
        match res {
            Ok(InterpreterEvent::ProgramEnd) => break,
            Ok(InterpreterEvent::Silent(s)) => {
                silent_toggle = s;
            }
            Ok(r) => {
                // println!("Result: {r:?}");
                // if !silent_toggle {
                //     println!("Stack: {:?}", interpreter.debug_get_stack());
                // }
            }
            Err(e) => panic!("{e:?}"),
        }
    }
    let duration = start_time.elapsed().as_millis();
    println!("Memory {:?}", interpreter.debug_get_memory());
    println!("Took {duration}ms");
}
