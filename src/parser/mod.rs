use crate::opcode::Opcode;

#[derive(Debug)]
pub enum ParseError {
    PushParameterReachedEnd,
    NeverHalts,
}

pub fn parse_bytes_to_instructions(bytes: &[u8]) -> Result<Vec<u8>, ParseError> {
    // Ensure input is valid
    let mut iter = bytes.iter().cloned();
    while let Some(next) = iter.next() {
        let opcode: Opcode = Opcode::from_byte(next).expect("Unknown Opcode while parsing");
        match opcode {
            Opcode::Push8 => {
                for _ in 0..8 {
                    iter.next().ok_or(ParseError::PushParameterReachedEnd)?;
                }
            }
            Opcode::Push1 => {
                iter.next().ok_or(ParseError::PushParameterReachedEnd)?;
            }
            Opcode::Push2 => {
                for _ in 0..2 {
                    iter.next().ok_or(ParseError::PushParameterReachedEnd)?;
                }
            }
            Opcode::Push3 | Opcode::Push4 | Opcode::Push5 | Opcode::Push6 | Opcode::Push7 => {
                todo!()
            }
            _ => (),
        }
    }

    if let Some(v) = bytes.last() {
        if *v != Opcode::Halt.into() {
            return Err(ParseError::PushParameterReachedEnd);
        }
    } else {
        return Err(ParseError::PushParameterReachedEnd);
    }

    Ok(bytes.to_owned())
}
