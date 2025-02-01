use std::collections::HashMap;
use crate::core::heap::HeapManager; // Remove `HeapValue` if unused

#[derive(Debug)]
pub struct StackFrame {
    pub return_address: usize,
    pub local_vars: HashMap<String, i64>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub address: usize,
    pub param_count: usize,
    pub local_vars: Vec<String>,
}

#[derive(Debug, Default)]
pub struct DebugOptions {
    pub show_stack: bool,
    pub show_pc: bool,
    pub show_memory: bool,
    pub show_instructions: bool,
}

#[derive(Debug)]
pub struct VMState {
    pub stack: Vec<i64>,
    pub memory: HashMap<String, i64>,
    pub program_counter: usize,
    pub call_stack: Vec<StackFrame>,
    pub functions: HashMap<String, Function>,
    pub heap: HeapManager,
    instructions: Vec<crate::core::instruction::Instruction>, // Private field
}

impl VMState {
    pub fn new(instructions: Vec<crate::core::instruction::Instruction>) -> Self {
        Self {
            stack: Vec::new(),
            memory: HashMap::new(),
            program_counter: 0,
            call_stack: Vec::new(),
            functions: HashMap::new(),
            heap: HeapManager::new(),
            instructions,
        }
    }

    pub fn instructions(&self) -> &Vec<crate::core::instruction::Instruction> {
        &self.instructions
    }
}