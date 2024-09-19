pub mod preprocessor;
use crate::opcode::Opcode;

pub fn assemble_string_to_bytes(input: &str) -> Vec<u8> {
    let mut bytecode = Vec::new();
    for (line_no, line) in input.lines().enumerate() {
        let mut line_iter = line.split_whitespace();

        let mnemonic = line_iter
            .next()
            .unwrap_or_else(|| panic!("Empty line at line no {line_no}"));
        let opcode: Opcode = mnemonic.try_into().unwrap();
        match opcode {
            Opcode::Push8 => {
                let param_str = line_iter.next().unwrap_or_else(|| {
                    panic!("Operation {mnemonic} requires parameter at line no {line_no}")
                });

                let param = param_str.parse::<u64>().expect("Could not parse number");
                bytecode.push(Opcode::Push8.into());
                bytecode.extend_from_slice(&param.to_le_bytes());
            }
            Opcode::Push1 => {
                let param_str = line_iter.next().unwrap_or_else(|| {
                    panic!("Operation {mnemonic} requires parameter at line no {line_no}")
                });
                let param = param_str.parse::<u64>().expect("Could not parse number");
                if param >= 256 {
                    todo!()
                }
                bytecode.push(Opcode::Push1.into());

                let byte = param as u8;
                bytecode.extend_from_slice(&byte.to_le_bytes());
            }
            Opcode::Push2 => {
                let param_str = line_iter.next().unwrap_or_else(|| {
                    panic!("Operation {mnemonic} requires parameter at line no {line_no}")
                });
                let param = param_str.parse::<u64>().expect("Could not parse number");
                if param >= 2u64.pow(16) {
                    todo!()
                }
                bytecode.push(Opcode::Push2.into());

                let byte = param as u16;
                bytecode.extend_from_slice(&byte.to_le_bytes());
            }
            Opcode::Push3 | Opcode::Push4 | Opcode::Push5 | Opcode::Push6 | Opcode::Push7 => {
                todo!()
            }
            _ => bytecode.push(opcode.into()),
        }

        if let Some(v) = line_iter.next() {
            if line_iter.next().is_some() {
                panic!("Found extra junk {v} at line {line_no} after instruction")
            }
        }
    }

    bytecode
}
#[cfg(test)]
mod test {
    use super::assemble_string_to_bytes;
    const INPUT: &str = r#"PUSH8 123
POP"#;
    #[test]
    fn test() {
        let out = assemble_string_to_bytes(INPUT);
        let expected = [40, 123, 0, 0, 0, 0, 0, 0, 0, 1];
        assert_eq!(out, expected);
    }
}
