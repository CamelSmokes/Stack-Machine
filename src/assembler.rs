use std::collections::HashMap;

use crate::opcode::Opcode;

// Temporary assembler to easily create bytecode
pub fn assemble_string_to_bytes(input: &str) -> Vec<u8> {
    let mut bytecode = Vec::new();
    let mut goto_labels_usage: Vec<(String, usize)> = Vec::new();
    let mut goto_label_destination_location: HashMap<String, usize> = HashMap::new();
    for (line_no, line) in input.lines().enumerate() {
        let mut line_iter = line.split_whitespace();

        let mnemonic = line_iter
            .next()
            .unwrap_or_else(|| panic!("Empty line at line no {line_no}"));

        // Parse loop label
        if line.starts_with(':') {
            goto_label_destination_location.insert(line.to_owned(), bytecode.len());
            continue;
        }
        let opcode: Opcode = mnemonic.try_into().unwrap();
        let byte: u8 = opcode.into();

        bytecode.push(byte);
        if opcode == Opcode::Push {
            let param_str = line_iter.next().unwrap_or_else(|| {
                panic!("Operation {mnemonic} requires parameter at line no {line_no}")
            });
            if let Ok(param) = param_str.parse::<u64>() {
                bytecode.extend_from_slice(&param.to_be_bytes());
            } else if param_str.starts_with(':') {
                goto_labels_usage.push((param_str.to_owned(), bytecode.len()));
                // Placeholder
                bytecode.extend_from_slice(&u64::to_be_bytes(0xDEADBEEF));
            } else {
                panic!("Could not parse parameter \"{param_str}\" to u64 or goto label at line no {line_no}.")
            }
        }
    }
    // replace goto label placeholders
    for (label_name, location) in goto_labels_usage {
        let destination_location = goto_label_destination_location
            .get(&label_name)
            .cloned()
            .unwrap_or_else(|| panic!("Use of undeclared goto label: \"{}\"", label_name));
        bytecode[location..location + 8]
            .copy_from_slice(&u64::to_be_bytes(destination_location as u64));
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
