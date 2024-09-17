use crate::opcode::Opcode;

pub fn parse_bytes_to_instructions(bytes: &[u8]) -> Vec<u8> {
    let mut iter = bytes.iter().cloned();
    // Validate input is proper.
    while let Some(next) = iter.next() {
        let opcode: Opcode = next.try_into().expect("Unknown Opcode");
        if opcode.takes_parameter() {
            let mut buffer = [0, 0, 0, 0, 0, 0, 0, 0];
            for item in buffer.iter_mut() {
                *item = iter
                    .next()
                    .expect("Ran out of bytes while reading parameter.");
            }
        };
    }

    bytes.to_owned()
}
