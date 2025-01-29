pub const SOURCE: &str = r#"
    // String Operations Test
    PRINTSTR "=== String Operations Test ===\n"

    // Create strings
    PRINTSTR "1. Creating strings\n"
    PRINTSTR "-------------------\n"
    NEWSTR "Hello, "
    STORE s1
    NEWSTR "World!"
    STORE s2

    // Get and print lengths
    PRINTSTR "2. String lengths\n"
    PRINTSTR "-------------------\n"
    PRINTSTR "Length of s1: "
    LOAD s1
    STRLEN
    PRINT
    PUSH 10
    PRINTCHAR

    PRINTSTR "Length of s2: "
    LOAD s2
    STRLEN
    PRINT
    PUSH 10
    PRINTCHAR

    // Concatenate strings
    PRINTSTR "\n3. String concatenation\n"
    PRINTSTR "-------------------\n"
    LOAD s1
    LOAD s2
    STRCAT
    STORE result

    PRINTSTR "Concatenated length: "
    LOAD result
    STRLEN
    PRINT
    PUSH 10
    PRINTCHAR

    // Clean up
    LOAD s1
    FREESTR
    LOAD s2
    FREESTR
    LOAD result
    FREESTR

    PRINTSTR "\nProgram completed!\n"
    HALT
"#;