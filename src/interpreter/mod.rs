use crate::opcode::Opcode;

const STACK_SIZE: usize = 64;
const TMP_MEMORY_SIZE: usize = 8192;

#[derive(Debug)]
pub enum InterpreterEvent {
    /// No event, execution is continuing successfully
    Nothing,
    /// Program has reached end without exception
    ProgramEnd,
}

#[derive(Debug)]
pub enum InterpreterError {
    /// Invalid Goto location
    InvalidGoto,
    /// Push too many items on stack
    StackOverflow,
    /// Popped too many items from stack
    StackUnderflow,

    DivideByZero,

    InvalidMemoryOffset,
}

#[derive(Debug)]
pub struct Interpreter {
    program: Vec<u8>,
    stack: [u64; STACK_SIZE],
    /// Temporary memory storage
    memory: [u64; TMP_MEMORY_SIZE],
    stack_length: usize,
    program_counter: usize,
}

impl Interpreter {
    pub fn new(program: Vec<u8>) -> Interpreter {
        Interpreter {
            program,
            stack: [0; STACK_SIZE],
            memory: [0; TMP_MEMORY_SIZE],
            stack_length: 0,
            program_counter: 0,
        }
    }
    pub fn debug_get_stack(&self) -> &[u64] {
        &self.stack
    }
    #[inline]
    fn push(&mut self, value: u64) -> Result<(), InterpreterError> {
        if self.stack_length < STACK_SIZE {
            self.stack[self.stack_length] = value;
            self.stack_length += 1;
            Ok(())
        } else {
            Err(InterpreterError::StackOverflow)
        }
    }
    #[inline]
    fn pop(&mut self) -> Result<u64, InterpreterError> {
        if self.stack_length > 0 {
            let val = self.stack[self.stack_length - 1];
            self.stack[self.stack_length - 1] = 0;
            self.stack_length -= 1;

            Ok(val)
        } else {
            Err(InterpreterError::StackUnderflow)
        }
    }
    fn next_byte(&mut self) -> Option<u8> {
        let v = self.program.get(self.program_counter).cloned();
        self.program_counter += 1;
        v
    }

    fn load_memory_offset(&self, offset: u64) -> Result<u64, InterpreterError> {
        self.memory
            .get(offset as usize)
            .copied()
            .ok_or(InterpreterError::InvalidMemoryOffset)
    }
    fn set_memory_offset(&mut self, offset: u64, value: u64) -> Result<(), InterpreterError> {
        let Some(m) = self.memory.get_mut(offset as usize) else {
            return Err(InterpreterError::InvalidMemoryOffset);
        };
        *m = value;
        Ok(())
    }

    /// parses next instruction parameter and advances the program counter past the instruction parameter.
    fn read_parameter(&mut self) -> Result<u64, InterpreterError> {
        let mut buffer: [u8; 8] = [0; 8];
        for b in buffer.iter_mut() {
            *b = self.next_byte().expect("TODO");
        }
        let v = u64::from_be_bytes(buffer);
        Ok(v)
    }

    pub fn next_instruction(&mut self) -> Result<InterpreterEvent, InterpreterError> {
        if let Some(instr) = self.next_byte() {
            let opcode: Opcode = instr.try_into().unwrap();
            match opcode {
                Opcode::Push => {
                    let v = self.read_parameter()?;
                    self.push(v)?
                }
                Opcode::Goto => {
                    let addr = self.pop()?;
                    self.goto(addr)?
                }
                Opcode::Pop => {
                    self.pop()?;
                }
                Opcode::Add => self.add()?,
                Opcode::Sub => self.sub()?,
                Opcode::Mul => self.mul()?,
                Opcode::Div => self.div()?,
                Opcode::Halt => return self.halt(),
                Opcode::Dup => self.dup()?,
                Opcode::Swap => self.swap()?,
                Opcode::Mod => self.rem()?,
                Opcode::MemLoad => self.mem_load()?,
                Opcode::MemStore => self.mem_store()?,
                Opcode::GotoNz => self.goto_nz()?,
                Opcode::Eq => self.eq()?,
                Opcode::Lt => self.lt()?,
                Opcode::Gt => self.gt()?,
                Opcode::Debug => {
                    let v = self.pop()?;
                    println!("{v}");
                    self.push(v)?;
                }
                Opcode::DebugChar => {
                    let v = self.pop()?;
                    let v: char = char::from_u32((v as u8) as u32).unwrap();
                    print!("{}", v);
                }
            }
        } else {
            return Ok(InterpreterEvent::ProgramEnd);
        }
        Ok(InterpreterEvent::Nothing)
    }
}

// Opcode implementations
impl Interpreter {
    fn goto(&mut self, addr: u64) -> Result<(), InterpreterError> {
        let addr = addr as usize;
        if addr < self.program.len() {
            self.program_counter = addr;
            Ok(())
        } else {
            Err(InterpreterError::InvalidGoto)
        }
    }
    #[inline]
    fn add(&mut self) -> Result<(), InterpreterError> {
        let a = self.pop()?;
        let b = self.pop()?;
        let c = a.wrapping_add(b);
        self.push(c)?;
        Ok(())
    }
    #[inline]
    fn sub(&mut self) -> Result<(), InterpreterError> {
        let a = self.pop()?;
        let b = self.pop()?;
        let c = a.wrapping_sub(b);
        self.push(c)?;
        Ok(())
    }
    #[inline]
    fn mul(&mut self) -> Result<(), InterpreterError> {
        let a = self.pop()?;
        let b = self.pop()?;
        let c = a.wrapping_mul(b);
        self.push(c)?;
        Ok(())
    }
    #[inline]
    fn div(&mut self) -> Result<(), InterpreterError> {
        let a = self.pop()?;
        let b = self.pop()?;
        if b == 0 {
            return Err(InterpreterError::DivideByZero);
        }
        let c = a.wrapping_div(b);
        self.push(c)?;
        Ok(())
    }
    #[inline]
    fn halt(&self) -> Result<InterpreterEvent, InterpreterError> {
        Ok(InterpreterEvent::ProgramEnd)
    }
    #[inline]
    fn dup(&mut self) -> Result<(), InterpreterError> {
        let v = self.pop()?;
        self.push(v)?;
        self.push(v)?;
        Ok(())
    }
    #[inline]
    fn swap(&mut self) -> Result<(), InterpreterError> {
        let a = self.pop()?;
        let b = self.pop()?;
        self.push(a)?;
        self.push(b)?;
        Ok(())
    }
    #[inline]
    fn rem(&mut self) -> Result<(), InterpreterError> {
        let a = self.pop()?;
        let b = self.pop()?;
        if b == 0 {
            return Err(InterpreterError::DivideByZero);
        }
        let c = a % b;
        self.push(c)?;
        Ok(())
    }
    #[inline]
    fn mem_load(&mut self) -> Result<(), InterpreterError> {
        let index = self.pop()?;
        let val = self.load_memory_offset(index)?;
        self.push(val)?;
        Ok(())
    }
    #[inline]
    fn mem_store(&mut self) -> Result<(), InterpreterError> {
        let index = self.pop()?;
        let val = self.pop()?;
        self.set_memory_offset(index, val)?;
        Ok(())
    }

    #[inline]
    fn goto_nz(&mut self) -> Result<(), InterpreterError> {
        let addr = self.pop()?;
        let conditional = self.pop()?;
        if conditional != 0 {
            self.goto(addr)?;
        }
        Ok(())
    }
    #[inline]
    fn eq(&mut self) -> Result<(), InterpreterError> {
        let a = self.pop()?;
        let b = self.pop()?;
        let c = if a == b { 1 } else { 0 };
        self.push(c)?;
        Ok(())
    }
    #[inline]
    fn lt(&mut self) -> Result<(), InterpreterError> {
        let a = self.pop()?;
        let b = self.pop()?;
        let c = if a < b { 1 } else { 0 };
        self.push(c)?;
        Ok(())
    }
    #[inline]
    fn gt(&mut self) -> Result<(), InterpreterError> {
        let a = self.pop()?;
        let b = self.pop()?;
        let c = if a > b { 1 } else { 0 };
        self.push(c)?;
        Ok(())
    }
}
