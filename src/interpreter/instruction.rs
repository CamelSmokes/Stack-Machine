use super::{Interpreter, InterpreterError, InterpreterEvent};

pub type Instruction = fn(&mut Interpreter) -> Result<InterpreterEvent, InterpreterError>;

pub type InstructionTable = [Instruction; 256];
pub fn get_instruction_table() -> InstructionTable {
    let mut table: [Instruction; 256] = [noop as Instruction; 256];
    table[1] = pop;
    table[2] = add;
    table[3] = sub;
    table[4] = mul;
    table[5] = div;
    table[6] = goto;
    table[7] = halt;
    table[8] = dup;
    table[9] = swap;
    table[10] = rem;
    table[11] = mem_load;
    table[12] = mem_store;
    table[13] = goto_nz;
    table[14] = eq;
    table[15] = lt;
    table[16] = gt;
    table[17] = swap2;
    table[18] = dup2;
    table[19] = not;
    table[20] = dup3;
    table[21] = dup4;
    table[22] = noop;
    table[32] = push0;
    table[33] = push1;
    table[34] = push2;
    table[40] = push8;

    table[128] = noop;

    table[253] = debug_silent;
    table[254] = debug_char;
    table[255] = debug;

    table
}

fn debug_silent(_: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    Ok(InterpreterEvent::Silent(true))
}

fn debug(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let v = i.pop()?;
    println!("{v}");
    i.push(v)?;
    Ok(InterpreterEvent::Nothing)
}

fn debug_char(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let v = i.pop()?;
    let v: char = char::from_u32((v as u8) as u32).unwrap();
    print!("{}", v);
    Ok(InterpreterEvent::Nothing)
}

fn push0(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    i.push(0)?;
    Ok(InterpreterEvent::Nothing)
}

fn push1(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let v = i.read_parameter_byte()?;
    i.push(v)?;
    Ok(InterpreterEvent::Nothing)
}
fn push2(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let v = i.read_parameter_2byte()?;
    i.push(v)?;
    Ok(InterpreterEvent::Nothing)
}

pub fn push8(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let v = i.read_parameter()?;
    i.push(v)?;
    Ok(InterpreterEvent::Nothing)
}

pub fn noop(_: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    Ok(InterpreterEvent::Nothing)
}

pub fn goto(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let addr = i.pop()?;
    i.goto(addr)?;
    Ok(InterpreterEvent::Nothing)
}

pub fn pop(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    i.pop()?;
    Ok(InterpreterEvent::Nothing)
}

pub fn add(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let (a, b) = i.pop_two()?;

    let c = a.wrapping_add(b);
    i.confident_push(c);
    Ok(InterpreterEvent::Nothing)
}

pub fn sub(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let (a, b) = i.pop_two()?;

    let c = a.wrapping_sub(b);
    i.confident_push(c);
    Ok(InterpreterEvent::Nothing)
}

pub fn mul(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let (a, b) = i.pop_two()?;

    let c = a.wrapping_mul(b);
    i.confident_push(c);
    Ok(InterpreterEvent::Nothing)
}

pub fn div(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let (a, b) = i.pop_two()?;

    if b == 0 {
        return Err(InterpreterError::DivideByZero);
    }
    let c = a.wrapping_div(b);
    i.confident_push(c);
    Ok(InterpreterEvent::Nothing)
}

pub fn halt(_: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    Ok(InterpreterEvent::ProgramEnd)
}

pub fn swap(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    i.swap_nth(1)?;
    Ok(InterpreterEvent::Nothing)
}

pub fn swap2(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    i.swap_nth(2)?;
    Ok(InterpreterEvent::Nothing)
}

pub fn rem(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let (a, b) = i.pop_two()?;

    if b == 0 {
        return Err(InterpreterError::DivideByZero);
    }
    let c = a % b;
    i.confident_push(c);
    Ok(InterpreterEvent::Nothing)
}

pub fn mem_load(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let index = i.pop()?;
    let val = i.load_memory_offset(index)?;
    i.confident_push(val);

    Ok(InterpreterEvent::Nothing)
}

pub fn mem_store(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let index = i.pop()?;
    let val = i.pop()?;
    i.set_memory_offset(index, val)?;
    Ok(InterpreterEvent::Nothing)
}

pub fn goto_nz(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let addr = i.pop()?;
    let conditional = i.pop()?;
    if conditional != 0 {
        i.goto(addr)?;
    }
    Ok(InterpreterEvent::Nothing)
}

pub fn eq(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let (a, b) = i.pop_two()?;

    let c = (a == b) as u64;
    i.confident_push(c);
    Ok(InterpreterEvent::Nothing)
}

pub fn lt(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let (a, b) = i.pop_two()?;

    let c = (a < b) as u64;
    i.confident_push(c);
    Ok(InterpreterEvent::Nothing)
}

pub fn gt(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let (a, b) = i.pop_two()?;
    let c = (a > b) as u64;
    i.confident_push(c);
    Ok(InterpreterEvent::Nothing)
}

pub fn dup(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let c = i.get_nth_from_top(0)?;
    i.push(c)?;
    Ok(InterpreterEvent::Nothing)
}

pub fn dup2(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let c = i.get_nth_from_top(1)?;
    i.push(c)?;
    Ok(InterpreterEvent::Nothing)
}

pub fn dup3(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let c = i.get_nth_from_top(2)?;
    i.push(c)?;
    Ok(InterpreterEvent::Nothing)
}

pub fn dup4(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let c = i.get_nth_from_top(3)?;
    i.push(c)?;
    Ok(InterpreterEvent::Nothing)
}

pub fn not(i: &mut Interpreter) -> Result<InterpreterEvent, InterpreterError> {
    let a = i.pop()?;
    i.confident_push(!a);
    Ok(InterpreterEvent::Nothing)
}
