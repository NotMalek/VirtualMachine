use crate::core::vm::VM;
use crate::core::assembler::Assembler;
use crate::core::state::DebugOptions;
use crate::core::error::VMError;

// Test helper struct to simplify test execution
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

    fn get_stack(&self) -> &Vec<i64> {
        &self.vm.get_state().stack
    }

    fn get_memory(&self) -> &std::collections::HashMap<String, i64> {
        self.vm.get_memory()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARITHMETIC_SOURCE: &str = r#"
        // Basic arithmetic operations test
        PUSH 10
        PUSH 20
        ADD
        DUP
        PRINT       // Should print 30
        PRINTSTR "\n"

        PUSH 20
        SUB
        DUP
        PRINT       // Should print 10
        PRINTSTR "\n"

        PUSH 2
        MUL
        DUP
        PRINT       // Should print 20
        PRINTSTR "\n"

        PUSH 4
        DIV
        PRINT       // Should print 5
        PRINTSTR "\n"

        HALT
    "#;

    const MEMORY_SOURCE: &str = r#"
        // Memory operations test
        PUSH 42
        STORE x
        PUSH 24
        STORE y
        LOAD x
        LOAD y
        ADD
        PRINT       // Should print 66
        PRINTSTR "\n"
        HALT
    "#;

    const COMPARISON_SOURCE: &str = r#"
        // Comparison operations test
        PUSH 10
        PUSH 20
        LT          // 10 < 20
        PRINT       // Should print 1
        PRINTSTR "\n"

        PUSH 30
        PUSH 20
        GT          // 30 > 20
        PRINT       // Should print 1
        PRINTSTR "\n"

        PUSH 10
        PUSH 10
        LE          // 10 <= 10
        PRINT       // Should print 1
        PRINTSTR "\n"

        PUSH 5
        PUSH 10
        GE          // 5 >= 10
        PRINT       // Should print 0
        PRINTSTR "\n"
        HALT
    "#;

    #[test]
    fn test_arithmetic_operations() {
        let mut tester = VMTester::new(ARITHMETIC_SOURCE, false)
            .expect("Failed to create VM tester");

        tester.run().expect("Failed to execute program");

        assert_eq!(tester.get_output(), "30\n10\n20\n5\n");
        assert!(tester.get_stack().is_empty(), "Stack should be empty after execution");
    }

    #[test]
    fn test_memory_operations() {
        let mut tester = VMTester::new(MEMORY_SOURCE, false)
            .expect("Failed to create VM tester");

        tester.run().expect("Failed to execute program");

        assert_eq!(tester.get_output(), "66\n");
        assert_eq!(tester.get_memory().get("x"), Some(&42));
        assert_eq!(tester.get_memory().get("y"), Some(&24));
    }

    #[test]
    fn test_comparison_operations() {
        let mut tester = VMTester::new(COMPARISON_SOURCE, false)
            .expect("Failed to create VM tester");

        tester.run().expect("Failed to execute program");

        assert_eq!(tester.get_output(), "1\n1\n1\n0\n");
    }

    #[test]
    fn test_error_stack_underflow() {
        const UNDERFLOW_SOURCE: &str = r#"
            POP     // Stack is empty, should cause underflow
            HALT
        "#;

        let mut tester = VMTester::new(UNDERFLOW_SOURCE, false)
            .expect("Failed to create VM tester");

        match tester.run() {
            Err(VMError::StackUnderflow) => (),
            _ => panic!("Expected stack underflow error"),
        }
    }

    #[test]
    fn test_error_division_by_zero() {
        const DIV_ZERO_SOURCE: &str = r#"
            PUSH 10
            PUSH 0
            DIV     // Division by zero
            HALT
        "#;

        let mut tester = VMTester::new(DIV_ZERO_SOURCE, false)
            .expect("Failed to create VM tester");

        match tester.run() {
            Err(VMError::DivisionByZero) => (),
            _ => panic!("Expected division by zero error"),
        }
    }
}