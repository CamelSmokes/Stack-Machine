use crate::opcode::Opcode;

pub fn parse_bytes_to_instructions(bytes: &[u8]) -> Vec<u8> {
    let mut iter = bytes.iter().cloned();
    // Validate input is proper.
    while let Some(next) = iter.next() {
        let opcode: Opcode = Opcode::from_byte(next).expect("Unknown Opcode");
        if opcode == Opcode::Push {
            iter.next().unwrap();
            iter.next().unwrap();
            iter.next().unwrap();
            iter.next().unwrap();
            iter.next().unwrap();
            iter.next().unwrap();
            iter.next().unwrap();
            iter.next().unwrap();
        };
        if opcode == Opcode::Push1 {
            iter.next().unwrap();
        }
    }

    bytes.to_owned()
}
