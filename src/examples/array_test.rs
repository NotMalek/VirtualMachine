pub const SOURCE: &str = r#"
    // Array Operations Test
    PRINTSTR "=== Array Operations Test ===\n"

    // Create an array
    PRINTSTR "1. Creating array of size 5\n"
    PRINTSTR "-------------------------\n"
    PUSH 5
    NEWARRAY
    STORE arr

    // Fill array with values
    PRINTSTR "2. Filling array with values\n"
    PRINTSTR "-------------------------\n"

    // Store values 10,20,30,40,50
    PUSH 0      // counter
fill_loop:
    DUP         // duplicate counter for comparison
    PUSH 5
    LessThan    // check if counter < 5
    JMPZ end_fill

    // Calculate value (counter * 10 + 10)
    DUP         // counter for calculation
    PUSH 10
    MUL
    PUSH 10
    ADD

    // Store in array
    LOAD arr    // get array reference
    DUP         // counter for index
    DUP         // value to store
    PRINTSTR "Setting arr["
    DUP         // index for printing
    PRINT
    PRINTSTR "] = "
    DUP         // value for printing
    PRINT
    PUSH 10     // newline
    PRINTCHAR

    ARRAYSET

    // Increment counter
    PUSH 1
    ADD
    JMP fill_loop

end_fill:
    POP         // remove counter

    // Read and verify values
    PRINTSTR "\n3. Reading array values\n"
    PRINTSTR "-------------------------\n"

    PUSH 0      // counter
read_loop:
    DUP         // counter for comparison
    PUSH 5
    LessThan
    JMPZ end_read

    PRINTSTR "arr["
    DUP
    PRINT
    PRINTSTR "] = "

    LOAD arr
    SWAP
    ARRAYGET
    PRINT
    PUSH 10     // newline
    PRINTCHAR

    PUSH 1
    ADD
    JMP read_loop

end_read:
    POP         // remove counter

    // Get and print array length
    PRINTSTR "\n4. Array length\n"
    PRINTSTR "-------------------------\n"
    PRINTSTR "Array length: "
    LOAD arr
    ARRAYLEN
    PRINT
    PUSH 10     // newline
    PRINTCHAR

    // Clean up
    LOAD arr
    FREEARR

    PRINTSTR "\nProgram completed!\n"
    HALT
"#;