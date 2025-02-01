use crate::core::vm::VM;
use crate::core::assembler::Assembler;
use crate::core::state::DebugOptions;
use crate::core::error::VMError;

struct VMTester {
    vm: VM,
    step_count: usize,
    max_steps: usize,
    all_output: Vec<String>,
}

impl VMTester {
    fn new(source: &str, debug: bool) -> Result<Self, String> {
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

    fn run(&mut self) -> Result<(), VMError> {
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

    fn get_output(&self) -> String {
        self.all_output.join("")
    }

    fn get_memory(&self) -> &std::collections::HashMap<String, i64> {
        self.vm.get_memory()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_ARRAY_SOURCE: &str = r#"
        // Create array of size 3
        PUSH 3
        NEWARRAY
        STORE arr

        // Set arr[0] = 10
        LOAD arr
        PUSH 0
        PUSH 10
        ARRAYSET

        // Set arr[1] = 20
        LOAD arr
        PUSH 1
        PUSH 20
        ARRAYSET

        // Print arr[0]
        LOAD arr
        PUSH 0
        ARRAYGET
        PRINT
        PRINTSTR "\n"

        // Print arr[1]
        LOAD arr
        PUSH 1
        ARRAYGET
        PRINT
        PRINTSTR "\n"

        HALT
    "#;

    const ARRAY_LENGTH_SOURCE: &str = r#"
        // Create and test array length
        PUSH 5
        NEWARRAY
        DUP
        ARRAYLEN
        PRINT
        PRINTSTR "\n"
        HALT
    "#;

    #[test]
    fn test_array_basic_operations() {
        let mut tester = VMTester::new(BASIC_ARRAY_SOURCE, false)
            .expect("Failed to create VM tester");

        tester.run().expect("Failed to execute program");

        assert_eq!(tester.get_output(), "10\n20\n",
                   "Array get/set operations failed");
    }

    #[test]
    fn test_array_length() {
        let mut tester = VMTester::new(ARRAY_LENGTH_SOURCE, false)
            .expect("Failed to create VM tester");

        tester.run().expect("Failed to execute program");

        assert_eq!(tester.get_output(), "5\n",
                   "Array length operation failed");
    }

    #[test]
    fn test_array_bounds_error() {
        const BOUNDS_ERROR_SOURCE: &str = r#"
            PUSH 2
            NEWARRAY
            PUSH 2  // Invalid index
            ARRAYGET
            HALT
        "#;

        let mut tester = VMTester::new(BOUNDS_ERROR_SOURCE, false)
            .expect("Failed to create VM tester");

        match tester.run() {
            Err(VMError::ArrayBoundsError(_, _)) => (),
            _ => panic!("Expected array bounds error"),
        }
    }

    #[test]
    fn test_array_memory_management() {
        const MEMORY_SOURCE: &str = r#"
            // Create and free array
            PUSH 3
            NEWARRAY
            DUP         // Duplicate array reference
            FREEARR    // Free the array
            HALT
        "#;

        let mut tester = VMTester::new(MEMORY_SOURCE, false)
            .expect("Failed to create VM tester");

        tester.run().expect("Failed to execute program");
    }
}