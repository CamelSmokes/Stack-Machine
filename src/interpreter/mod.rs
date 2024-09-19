mod instruction;
use std::io::Read;

use instruction::{get_instruction_table, InstructionTable};

const STACK_SIZE: usize = 64;
const TMP_MEMORY_SIZE: usize = 8192;

#[derive(Debug)]
pub enum InterpreterEvent {
    /// No event, execution is continuing successfully
    Nothing,
    /// Program has reached end without exception
    ProgramEnd,
    Silent(bool),
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
    InvalidInstruction,
}

#[derive(Debug)]
pub struct Interpreter {
    program: Vec<u8>,
    stack: [u64; STACK_SIZE],
    /// Temporary memory storage
    memory: [u64; TMP_MEMORY_SIZE],
    stack_length: usize,
    program_counter: usize,
    instruction_table: InstructionTable,
}

impl Interpreter {
    pub fn new(program: Vec<u8>) -> Interpreter {
        Interpreter {
            program,
            stack: [0; STACK_SIZE],
            memory: [0; TMP_MEMORY_SIZE],
            stack_length: 0,
            program_counter: 0,
            instruction_table: get_instruction_table(),
        }
    }
    pub fn debug_get_stack(&self) -> &[u64] {
        &self.stack[0..self.stack_length]
    }
    pub fn debug_get_memory(&self) -> &[u64] {
        &self.memory
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
            self.stack_length -= 1;

            Ok(val)
        } else {
            Err(InterpreterError::StackUnderflow)
        }
    }
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
    fn pop_two(&mut self) -> Result<(u64, u64), InterpreterError> {
        if self.stack_length > 1 {
            assert!(self.stack_length - 1 < STACK_SIZE);
            assert!(self.stack_length - 2 < STACK_SIZE);
            let a = self.stack.get(self.stack_length - 1).unwrap();
            let b = self.stack.get(self.stack_length - 2).unwrap();
            self.stack_length -= 2;
            Ok((*a, *b))
        } else {
            Err(InterpreterError::StackUnderflow)
        }
    }
    /// Push with confidence that the stack will not overflow
    #[inline(never)]
    fn confident_push(&mut self, value: u64) {
        assert!(self.stack_length < STACK_SIZE);
        self.stack[self.stack_length] = value;
        self.stack_length += 1;
    }
    /// pop with confidence that the stack will not underflow
    #[inline]
    fn confident_pop(&mut self) -> u64 {
        assert!(self.stack_length > 0);
        let val = self.stack[self.stack_length - 1];
        // self.stack[self.stack_length - 1] = 0;
        self.stack_length -= 1;
        val
    }
    fn get_nth_from_top(&self, nth_stack: u64) -> Result<u64, InterpreterError> {
        let nth_stack = nth_stack as usize;
        if nth_stack >= self.stack_length {
            return Err(InterpreterError::StackUnderflow);
        }
        let index = self.stack_length - nth_stack - 1;
        Ok(self.stack[index])
    }
    fn set_nth_from_top(&mut self, nth_stack: u64, value: u64) -> Result<(), InterpreterError> {
        let nth_stack = nth_stack as usize;
        if nth_stack >= self.stack_length {
            return Err(InterpreterError::StackUnderflow);
        }
        let index = self.stack_length - nth_stack - 1;

        self.stack[index] = value;
        Ok(())
    }

    fn set_nth_from_top_unchecked(&mut self, nth_stack: u64, value: u64) {
        let nth_stack = nth_stack as usize;
        let index = self.stack_length - nth_stack - 1;
        assert!(index < self.stack_length);
        assert!(index < self.stack.len());
        *unsafe { self.stack.get_unchecked_mut(index) } = value;
        // self.stack[index] = value;
    }
    #[inline]
    fn swap_nth(&mut self, nth_stack: u64) -> Result<(), InterpreterError> {
        let a = self.get_nth_from_top(nth_stack)?;
        let b = self.get_nth_from_top(0)?;
        self.set_nth_from_top_unchecked(nth_stack, b);
        self.set_nth_from_top_unchecked(0, a);
        Ok(())
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
        (&self.program[self.program_counter..])
            .read_exact(&mut buffer)
            .unwrap();
        self.program_counter += 8;
        let v = u64::from_le_bytes(buffer);
        Ok(v)
    }
    fn read_parameter_byte(&mut self) -> Result<u64, InterpreterError> {
        let byte = self.next_byte().unwrap();
        let val = u8::from_le_bytes([byte]);
        Ok(val as u64)
    }

    pub fn next_instruction(&mut self) -> Result<InterpreterEvent, InterpreterError> {
        if let Some(instr) = self.next_byte() {
            self.instruction_table[instr as usize](self)
        } else {
            Ok(InterpreterEvent::ProgramEnd)
        }
    }
}
