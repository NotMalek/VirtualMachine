use super::VMTester;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string_operations() {
        const SOURCE: &str = r#"
        // Test empty string length
        start:  NEWSTR ""
                STORE s1

        // Get and print empty string length
        loop1:  LOAD s1
                STRLEN
                PRINT
                PRINTSTR "\n"

        // Concatenate with non-empty string
        loop2:  LOAD s1
                NEWSTR "Hello"
                STRCAT
                DUP
                STRLEN
                PRINT
                PRINTSTR "\n"

        end:    HALT
        "#;

        let mut tester = VMTester::new(SOURCE, false)
            .expect("Failed to create VM tester");

        tester.run().expect("Failed to execute program");
        assert_eq!(tester.get_output(), "0\n5\n");
    }
}