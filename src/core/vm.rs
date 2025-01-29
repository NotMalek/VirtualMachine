use crate::core::error::VMError;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub address: usize,
    pub param_count: usize,
    pub local_vars: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    return_address: usize,
    local_vars: HashMap<String, i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Instruction {
    // Stack Operations
    Push(i64),
    Pop,
    Dup,
    Swap,

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,

    // Memory
    Load(String),
    Store(String),

    // Control Flow
    Jump(usize),
    JumpIf(usize),

    // Comparison operations
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,

    // Boolean operations
    And,
    Or,
    Not,

    // Enhanced control flow
    JumpIfZero(usize),
    JumpIfNotZero(usize),

    // Function instructions
    DefineFunction(String, usize),    // name, parameter count
    BeginFunction,
    EndFunction,
    CreateLocal(String),
    LoadLocal(String),
    StoreLocal(String),
    PushParam(usize),
    Call(String),
    Return,
    Halt,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VMState {
    pub stack: Vec<i64>,
    pub memory: HashMap<String, i64>,
    pub program_counter: usize,
    pub call_stack: Vec<StackFrame>,
    pub functions: HashMap<String, Function>,
    instructions: Vec<Instruction>,
}

impl VMState {
    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }
}

pub struct VM {
    state: VMState,
    debug_mode: bool,
}

impl VM {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        VM {
            state: VMState {
                stack: Vec::new(),
                memory: HashMap::new(),
                program_counter: 0,
                call_stack: Vec::new(),
                functions: HashMap::new(),
                instructions,
            },
            debug_mode: false,
        }
    }

    pub fn step(&mut self) -> Result<bool, VMError> {
        if self.state.program_counter >= self.state.instructions.len() {
            return Ok(false);
        }

        let instruction = self.state.instructions[self.state.program_counter].clone();
        self.execute_instruction(instruction)?;

        if self.debug_mode {
            println!("Stack: {:?}", self.state.stack);
            println!("PC: {}", self.state.program_counter);
        }

        self.state.program_counter += 1;
        Ok(true)
    }

    fn binary_op<F>(&mut self, op: F) -> Result<(), VMError>
    where
        F: FnOnce(i64, i64) -> Result<i64, VMError>,
    {
        let b = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
        let a = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
        let result = op(a, b)?;
        self.state.stack.push(result);
        Ok(())
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> Result<(), VMError> {
        match instruction {
            Instruction::Push(value) => {
                self.state.stack.push(value);
                Ok(())
            }
            Instruction::Pop => {
                self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                Ok(())
            }
            Instruction::Dup => {
                let value = *self.state.stack.last().ok_or(VMError::StackUnderflow)?;
                self.state.stack.push(value);
                Ok(())
            }
            Instruction::Swap => {
                if self.state.stack.len() < 2 {
                    return Err(VMError::StackUnderflow);
                }
                let len = self.state.stack.len();
                self.state.stack.swap(len - 1, len - 2);
                Ok(())
            }
            Instruction::Add => self.binary_op(|a, b| Ok(a + b)),
            Instruction::Sub => self.binary_op(|a, b| Ok(a - b)),
            Instruction::Mul => self.binary_op(|a, b| Ok(a * b)),
            Instruction::Div => self.binary_op(|a, b| {
                if b == 0 {
                    return Err(VMError::DivisionByZero);
                }
                Ok(a / b)
            }),
            Instruction::Store(name) => {
                let value = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                self.state.memory.insert(name, value);
                Ok(())
            }
            Instruction::Load(name) => {
                let value = *self.state.memory.get(&name)
                    .ok_or(VMError::InvalidMemoryAccess(0))?;
                self.state.stack.push(value);
                Ok(())
            }
            Instruction::Jump(target) => {
                if target >= self.state.instructions.len() {
                    return Err(VMError::InvalidInstruction(target));
                }
                self.state.program_counter = target - 1; // -1 because step() will increment
                Ok(())
            }
            Instruction::JumpIf(target) => {
                let condition = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                if condition != 0 {
                    if target >= self.state.instructions.len() {
                        return Err(VMError::InvalidInstruction(target));
                    }
                    self.state.program_counter = target - 1;
                }
                Ok(())
            }
            Instruction::Equal => self.binary_op(|a, b| Ok(if a == b { 1 } else { 0 })),
            Instruction::NotEqual => self.binary_op(|a, b| Ok(if a != b { 1 } else { 0 })),
            Instruction::LessThan => self.binary_op(|a, b| Ok(if a < b { 1 } else { 0 })),
            Instruction::LessEqual => self.binary_op(|a, b| Ok(if a <= b { 1 } else { 0 })),
            Instruction::GreaterThan => self.binary_op(|a, b| Ok(if a > b { 1 } else { 0 })),
            Instruction::GreaterEqual => self.binary_op(|a, b| Ok(if a >= b { 1 } else { 0 })),
            Instruction::And => self.binary_op(|a, b| Ok(if a != 0 && b != 0 { 1 } else { 0 })),
            Instruction::Or => self.binary_op(|a, b| Ok(if a != 0 || b != 0 { 1 } else { 0 })),
            Instruction::Not => {
                let value = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                self.state.stack.push(if value == 0 { 1 } else { 0 });
                Ok(())
            }
            Instruction::JumpIfZero(target) => {
                let condition = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                if condition == 0 {
                    if target >= self.state.instructions.len() {
                        return Err(VMError::InvalidInstruction(target));
                    }
                    self.state.program_counter = target - 1;
                }
                Ok(())
            }
            Instruction::JumpIfNotZero(target) => {
                let condition = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                if condition != 0 {
                    if target >= self.state.instructions.len() {
                        return Err(VMError::InvalidInstruction(target));
                    }
                    self.state.program_counter = target - 1;
                }
                Ok(())
            }
            Instruction::DefineFunction(name, param_count) => {
                let function = Function {
                    name: name.clone(),
                    address: self.state.program_counter + 1,
                    param_count,
                    local_vars: Vec::new(),
                };
                self.state.functions.insert(name, function);

                // Skip to end of function definition
                while let Some(instr) = self.state.instructions.get(self.state.program_counter) {
                    if matches!(instr, Instruction::EndFunction) {
                        break;
                    }
                    self.state.program_counter += 1;
                }
                Ok(())
            }
            Instruction::BeginFunction => Ok(()), // Marker instruction
            Instruction::EndFunction => Ok(()),   // Marker instruction
            Instruction::CreateLocal(name) => {
                if let Some(frame) = self.state.call_stack.last_mut() {
                    let value = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                    frame.local_vars.insert(name, value);
                    Ok(())
                } else {
                    Err(VMError::EmptyCallStack)
                }
            }
            Instruction::LoadLocal(name) => {
                if let Some(frame) = self.state.call_stack.last() {
                    let value = *frame.local_vars.get(&name)
                        .ok_or(VMError::LocalVarNotFound(name))?;
                    self.state.stack.push(value);
                    Ok(())
                } else {
                    Err(VMError::EmptyCallStack)
                }
            }
            Instruction::StoreLocal(name) => {
                if let Some(frame) = self.state.call_stack.last_mut() {
                    let value = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                    frame.local_vars.insert(name, value);
                    Ok(())
                } else {
                    Err(VMError::EmptyCallStack)
                }
            }
            Instruction::PushParam(index) => {
                if let Some(frame) = self.state.call_stack.last() {
                    let stack_pos = self.state.stack.len() - 1 - index;
                    if let Some(&value) = self.state.stack.get(stack_pos) {
                        self.state.stack.push(value);
                        Ok(())
                    } else {
                        Err(VMError::InvalidParameter(index))
                    }
                } else {
                    Err(VMError::EmptyCallStack)
                }
            }
            Instruction::Call(name) => {
                let function = self.state.functions.get(&name)
                    .ok_or(VMError::FunctionNotFound(name.clone()))?;

                let frame = StackFrame {
                    return_address: self.state.program_counter + 1,
                    local_vars: HashMap::new(),
                };

                self.state.call_stack.push(frame);
                self.state.program_counter = function.address;
                Ok(())
            }
            Instruction::Return => {
                if let Some(frame) = self.state.call_stack.pop() {
                    self.state.program_counter = frame.return_address - 1; // -1 because step() will increment
                    Ok(())
                } else {
                    Err(VMError::EmptyCallStack)
                }
            }
            Instruction::Halt => Ok(()),
        }
    }

    pub fn enable_debug(&mut self) {
        self.debug_mode = true;
    }

    pub fn get_state(&self) -> &VMState {
        &self.state
    }

    pub fn get_memory(&self) -> &HashMap<String, i64> {
        &self.state.memory
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_operations() {
        let instructions = vec![
            Instruction::Push(5),
            Instruction::Dup,
            Instruction::Swap,
            Instruction::Pop,
        ];

        let mut vm = VM::new(instructions);
        while vm.step().unwrap() {}

        assert_eq!(vm.get_state().stack, vec![5]);
    }

    #[test]
    fn test_memory_operations() {
        let instructions = vec![
            Instruction::Push(42),
            Instruction::Store("x".to_string()),
            Instruction::Push(10),
            Instruction::Load("x".to_string()),
            Instruction::Add,
        ];

        let mut vm = VM::new(instructions);
        while vm.step().unwrap() {}

        assert_eq!(vm.get_state().stack, vec![52]);
        assert_eq!(vm.get_memory().get("x"), Some(&42));
    }

    #[test]
    fn test_comparison_operations() {
        let instructions = vec![
            Instruction::Push(5),
            Instruction::Push(3),
            Instruction::LessThan,
            Instruction::Push(2),
            Instruction::Push(2),
            Instruction::Equal,
        ];

        let mut vm = VM::new(instructions);
        while vm.step().unwrap() {}

        assert_eq!(vm.get_state().stack, vec![0, 1]);
    }
}