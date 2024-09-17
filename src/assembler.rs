use crate::opcode::Opcode;

// Temporary assembler to easily create bytecode
pub fn assemble_string_to_bytes(input: &str) -> Vec<u8> {
    let mut bytecode = Vec::new();
    for (line_no, line) in input.lines().enumerate() {
        let mut line_iter = line.split_whitespace();

        let mnemonic = line_iter
            .next()
            .unwrap_or_else(|| panic!("Empty line at line no {line_no}"));
        let opcode: Opcode = mnemonic.try_into().unwrap();
        let byte: u8 = opcode.into();
        bytecode.push(byte);
        if opcode.takes_parameter() {
            let param_str = line_iter.next().unwrap_or_else(|| {
                panic!("Operation {mnemonic} requires parameter at line no {line_no}")
            });
            let param: u64 = param_str.parse().unwrap_or_else(|e| {
                panic!("Could not parse parameter to u64 at line no {line_no}. Error: {e}")
            });
            bytecode.extend_from_slice(&param.to_be_bytes());
        }
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
