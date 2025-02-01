use super::VMTester;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conditional_branching() {
        const SOURCE: &str = r#"
        // Initialize counter
        start:  PUSH 1
                STORE x

        loop:   // Check if number is odd
                LOAD x
                PUSH 2
                DIV
                PUSH 2
                MUL
                LOAD x
                SUB
                PUSH -1
                MUL        // Convert negative to positive
                PRINT
                PRINTSTR "\n"

                // Increment x
                LOAD x
                PUSH 1
                ADD
                DUP
                STORE x

                // Continue if x < 4
                PUSH 4
                LT
                JMPNZ loop

        end:    HALT
        "#;

        let mut tester = VMTester::new(SOURCE, false)
            .expect("Failed to create VM tester");

        tester.run().expect("Failed to execute program");
        assert_eq!(tester.get_output(), "1\n0\n1\n");
    }
}