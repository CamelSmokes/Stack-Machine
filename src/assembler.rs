use std::collections::HashMap;

use crate::opcode::Opcode;

// Temporary assembler to easily create bytecode
pub fn assemble_string_to_bytes(input: &str) -> Vec<u8> {
    let mut bytecode = Vec::new();
    let mut goto_labels_usage: Vec<(String, usize)> = Vec::new();
    let mut goto_label_destination_location: HashMap<String, usize> = HashMap::new();
    for (line_no, line) in input.lines().enumerate() {
        let mut line_iter = line.split_whitespace();

        if line.trim().is_empty() {
            continue;
        }
        // Parse loop label
        if line.starts_with(':') {
            goto_label_destination_location.insert(line.to_owned(), bytecode.len());
            continue;
        } else if line.starts_with("//") {
            continue;
        } else if line.starts_with("print") {
            let (a, rest) = line.split_once(' ').unwrap();
            assert_eq!(a, "print");
            bytecode.push(Opcode::Push.into());
            bytecode.extend_from_slice(&u64::to_le_bytes(1));
            bytecode.push(Opcode::DbgSilent.into());
            for char in rest.bytes() {
                bytecode.push(Opcode::Push.into());
                bytecode.extend_from_slice(&u64::to_le_bytes(char as u64));
                bytecode.push(Opcode::DebugChar.into());
            }
            bytecode.push(Opcode::Push.into());
            bytecode.extend_from_slice(&u64::to_le_bytes('\n' as u64));
            bytecode.push(Opcode::DebugChar.into());
            bytecode.push(Opcode::Push.into());
            bytecode.extend_from_slice(&u64::to_le_bytes(0));
            bytecode.push(Opcode::DbgSilent.into());

            continue;
        }

        let mnemonic = line_iter
            .next()
            .unwrap_or_else(|| panic!("Empty line at line no {line_no}"));
        let opcode: Opcode = mnemonic.try_into().unwrap();
        match opcode {
            Opcode::Push => {
                // while (bytecode.len() + 1) % 8 != 0 {
                //     bytecode.push(Opcode::NoOp.into());
                // }

                let param_str = line_iter.next().unwrap_or_else(|| {
                    panic!("Operation {mnemonic} requires parameter at line no {line_no}")
                });
                if let Ok(param) = param_str.parse::<u64>() {
                    // Determine which push instruction to use
                    if param == 0 {
                        bytecode.push(Opcode::Push0.into());
                    } else if param < 256 {
                        let byte = param as u8;
                        bytecode.push(Opcode::Push1.into());
                        bytecode.push(byte);
                    } else {
                        bytecode.push(Opcode::Push.into());

                        bytecode.extend_from_slice(&param.to_le_bytes());
                    }
                } else if param_str.starts_with(':') {
                    bytecode.push(Opcode::Push.into());
                    goto_labels_usage.push((param_str.trim().to_owned(), bytecode.len()));
                    // Placeholder
                    bytecode.extend_from_slice(&u64::to_le_bytes(0xDEADBEEFDEADBEEF));
                } else {
                    panic!("Could not parse parameter \"{param_str}\" to u64 or goto label at line no {line_no}.")
                }
            }
            Opcode::Push1 => {
                panic!("Found push1. Use push instead it handles this");
            }
            _ => bytecode.push(opcode.into()),
        }

        if let Some(v) = line_iter.next() {
            if !v.starts_with("//") {
                panic!("Found extra junk {v} at line {line_no} after instruction")
            }
        }
    }
    println!("{:?}", bytecode);
    // replace goto label placeholders
    for (label_name, location) in goto_labels_usage {
        let destination_location = goto_label_destination_location
            .get(&label_name)
            .cloned()
            .unwrap_or_else(|| panic!("Use of undeclared goto label: \"{}\"", label_name));
        bytecode[location..location + 8]
            .copy_from_slice(&u64::to_le_bytes(destination_location as u64));
    }
    bytecode
}

#[cfg(test)]
mod test {
    use super::assemble_string_to_bytes;
    const INPUT: &str = r#"PUSH 123
POP"#;
    #[test]
    fn test() {
        let out = assemble_string_to_bytes(INPUT);
        let expected = [0, 0, 0, 0, 0, 0, 0, 0, 123, 1];
        assert_eq!(out, expected);
    }
}
