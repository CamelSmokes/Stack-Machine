use std::fs;

use stack_machine::assembler;
use stack_machine::interpreter::InterpreterEvent;
use stack_machine::{interpreter::Interpreter, parser::parse_bytes_to_instructions};

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();
    let mode = args.next().unwrap().trim().to_uppercase();
    let file_path = args.next().unwrap();
    match mode.as_str() {
        "-A" => {
            let out_path = args.next().expect("No output file path provided");
            let assembly_txt = fs::read_to_string(file_path).expect("Error loading assembly file");
            let s = assembler::preprocessor::parse_to_statements(&assembly_txt).unwrap();
            let s = assembler::preprocessor::to_stage2(s).unwrap();
            let s = assembler::preprocessor::to_stage3(s).unwrap();
            println!("{:?}", s);
            let compiled = assembler::preprocessor::compile_statements(s).unwrap();
            let assembled = assembler::assemble_string_to_bytes(&compiled);
            println!("{assembled:?}");
            std::fs::write(out_path, assembled).unwrap();
        }
        "-R" => {
            let bytecode = std::fs::read(file_path).unwrap();
            let instructions = parse_bytes_to_instructions(&bytecode).unwrap();
            println!("{:?}", instructions);
            println!("Starting");
            let start_time = std::time::Instant::now();
            let mut interpreter = Interpreter::new(instructions);
            // let mut silent_toggle = false;
            loop {
                let res = interpreter.next_instruction();
                match res {
                    Ok(InterpreterEvent::ProgramEnd) => break,
                    // Ok(InterpreterEvent::Silent(s)) => {
                    //     silent_toggle = s;
                    // }
                    Ok(_) => {
                        // if !silent_toggle {
                        // println!("Stack: {:?}", interpreter.debug_get_stack());
                        // }
                    }
                    Err(e) => panic!("{e:?}"),
                }
            }
            let duration = start_time.elapsed().as_millis();
            // println!("Memory {:?}", interpreter.debug_get_memory());
            println!("Took {duration}ms");
        }
        _ => panic!("Unknown mode {}", mode),
    };
}
