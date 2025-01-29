pub const SOURCE: &str = r#"
    // Control Flow Test
    PRINTSTR "=== Control Flow Test ===\n"

    // 1. Basic loop with counter
    PRINTSTR "1. Counting from 1 to 5\n"
    PRINTSTR "---------------------\n"

    PUSH 1        // counter
count_loop:
    DUP
    PRINTSTR "Counter: "
    PRINT
    PUSH 10       // newline
    PRINTCHAR

    DUP
    PUSH 5
    LessEqual     // check if counter <= 5
    JMPZ end_count

    PUSH 1
    ADD           // increment counter
    JMP count_loop

end_count:
    POP           // clean up counter

    // 2. Conditional execution
    PRINTSTR "\n2. Conditional Execution\n"
    PRINTSTR "---------------------\n"

    PUSH 42
    PUSH 50
    LessThan
    JMPZ else_branch

    PRINTSTR "42 is less than 50\n"
    JMP end_if

else_branch:
    PRINTSTR "42 is not less than 50\n"

end_if:

    // 3. Nested conditions
    PRINTSTR "\n3. Nested Conditions\n"
    PRINTSTR "---------------------\n"

    PUSH 15
    DUP
    PUSH 10
    LessThan
    JMPZ else_outer

    PRINTSTR "Number is less than 10\n"
    JMP end_outer

else_outer:
    DUP
    PUSH 20
    LessThan
    JMPZ else_inner

    PRINTSTR "Number is between 10 and 20\n"
    JMP end_inner

else_inner:
    PRINTSTR "Number is 20 or greater\n"

end_inner:
end_outer:
    POP           // clean up number

    PRINTSTR "\nProgram completed!\n"
    HALT
"#;