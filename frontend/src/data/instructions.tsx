export interface InstructionDoc {
    name: string;
    description: string;
    example: string;
    category: string;
}

export const instructions: InstructionDoc[] = [
    // Stack Operations
    {
        name: "PUSH <value>",
        description: "Push a numeric value onto the stack",
        example: "PUSH 42",
        category: "Stack Operations"
    },
    {
        name: "POP",
        description: "Remove and discard the top value from the stack",
        example: "POP",
        category: "Stack Operations"
    },
    {
        name: "DUP",
        description: "Duplicate the top value on the stack",
        example: "DUP",
        category: "Stack Operations"
    },
    {
        name: "SWAP",
        description: "Swap the top two values on the stack",
        example: "SWAP",
        category: "Stack Operations"
    },

    // Arithmetic Operations
    {
        name: "ADD",
        description: "Add the top two values on the stack",
        example: "PUSH 5\nPUSH 3\nADD  // Result: 8",
        category: "Arithmetic"
    },
    {
        name: "SUB",
        description: "Subtract the top value from the second top value",
        example: "PUSH 5\nPUSH 3\nSUB  // Result: 2",
        category: "Arithmetic"
    },
    {
        name: "MUL",
        description: "Multiply the top two values on the stack",
        example: "PUSH 5\nPUSH 3\nMUL  // Result: 15",
        category: "Arithmetic"
    },
    {
        name: "DIV",
        description: "Divide the second top value by the top value",
        example: "PUSH 6\nPUSH 2\nDIV  // Result: 3",
        category: "Arithmetic"
    },

    // Memory Operations
    {
        name: "STORE <name>",
        description: "Store the top value in memory with the given name",
        example: "PUSH 42\nSTORE x",
        category: "Memory"
    },
    {
        name: "LOAD <name>",
        description: "Push the value from memory onto the stack",
        example: "LOAD x",
        category: "Memory"
    },

    // Control Flow
    {
        name: "JMP <label>",
        description: "Jump to the specified label",
        example: "JMP loop_start",
        category: "Control Flow"
    },
    {
        name: "JMPZ <label>",
        description: "Jump to the label if the top value is zero",
        example: "JMPZ end_loop",
        category: "Control Flow"
    },
    {
        name: "JMPNZ <label>",
        description: "Jump to the label if the top value is not zero",
        example: "JMPNZ continue_loop",
        category: "Control Flow"
    },

    // Array Operations
    {
        name: "NEWARRAY",
        description: "Create a new array of size specified by the top value",
        example: "PUSH 5\nNEWARRAY  // Creates array of size 5",
        category: "Arrays"
    },
    {
        name: "ARRAYGET",
        description: "Get value at index (top) from array (second top)",
        example: "LOAD arr\nPUSH 0\nARRAYGET",
        category: "Arrays"
    },
    {
        name: "ARRAYSET",
        description: "Set array (third top) at index (second top) to value (top)",
        example: "LOAD arr\nPUSH 0\nPUSH 42\nARRAYSET",
        category: "Arrays"
    },
    {
        name: "ARRAYLEN",
        description: "Get the length of the array on top of the stack",
        example: "LOAD arr\nARRAYLEN",
        category: "Arrays"
    },
    {
        name: "FREEARR",
        description: "Free the array on top of the stack",
        example: "LOAD arr\nFREEARR",
        category: "Arrays"
    },

    // String Operations
    {
        name: "NEWSTR <string>",
        description: "Create a new string and push its reference onto the stack",
        example: 'NEWSTR "Hello, World!"',
        category: "Strings"
    },
    {
        name: "STRCAT",
        description: "Concatenate two strings on the stack",
        example: 'NEWSTR "Hello, "\nNEWSTR "World!"\nSTRCAT',
        category: "Strings"
    },
    {
        name: "STRLEN",
        description: "Get the length of the string on top of the stack",
        example: 'NEWSTR "Hello"\nSTRLEN  // Result: 5',
        category: "Strings"
    },
    {
        name: "FREESTR",
        description: "Free the string on top of the stack",
        example: 'NEWSTR "Hello"\nFREESTR',
        category: "Strings"
    },

    // I/O Operations
    {
        name: "PRINT",
        description: "Print the top value from the stack",
        example: "PUSH 42\nPRINT",
        category: "I/O"
    },
    {
        name: "PRINTSTR <string>",
        description: "Print a string literal",
        example: 'PRINTSTR "Hello, World!"',
        category: "I/O"
    },
    {
        name: "PRINTCHAR",
        description: "Print the top value as an ASCII character",
        example: "PUSH 65\nPRINTCHAR  // Prints: A",
        category: "I/O"
    },
    {
        name: "HALT",
        description: "Stop program execution",
        example: "HALT",
        category: "Control Flow"
    },
];