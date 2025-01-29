pub const SOURCE: &str = r#"
    // I/O Operations Test
    PRINTSTR "=== I/O Operations Test ===\n"
    PRINTSTR "1. String output\n"
    PRINTSTR "-------------------\n"

    // Test different types of output
    PRINTSTR "Hello, World!\n"

    // Number output
    PRINTSTR "\n2. Number output\n"
    PRINTSTR "-------------------\n"
    PUSH 42
    PRINTSTR "The answer is: "
    PRINT
    PUSH 10
    PRINTCHAR

    // Character output
    PRINTSTR "\n3. Character output\n"
    PRINTSTR "-------------------\n"
    PRINTSTR "ASCII characters: "
    PUSH 65      // 'A'
    PRINTCHAR
    PUSH 66      // 'B'
    PRINTCHAR
    PUSH 67      // 'C'
    PRINTCHAR
    PUSH 10      // newline
    PRINTCHAR

    // Combined output
    PRINTSTR "\n4. Combined output\n"
    PRINTSTR "-------------------\n"
    PUSH 12345
    PRINTSTR "Number: "
    PRINT
    PUSH 10
    PRINTCHAR

    PRINTSTR "Program completed!\n"
    HALT
"#;