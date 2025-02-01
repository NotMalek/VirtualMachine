use crate::core::vm::VM;
use crate::core::assembler::Assembler;
use crate::core::state::DebugOptions;
use crate::core::error::VMError;
use std::collections::HashMap;

pub struct VMTester {
    vm: VM,
    step_count: usize,
    max_steps: usize,
    all_output: Vec<String>,
}

impl VMTester {
    pub fn new(source: &str, debug: bool) -> Result<Self, String> {
        let mut assembler = Assembler::new();
        let program = assembler.assemble(source)
            .map_err(|e| format!("Assembly error: {}", e))?;

        let mut vm = VM::new(program);

        if debug {
            vm.set_debug_options(DebugOptions {
                show_instructions: true,
                show_stack: true,
                show_pc: true,
                show_memory: true,
            });
        }

        Ok(VMTester {
            vm,
            step_count: 0,
            max_steps: 1000,
            all_output: Vec::new(),
        })
    }

    pub fn run(&mut self) -> Result<(), VMError> {
        while self.step_count < self.max_steps {
            match self.vm.step() {
                Ok(true) => {
                    let output = self.vm.take_output();
                    self.all_output.extend(output);
                    self.step_count += 1;
                },
                Ok(false) => return Ok(()),
                Err(e) => return Err(e),
            }
        }
        Err(VMError::InvalidInstruction(self.step_count))
    }

    pub fn get_output(&self) -> String {
        self.all_output.join("")
    }

    pub fn get_memory(&self) -> &HashMap<String, i64> {
        self.vm.get_memory()
    }

    pub fn get_stack(&self) -> &Vec<i64> {
        &self.vm.get_state().stack
    }
}