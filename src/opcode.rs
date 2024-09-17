#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Opcode {
    Push = 0,
    Pop = 1,
    Add = 2,
    Sub = 3,
    Mul = 4,
    Div = 5,
    Goto = 6,
    Halt = 7,
    Dup = 8,
    Swap = 9,
    Mod = 10,
    MemLoad = 11,
    MemStore = 12,
    /// Goto if not equal to zero
    GotoNz = 13,
    Eq = 14,
}
impl Opcode {
    pub fn takes_parameter(self) -> bool {
        matches!(self, Opcode::Push)
    }
}

impl From<Opcode> for u8 {
    fn from(val: Opcode) -> Self {
        val as u8
    }
}
impl TryInto<Opcode> for u8 {
    type Error = String;

    fn try_into(self) -> Result<Opcode, Self::Error> {
        use Opcode::*;
        Ok(match self {
            0 => Push,
            1 => Pop,
            2 => Add,
            3 => Sub,
            4 => Mul,
            5 => Div,
            6 => Goto,
            7 => Halt,
            8 => Dup,
            9 => Swap,
            10 => Mod,
            11 => MemLoad,
            12 => MemStore,
            13 => GotoNz,
            14 => Eq,
            _ => return Err(format!("Unknown Opcode {}", self)),
        })
    }
}

impl TryInto<Opcode> for &str {
    type Error = String;

    fn try_into(self) -> Result<Opcode, Self::Error> {
        use Opcode::*;
        Ok(match self {
            "PUSH" => Push,
            "POP" => Pop,
            "GOTO" => Goto,
            "ADD" => Add,
            "SUB" => Sub,
            "MUL" => Mul,
            "DIV" => Div,
            "HALT" => Halt,
            "DUP" => Dup,
            "SWAP" => Swap,
            "MOD" => Mod,
            "MLOAD" => MemLoad,
            "MSTORE" => MemStore,
            "GOTONZ" => GotoNz,
            "EQ" => Eq,
            _ => return Err(format!("Unknown Opcode {}", self)),
        })
    }
}
