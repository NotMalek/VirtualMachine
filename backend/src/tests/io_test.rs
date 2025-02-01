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
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic I/O test with string and number printing
    const BASIC_IO_SOURCE: &str = r#"
        PRINTSTR "Hello, World!\n"
        PUSH 42
        PRINT
        PRINTSTR "\n"
        HALT
    "#;

    // Test character printing
    const CHAR_PRINT_SOURCE: &str = r#"
        // Print "Hi!" using character codes
        PUSH 72    // 'H'
        PRINTCHAR
        PUSH 105   // 'i'
        PRINTCHAR
        PUSH 33    // '!'
        PRINTCHAR
        PRINTSTR "\n"
        HALT
    "#;

    // Test mixed output types
    const MIXED_OUTPUT_SOURCE: &str = r#"
        PRINTSTR "Count: "
        PUSH 1
        PRINT
        PRINTSTR "\n"

        PRINTSTR "Next: "
        PUSH 2
        PRINT
        PRINTSTR "\n"

        PRINTSTR "Done.\n"
        HALT
    "#;

    // Test string with escape sequences
    const ESCAPE_SEQUENCE_SOURCE: &str = r#"
        PRINTSTR "Line 1\nLine 2\nLine 3\n"
        HALT
    "#;

    #[test]
    fn test_basic_io() {
        let mut tester = VMTester::new(BASIC_IO_SOURCE, false)
            .expect("Failed to create VM tester");

        tester.run().expect("Failed to execute program");

        assert_eq!(tester.get_output(), "Hello, World!\n42\n",
                   "Basic I/O operations failed");
    }

    #[test]
    fn test_char_printing() {
        let mut tester = VMTester::new(CHAR_PRINT_SOURCE, false)
            .expect("Failed to create VM tester");

        tester.run().expect("Failed to execute program");

        assert_eq!(tester.get_output(), "Hi!\n",
                   "Character printing failed");
    }

    #[test]
    fn test_mixed_output() {
        let mut tester = VMTester::new(MIXED_OUTPUT_SOURCE, false)
            .expect("Failed to create VM tester");

        tester.run().expect("Failed to execute program");

        assert_eq!(tester.get_output(), "Count: 1\nNext: 2\nDone.\n",
                   "Mixed output types failed");
    }

    #[test]
    fn test_escape_sequences() {
        let mut tester = VMTester::new(ESCAPE_SEQUENCE_SOURCE, false)
            .expect("Failed to create VM tester");

        tester.run().expect("Failed to execute program");

        assert_eq!(tester.get_output(), "Line 1\nLine 2\nLine 3\n",
                   "Escape sequences failed");
    }
}