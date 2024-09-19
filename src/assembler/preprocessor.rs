use std::{collections::HashMap, fmt::Display, ops::Sub};

use crate::opcode::Opcode;

/// Container type representing a series of bytes with length between 1 and 8
#[derive(Debug, Clone)]
pub enum VarlenBytes {
    B1(u8),
    B2(u16),
    B3(u32),
    B4(u32),
    B5(u64),
    B6(u64),
    B7(u64),
    B8(u64),
}

impl Display for VarlenBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VarlenBytes::B1(b) => write!(f, "{b}"),
            VarlenBytes::B2(b) => write!(f, "{b}"),
            VarlenBytes::B3(b) => write!(f, "{b}"),
            VarlenBytes::B4(b) => write!(f, "{b}"),
            VarlenBytes::B5(b) => write!(f, "{b}"),
            VarlenBytes::B6(b) => write!(f, "{b}"),
            VarlenBytes::B7(b) => write!(f, "{b}"),
            VarlenBytes::B8(b) => write!(f, "{b}"),
        }
    }
}

impl VarlenBytes {
    fn compile(self) -> String {
        let mut out = String::new();
        match self {
            VarlenBytes::B1(_) => out.push_str(&Opcode::Push1.to_string()),
            VarlenBytes::B2(_) => out.push_str(&Opcode::Push2.to_string()),
            VarlenBytes::B3(_) => out.push_str(&Opcode::Push3.to_string()),
            VarlenBytes::B4(_) => out.push_str(&Opcode::Push4.to_string()),
            VarlenBytes::B5(_) => out.push_str(&Opcode::Push5.to_string()),
            VarlenBytes::B6(_) => out.push_str(&Opcode::Push6.to_string()),
            VarlenBytes::B7(_) => out.push_str(&Opcode::Push7.to_string()),
            VarlenBytes::B8(_) => out.push_str(&Opcode::Push8.to_string()),
        };
        out.push(' ');
        out.push_str(&self.to_string());
        out
    }
    fn reduce(&self) -> VarlenBytes {
        let num: u64 = self.clone().into();
        if num < 2u64.pow(8) {
            VarlenBytes::B1(num as u8)
        } else if num < 2u64.pow(8 * 2) {
            VarlenBytes::B2(num as u16)
        } else if num < 2u64.pow(8 * 3) {
            VarlenBytes::B3(num as u32)
        } else if num < 2u64.pow(8 * 4) {
            VarlenBytes::B4(num as u32)
        } else if num < 2u64.pow(8 * 5) {
            VarlenBytes::B5(num)
        } else if num < 2u64.pow(8 * 6) {
            VarlenBytes::B6(num)
        } else if num < 2u64.pow(8 * 7) {
            VarlenBytes::B7(num)
        } else {
            VarlenBytes::B8(num)
        }
    }

    fn byte_count(&self) -> usize {
        match self {
            VarlenBytes::B1(_) => 1,
            VarlenBytes::B2(_) => 2,
            VarlenBytes::B3(_) => 3,
            VarlenBytes::B4(_) => 4,
            VarlenBytes::B5(_) => 5,
            VarlenBytes::B6(_) => 6,
            VarlenBytes::B7(_) => 7,
            VarlenBytes::B8(_) => 8,
        }
    }
}

impl Sub<u64> for VarlenBytes {
    type Output = VarlenBytes;

    fn sub(self, rhs: u64) -> Self::Output {
        match self {
            VarlenBytes::B1(v) => VarlenBytes::B1(v - rhs as u8),
            VarlenBytes::B2(v) => VarlenBytes::B2(v - rhs as u16),
            VarlenBytes::B3(v) => VarlenBytes::B3(v - rhs as u32),
            VarlenBytes::B4(v) => VarlenBytes::B4(v - rhs as u32),
            VarlenBytes::B5(v) => VarlenBytes::B5(v - rhs),
            VarlenBytes::B6(v) => VarlenBytes::B6(v - rhs),
            VarlenBytes::B7(v) => VarlenBytes::B7(v - rhs),
            VarlenBytes::B8(v) => VarlenBytes::B8(v - rhs),
        }
    }
}

impl From<VarlenBytes> for u64 {
    fn from(val: VarlenBytes) -> Self {
        match val {
            VarlenBytes::B1(v) => v as u64,
            VarlenBytes::B2(v) => v as u64,
            VarlenBytes::B3(v) => v as u64,
            VarlenBytes::B4(v) => v as u64,
            VarlenBytes::B5(v) => v,
            VarlenBytes::B6(v) => v,
            VarlenBytes::B7(v) => v,
            VarlenBytes::B8(v) => v,
        }
    }
}

impl From<u64> for VarlenBytes {
    fn from(value: u64) -> Self {
        VarlenBytes::B8(value)
    }
}

#[derive(Debug)]
pub enum Alias {
    Increment,
    Decrement,
}

impl Alias {
    fn from_str(input: &str) -> Option<Alias> {
        Some(match input {
            "INC" => Alias::Increment,
            "DEC" => Alias::Decrement,
            _ => return None,
        })
    }
    fn compile(self) -> Vec<Stage2> {
        let mut statements = Vec::new();
        match self {
            Alias::Increment => {
                statements.push(Stage2::Push(VarlenBytes::B1(1)));
                statements.push(Stage2::Opcode(Opcode::Add));
            }
            Alias::Decrement => {
                statements.push(Stage2::Push(VarlenBytes::B1(1)));
                statements.push(Stage2::Opcode(Opcode::Swap));
                statements.push(Stage2::Opcode(Opcode::Sub));
            }
        }
        statements
    }
}

#[derive(Debug)]
pub enum Stage1 {
    /// Empty line, or comment
    Empty,
    Opcode(Opcode),
    Print(String),
    Alias(Alias),
    Push(u64),
    GotoLabel(String),
    UnresolvedGoto(String),
    UnresolvedConditionalGoto(String),
}

#[derive(Debug)]
pub enum Stage2 {
    Opcode(Opcode),
    Push(VarlenBytes),
    GotoLabel(String),
    UnresolvedGoto(String),
    UnresolvedConditionalGoto(String),
}
#[derive(Debug)]
pub enum Stage3 {
    Opcode(Opcode),
    Push(VarlenBytes),
    ResolvedGoto(VarlenBytes),
    ResolvedConditionalGoto(VarlenBytes),
}

impl Stage2 {
    fn byte_count(&self) -> usize {
        match self {
            Stage2::Opcode(_) => 1,
            Stage2::Push(b) => b.byte_count() + 1,
            Stage2::UnresolvedGoto(_) => 10, // goto = 1, push8 = 1, push8 bytes = 8
            Stage2::UnresolvedConditionalGoto(_) => 10, // goto = 1, push8 = 1, push8 bytes = 8
            Stage2::GotoLabel(_) => 1,
        }
    }
}

impl Stage3 {
    fn compile(self) -> String {
        let mut out = String::new();
        match self {
            Stage3::Opcode(opcode) => out.push_str(&opcode.to_string()),
            Stage3::Push(bytes) => {
                out.push_str(&bytes.compile());
            }
            Stage3::ResolvedGoto(bytes) => {
                out.push_str(&bytes.compile());
                out.push('\n');
                out.push_str(&Opcode::Goto.to_string());
            }
            Stage3::ResolvedConditionalGoto(bytes) => {
                out.push_str(&bytes.compile());
                out.push('\n');
                out.push_str(&Opcode::GotoNz.to_string());
            }
        }
        out
    }
}
#[derive(Debug)]
pub enum PreprocessorError {
    UsedNumberedPushOpcode,
    NonParsableParameter(String),
    InvalidGoto(String),
    TextAfterStatement(String),
    UnknownOpcode(String),
    NoParameter,
}

pub fn compile_statements(statements: Vec<Stage3>) -> Result<String, PreprocessorError> {
    let mut out = String::new();
    for statement in statements {
        out.push_str(&statement.compile());
        out.push('\n');
    }
    Ok(out)
}

fn parse_line(line: &str) -> Result<Stage1, PreprocessorError> {
    let line = line.trim();
    if line.is_empty() {
        return Ok(Stage1::Empty);
    }
    if line.starts_with("//") {
        return Ok(Stage1::Empty);
    }

    let mut line_iter = line.split_whitespace();
    let first = line_iter.next().ok_or(PreprocessorError::NoParameter)?;
    let parsed = if line.starts_with(':') {
        Stage1::GotoLabel(first.to_string())
    } else if let Some(a) = Alias::from_str(first) {
        Stage1::Alias(a)
    } else if first == "PUSH" {
        let Some(parameter_str) = line_iter.next() else {
            return Err(PreprocessorError::NoParameter);
        };
        if parameter_str.starts_with(':') {
            // Future feature. allow pushing goto labels directly. This would allow things like callstacks and function returns.
            todo!()
        }
        let Ok(v) = parameter_str.parse::<u64>() else {
            return Err(PreprocessorError::NonParsableParameter(
                parameter_str.to_string(),
            ));
        };
        Stage1::Push(v)
    } else {
        let Ok(op) = first.try_into() else {
            return Err(PreprocessorError::UnknownOpcode(first.to_string()));
        };
        match op {
            Opcode::Goto => {
                if let Some(label) = line_iter.next() {
                    if !label.starts_with(':') {
                        return Err(PreprocessorError::InvalidGoto(label.to_string()));
                    }
                    Stage1::UnresolvedGoto(label.to_string())
                } else {
                    // Raw goto
                    Stage1::Opcode(Opcode::Goto)
                }
            }
            Opcode::GotoNz => {
                if let Some(label) = line_iter.next() {
                    if !label.starts_with(':') {
                        return Err(PreprocessorError::InvalidGoto(label.to_string()));
                    }
                    Stage1::UnresolvedConditionalGoto(label.to_string())
                } else {
                    // Raw goto
                    Stage1::Opcode(Opcode::GotoNz)
                }
            }
            Opcode::Push0
            | Opcode::Push1
            | Opcode::Push2
            | Opcode::Push3
            | Opcode::Push4
            | Opcode::Push5
            | Opcode::Push6
            | Opcode::Push7
            | Opcode::Push8 => return Err(PreprocessorError::UsedNumberedPushOpcode),
            _ => Stage1::Opcode(op),
        }
    };
    if let Some(rest) = line_iter.next() {
        if !rest.starts_with("//") {
            return Err(PreprocessorError::TextAfterStatement(rest.to_string()));
        };
    };
    Ok(parsed)
}

pub fn parse_to_statements(input: &str) -> Result<Vec<Stage1>, PreprocessorError> {
    let mut statements = Vec::new();
    for line in input.lines() {
        let s = parse_line(line)?;
        statements.push(s);
    }
    Ok(statements)
}

pub fn to_stage2(statements: Vec<Stage1>) -> Result<Vec<Stage2>, PreprocessorError> {
    let mut out = Vec::new();

    for statement in statements.into_iter() {
        // Flatten push into specific bit widths
        match statement {
            Stage1::Opcode(opcode) => out.push(Stage2::Opcode(opcode)),
            Stage1::Print(_) => todo!(),
            Stage1::Alias(alias) => out.extend(alias.compile()),
            Stage1::Push(v) => {
                let new = if v == 0 {
                    Stage2::Opcode(Opcode::Push0)
                } else {
                    let b = VarlenBytes::from(v).reduce();
                    Stage2::Push(b)
                };
                out.push(new);
            }
            Stage1::GotoLabel(s) => out.push(Stage2::GotoLabel(s)),
            Stage1::UnresolvedGoto(s) => out.push(Stage2::UnresolvedGoto(s)),
            Stage1::UnresolvedConditionalGoto(s) => out.push(Stage2::UnresolvedConditionalGoto(s)),
            Stage1::Empty => (),
        }
    }

    Ok(out)
}

pub fn to_stage3(input: Vec<Stage2>) -> Result<Vec<Stage3>, PreprocessorError> {
    let mut goto_destinations: HashMap<String, u64> = HashMap::new();
    let mut byte_count = 0;
    for s in input.iter() {
        if let Stage2::GotoLabel(label) = s {
            goto_destinations.insert(label.to_string(), byte_count);
        }
        byte_count += s.byte_count() as u64;
    }
    let mut statements = Vec::new();
    for statement in input {
        let s: Stage3 = match statement {
            Stage2::Opcode(opcode) => Stage3::Opcode(opcode),
            Stage2::Push(b) => Stage3::Push(b),

            Stage2::GotoLabel(_) => Stage3::Opcode(Opcode::GotoTarget),
            Stage2::UnresolvedGoto(label) => {
                let destination = *goto_destinations.get(&label).expect("TODO");
                Stage3::ResolvedGoto(destination.into())
            }
            Stage2::UnresolvedConditionalGoto(label) => {
                let destination = *goto_destinations.get(&label).expect("TODO");
                Stage3::ResolvedConditionalGoto(destination.into())
            }
        };
        statements.push(s);
    }

    Ok(statements)
}
