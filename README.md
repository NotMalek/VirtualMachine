# Virtual Machine with Web Interface

A stack-based virtual machine implemented in Rust with a modern web interface built using Next.js. Features assembly language support, real-time execution visualization, and interactive debugging capabilities.

## Features

- **Virtual Machine Core**
    - Stack-based architecture
    - Rich instruction set
    - Memory management
    - Array and string operations
    - Dynamic heap allocation

- **Assembly Language Support**
    - Custom assembly language parser
    - Label support
    - Comments and documentation
    - Error detection and reporting

- **Interactive Web Interface**
    - Real-time stack visualization
    - Memory inspection
    - Program execution controls
    - Code editor with syntax highlighting
    - Step-by-step debugging

- **Robust Architecture**
    - Clean separation of concerns
    - Type-safe implementation
    - Error handling
    - CORS-enabled API

## Architecture

```
┌─────────────┐     ┌─────────────┐
│  Next.js UI │◄────┤ Rust Server │
└─────┬───────┘     └─────────────┘
      │                    │
Code Editor          Virtual Machine
      │                    │
   Program              Program
Visualization          Execution
```

## Prerequisites

- Rust 1.70 or later
- Node.js 18.0 or later
- npm or yarn

## Installation

1. Clone the repository:
```bash
git clone https://github.com/NotMalek/virtual-machine.git
cd virtual-machine
```

2. Install backend dependencies:
```bash
cargo build
```

3. Install frontend dependencies:
```bash
cd frontend
npm install
```

## Running the System

### 1. Start Backend
```bash
# From the root directory
cargo run
```

### 2. Start Frontend
```bash
# From the frontend directory
npm run dev
```

The application will be available at:
- Frontend: http://localhost:3000
- Backend API: http://localhost:3001

## Instruction Set

The VM supports the following instruction types:

### Stack Operations
- `PUSH <value>` - Push value onto stack
- `POP` - Remove top value from stack
- `DUP` - Duplicate top value
- `SWAP` - Swap top two values

### Arithmetic
- `ADD` - Add top two values
- `SUB` - Subtract top two values
- `MUL` - Multiply top two values
- `DIV` - Divide top two values

### Memory Operations
- `STORE <name>` - Store value in memory
- `LOAD <name>` - Load value from memory

### Control Flow
- `JMP <label>` - Jump to label
- `JMPZ <label>` - Jump if zero
- `JMPNZ <label>` - Jump if not zero

### Array Operations
- `NEWARRAY` - Create new array
- `ARRAYGET` - Get array element
- `ARRAYSET` - Set array element
- `ARRAYLEN` - Get array length

### String Operations
- `NEWSTRING <string>` - Create new string
- `STRCAT` - Concatenate strings
- `STRLEN` - Get string length

### I/O Operations
- `PRINT` - Print value
- `PRINTSTR <string>` - Print string
- `PRINTCHAR` - Print character

## Example Programs

### Basic Arithmetic
```assembly
// Add two numbers
PUSH 10
PUSH 5
ADD
PRINT  // Outputs: 15
```

### Array Manipulation
```assembly
// Create and manipulate array
PUSH 3       // array size
NEWARRAY
DUP
PUSH 0
PUSH 100
ARRAYSET     // arr[0] = 100
```

### String Operations
```assembly
NEWSTRING "Hello, "
NEWSTRING "VM!"
STRCAT
PRINT       // Outputs: Hello, VM!
```

## Project Structure

```
.
├── backend/
│   ├── src/
│   │   ├── core/
│   │   │   ├── assembler/
│   │   │   ├── error/
│   │   │   ├── heap/
│   │   │   ├── instruction/
│   │   │   ├── state/
│   │   │   └── vm/
│   │   └── main.rs
│   └── Cargo.toml
└── frontend/
    ├── src/
    │   ├── components/
    │   ├── services/
    │   ├── types/
    │   └── app/
    ├──  package.json
    └── tsconfig.json
```

## Web Interface

![image](https://github.com/user-attachments/assets/72d81d1a-ed66-47e5-b186-7e403812d814)


The web interface provides:
- Code editor with syntax highlighting
- Real-time stack visualization
- Memory inspection panel
- Program output display
- Step-by-step execution controls
- Sample programs

