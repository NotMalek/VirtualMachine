pub const SOURCE: &str = r#"
    // Arithmetic Operations Test
    PRINTSTR "=== Arithmetic Operations Test ===\n"

    // Addition
    PRINTSTR "1. Addition\n"
    PRINTSTR "------------\n"
    PUSH 5
    PUSH 3
    PRINTSTR "5 + 3 = "
    ADD
    PRINT
    PUSH 10
    PRINTCHAR

    // Subtraction
    PRINTSTR "\n2. Subtraction\n"
    PRINTSTR "------------\n"
    PUSH 10
    PUSH 4
    PRINTSTR "10 - 4 = "
    SUB
    PRINT
    PUSH 10
    PRINTCHAR

    // Multiplication
    PRINTSTR "\n3. Multiplication\n"
    PRINTSTR "------------\n"
    PUSH 6
    PUSH 7
    PRINTSTR "6 * 7 = "
    MUL
    PRINT
    PUSH 10
    PRINTCHAR

    // Division
    PRINTSTR "\n4. Division\n"
    PRINTSTR "------------\n"
    PUSH 20
    PUSH 5
    PRINTSTR "20 / 5 = "
    DIV
    PRINT
    PUSH 10
    PRINTCHAR

    // Complex expression: (10 + 5) * (20 - 15)
    PRINTSTR "\n5. Complex Expression\n"
    PRINTSTR "------------\n"
    PRINTSTR "(10 + 5) * (20 - 15) = "
    PUSH 10
    PUSH 5
    ADD     // 10 + 5 = 15
    PUSH 20
    PUSH 15
    SUB     // 20 - 15 = 5
    MUL     // 15 * 5 = 75
    PRINT
    PUSH 10
    PRINTCHAR

    PRINTSTR "\nProgram completed!\n"
    HALT
"#;