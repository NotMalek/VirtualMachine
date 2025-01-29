use crate::core::instruction::Instruction;
use crate::core::error::VMError;
use crate::core::state::{VMState, DebugOptions};
use crate::core::heap::HeapValue;
use std::collections::HashMap;

pub struct VM {
    state: VMState,
    debug_options: DebugOptions,
    output_buffer: Vec<String>,
}

impl VM {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        VM {
            state: VMState::new(instructions),
            debug_options: DebugOptions::default(),
            output_buffer: Vec::new(),
        }
    }

    pub fn set_debug_options(&mut self, options: DebugOptions) {
        self.debug_options = options;
    }

    pub fn step(&mut self) -> Result<bool, VMError> {
        if self.state.program_counter >= self.state.instructions().len() {
            return Ok(false);
        }

        let instruction = self.state.instructions()[self.state.program_counter].clone();

        if self.debug_options.show_instructions {
            println!("Executing instruction at PC {}: {:?}", self.state.program_counter, instruction);
            println!("Current stack: {:?}", self.state.stack);
        }

        // Execute the instruction
        if let Err(e) = self.execute_instruction(instruction.clone()) {
            if self.debug_options.show_instructions {
                println!("Error executing instruction: {:?}", e);
            }
            return Err(e);
        }

        self.state.program_counter += 1;

        // Check for halt after executing the instruction
        if matches!(instruction, Instruction::Halt) {
            return Ok(false);
        }

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
            Instruction::Add => {
                let b = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                let a = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                self.state.stack.push(a + b);
                Ok(())
            }
            Instruction::Sub => {
                let b = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                let a = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                self.state.stack.push(a - b);
                Ok(())
            }
            Instruction::Mul => {
                let b = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                let a = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                self.state.stack.push(a * b);
                Ok(())
            }
            Instruction::Div => {
                let b = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                let a = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                if b == 0 {
                    return Err(VMError::DivisionByZero);
                }
                self.state.stack.push(a / b);
                Ok(())
            }
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
                if target >= self.state.instructions().len() {
                    return Err(VMError::InvalidInstruction(target));
                }
                self.state.program_counter = target - 1; // -1 because step() will increment
                Ok(())
            }
            Instruction::JumpIfZero(target) => {
                let condition = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                if condition == 0 {
                    if target >= self.state.instructions().len() {
                        return Err(VMError::InvalidInstruction(target));
                    }
                    self.state.program_counter = target - 1;
                }
                Ok(())
            }
            Instruction::JumpIfNotZero(target) => {
                let condition = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                if condition != 0 {
                    if target >= self.state.instructions().len() {
                        return Err(VMError::InvalidInstruction(target));
                    }
                    self.state.program_counter = target - 1;
                }
                Ok(())
            }
            Instruction::JumpIf(target) => {
                let condition = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                if condition != 0 {
                    if target >= self.state.instructions().len() {
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
            Instruction::Print => {
                let value = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                self.push_output(format!("{}\n", value));
                Ok(())
            }
            Instruction::PrintStr(s) => {
                let unescaped = s.replace("\\n", "\n");
                self.push_output(unescaped);
                Ok(())
            }
            Instruction::PrintChar => {
                let value = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                self.push_output((value as u8 as char).to_string());
                Ok(())
            }
            Instruction::NewArray => {
                let size = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                if size < 0 {
                    return Err(VMError::InvalidArrayIndex(size));
                }
                let array = vec![0; size as usize];
                let array_id = self.state.heap.allocate(HeapValue::Array(array));
                self.state.stack.push(array_id as i64);
                Ok(())
            }
            Instruction::ArrayGet => {
                let index = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                let array_id = self.state.stack.pop().ok_or(VMError::StackUnderflow)? as usize;

                if let Some(HeapValue::Array(array)) = self.state.heap.get(array_id) {
                    if index < 0 || index as usize >= array.len() {
                        return Err(VMError::ArrayBoundsError(index, array.len()));
                    }
                    self.state.stack.push(array[index as usize]);
                    Ok(())
                } else {
                    Err(VMError::InvalidHeapAddress(array_id))
                }
            }
            Instruction::ArraySet => {
                let value = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                let index = self.state.stack.pop().ok_or(VMError::StackUnderflow)?;
                let array_id = self.state.stack.pop().ok_or(VMError::StackUnderflow)? as usize;

                if let Some(HeapValue::Array(array)) = self.state.heap.get_mut(array_id) {
                    if index < 0 || index as usize >= array.len() {
                        return Err(VMError::ArrayBoundsError(index, array.len()));
                    }
                    array[index as usize] = value;
                    Ok(())
                } else {
                    Err(VMError::InvalidHeapAddress(array_id))
                }
            }
            Instruction::ArrayLength => {
                let array_id = self.state.stack.pop().ok_or(VMError::StackUnderflow)? as usize;

                if let Some(HeapValue::Array(array)) = self.state.heap.get(array_id) {
                    self.state.stack.push(array.len() as i64);
                    Ok(())
                } else {
                    Err(VMError::InvalidHeapAddress(array_id))
                }
            }
            Instruction::FreeArray => {
                let array_id = self.state.stack.pop().ok_or(VMError::StackUnderflow)? as usize;

                if let Some(HeapValue::Array(_)) = self.state.heap.free(array_id) {
                    Ok(())
                } else {
                    Err(VMError::InvalidHeapAddress(array_id))
                }
            }
            Instruction::NewString(s) => {
                let string_id = self.state.heap.allocate(HeapValue::String(s));
                self.state.stack.push(string_id as i64);
                Ok(())
            }
            Instruction::StringConcat => {
                let str2_id = self.state.stack.pop().ok_or(VMError::StackUnderflow)? as usize;
                let str1_id = self.state.stack.pop().ok_or(VMError::StackUnderflow)? as usize;

                let s1 = if let Some(HeapValue::String(s1)) = self.state.heap.get(str1_id) {
                    s1.clone()
                } else {
                    return Err(VMError::TypeError("string".into(), "non-string".into()));
                };

                let s2 = if let Some(HeapValue::String(s2)) = self.state.heap.get(str2_id) {
                    s2.clone()
                } else {
                    return Err(VMError::TypeError("string".into(), "non-string".into()));
                };

                let result = format!("{}{}", s1, s2);
                let result_id = self.state.heap.allocate(HeapValue::String(result));
                self.state.stack.push(result_id as i64);
                Ok(())
            }
            Instruction::StringLength => {
                let string_id = self.state.stack.pop().ok_or(VMError::StackUnderflow)? as usize;

                if let Some(HeapValue::String(s)) = self.state.heap.get(string_id) {
                    self.state.stack.push(s.len() as i64);
                    Ok(())
                } else {
                    Err(VMError::TypeError("string".into(), "non-string".into()))
                }
            }
            Instruction::FreeString => {
                let string_id = self.state.stack.pop().ok_or(VMError::StackUnderflow)? as usize;

                if let Some(HeapValue::String(_)) = self.state.heap.free(string_id) {
                    Ok(())
                } else {
                    Err(VMError::InvalidHeapAddress(string_id))
                }
            }
            Instruction::Halt => Ok(()),
            _ => Err(VMError::InvalidInstruction(self.state.program_counter)),
        }
    }

    pub fn get_state(&self) -> &VMState {
        &self.state
    }

    pub fn get_memory(&self) -> &HashMap<String, i64> {
        &self.state.memory
    }

    pub fn current_instruction(&self) -> Option<&Instruction> {
        self.state.instructions().get(self.state.program_counter)
    }

    pub fn call_stack_depth(&self) -> usize {
        self.state.call_stack.len()
    }

    pub fn push_output(&mut self, output: String) {
        if self.debug_options.show_instructions {
            println!("Output: {}", output);
        }
        self.output_buffer.push(output);
    }

    pub fn take_output(&mut self) -> Vec<String> {
        std::mem::take(&mut self.output_buffer)
    }
}