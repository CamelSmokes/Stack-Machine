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
    Lt = 15,
    Gt = 16,
    Swap2 = 17,
    Dup2 = 18,
    Not = 19,
    Dup3 = 20,
    Dup4 = 21,

    Push0 = 32,
    Push1 = 33,

    NoOp = 128,

    // Debugging instructions
    DbgSilent = 253,
    /// Print top of stack to debug stdout
    Debug = 255,
    /// Print top of stack as ASCII code to stdout
    DebugChar = 254,
}

impl From<Opcode> for u8 {
    fn from(val: Opcode) -> Self {
        val as u8
    }
}
impl Opcode {
    pub fn from_byte(input: u8) -> Option<Opcode> {
        use Opcode::*;
        Some(match input {
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
            15 => Lt,
            16 => Gt,
            17 => Swap2,
            18 => Dup2,
            19 => Not,
            20 => Dup3,
            21 => Dup4,

            32 => Push0,
            33 => Push1,

            128 => NoOp,
            253 => DbgSilent,
            254 => DebugChar,
            255 => Debug,
            _ => return None,
        })
    }
}

impl TryInto<Opcode> for &str {
    type Error = String;

    fn try_into(self) -> Result<Opcode, Self::Error> {
        use Opcode::*;
        Ok(match self {
            "PUSH" => Push,
            "PUSH0" => Push0,
            "PUSH1" => Push1,
            "POP" => Pop,
            "GOTO" => Goto,
            "ADD" => Add,
            "SUB" => Sub,
            "MUL" => Mul,
            "DIV" => Div,
            "HALT" => Halt,
            "DUP" => Dup,
            "DUP2" => Dup2,
            "DUP3" => Dup3,
            "DUP4" => Dup4,
            "SWAP" => Swap,
            "SWAP2" => Swap2,
            "MOD" => Mod,
            "MLOAD" => MemLoad,
            "MSTORE" => MemStore,
            "GOTONZ" => GotoNz,
            "EQ" => Eq,
            "LT" => Lt,
            "GT" => Gt,
            "NOT" => Not,
            "NOOP" => NoOp,
            "DEBUG" => Debug,
            "DEBUGCHAR" => DebugChar,
            _ => return Err(format!("Unknown Opcode {}", self)),
        })
    }
}
