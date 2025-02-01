export const examplePrograms = {
    // Basic Operations
    basic: `// Basic Arithmetic
PUSH 10     // Push first operand
PUSH 5      // Push second operand
ADD         // Add them -> 15
DUP         // Duplicate 15 for later use
PRINT       // Print the first 15
PUSH 3      // Push multiplier
MUL         // Multiply 15 * 3 -> 45
PRINT       // Print the result
HALT        // Stop execution`,

    // Memory and Variables
    memory: `// Memory Operations
PUSH 42
STORE x    // Store 42 in variable 'x'
PUSH 100
STORE y    // Store 100 in variable 'y'
LOAD x     // Push value of x onto stack
LOAD y     // Push value of y onto stack
ADD        // Add x and y
PRINT      // Output: 142
HALT`,

    // Arrays
    arrays: `// Array Operations
PUSH 3       // Array size
NEWARRAY     // Create array of size 3

// Store values: [10, 20, 30]
DUP
PUSH 0
PUSH 10
ARRAYSET

DUP
PUSH 1
PUSH 20
ARRAYSET

DUP
PUSH 2
PUSH 30
ARRAYSET

// Read and print values
DUP
PUSH 0
ARRAYGET
PRINT       // Output: 10

DUP
PUSH 1
ARRAYGET
PRINT       // Output: 20

DUP
PUSH 2
ARRAYGET
PRINT       // Output: 30
HALT`,

    // Strings
    strings: `// String Operations
NEWSTR "Hello, "
NEWSTR "Virtual "
STRCAT        // Concatenate strings

NEWSTR "Machine!"
STRCAT        // Concatenate again
PRINT         // Output: Hello, Virtual Machine!

PRINTSTR "\\nString Demo Complete\\n"
HALT`,

    // Control Flow
    loops: `// Loop Example
// Print numbers from 1 to 5
PUSH 1        // Counter
STORE i

start_loop:
  LOAD i
  DUP
  PRINT       // Print current number
  
  PUSH 1
  ADD         // Increment counter
  DUP
  STORE i     // Update counter
  
  PUSH 5
  LessEqual   // Check if <= 5
  JMPNZ start_loop  // Continue if true

HALT`,

    // Comparison
    comparison: `// Comparison Operations
PUSH 10
PUSH 20
LT            // 10 < 20
PRINT         // Output: 1 (true)
PRINTSTR "\n"

PUSH 30
PUSH 30
LE            // 30 <= 30
PRINT         // Output: 1 (true)
PRINTSTR "\n"

PUSH 50
PUSH 40
GT            // 50 > 40
PRINT         // Output: 1 (true)
PRINTSTR "\n"
HALT`,

    // Calculator
    calculator: `// Simple Calculator
PRINTSTR "First number: "
PUSH 15       // Simulated input
PRINT

PRINTSTR "Second number: "
PUSH 5        // Simulated input
PRINT

PRINTSTR "\\nOperations:\\n"

DUP
OVER         // Duplicate numbers for multiple operations

// Addition
PRINTSTR "Sum: "
ADD
PRINT

// Subtraction
PRINTSTR "Difference: "
SUB
PRINT

// Multiplication
PRINTSTR "Product: "
MUL
PRINT

// Division
PRINTSTR "Quotient: "
DIV
PRINT

HALT`,

    // Character Printing
    chars: `// Character Printing
// Print "HELLO" using ASCII values
PUSH 72    // H
PRINTCHAR
PUSH 69    // E
PRINTCHAR
PUSH 76    // L
PRINTCHAR
PUSH 76    // L
PRINTCHAR
PUSH 79    // O
PRINTCHAR
HALT`
} as const;