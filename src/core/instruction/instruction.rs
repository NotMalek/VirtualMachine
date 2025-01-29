use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Instruction {
    // Stack Operations
    Push(i64),
    Pop,
    Dup,
    Swap,

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,

    // Memory
    Load(String),
    Store(String),

    // Control Flow
    Jump(usize),
    JumpIf(usize),
    JumpIfZero(usize),
    JumpIfNotZero(usize),

    // Comparison operations
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,

    // Boolean operations
    And,
    Or,
    Not,

    // Function instructions
    DefineFunction(String, usize),
    BeginFunction,
    EndFunction,
    CreateLocal(String),
    LoadLocal(String),
    StoreLocal(String),
    PushParam(usize),
    Call(String),
    Return,

    // Array operations
    NewArray,
    ArrayGet,
    ArraySet,
    ArrayLength,
    FreeArray,

    // String operations
    NewString(String),
    StringConcat,
    StringLength,
    FreeString,

    // I/O Operations
    Print,
    PrintChar,
    PrintStr(String),

    Halt,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Push(n) => write!(f, "PUSH {}", n),
            Instruction::Pop => write!(f, "POP"),
            Instruction::Dup => write!(f, "DUP"),
            Instruction::Swap => write!(f, "SWAP"),
            Instruction::Add => write!(f, "ADD"),
            Instruction::Sub => write!(f, "SUB"),
            Instruction::Mul => write!(f, "MUL"),
            Instruction::Div => write!(f, "DIV"),
            Instruction::Load(var) => write!(f, "LOAD {}", var),
            Instruction::Store(var) => write!(f, "STORE {}", var),
            Instruction::Jump(addr) => write!(f, "JMP {}", addr),
            Instruction::JumpIf(addr) => write!(f, "JMP_IF {}", addr),
            Instruction::JumpIfZero(addr) => write!(f, "JMPZ {}", addr),
            Instruction::JumpIfNotZero(addr) => write!(f, "JMPNZ {}", addr),
            Instruction::Print => write!(f, "PRINT"),
            Instruction::PrintChar => write!(f, "PRINTCHAR"),
            Instruction::PrintStr(s) => write!(f, "PRINTSTR \"{}\"", s),
            Instruction::NewArray => write!(f, "NEWARRAY"),
            Instruction::ArrayGet => write!(f, "ARRAYGET"),
            Instruction::ArraySet => write!(f, "ARRAYSET"),
            Instruction::ArrayLength => write!(f, "ARRAYLEN"),
            Instruction::FreeArray => write!(f, "FREEARR"),
            Instruction::NewString(s) => write!(f, "NEWSTR \"{}\"", s),
            Instruction::StringConcat => write!(f, "STRCAT"),
            Instruction::StringLength => write!(f, "STRLEN"),
            Instruction::FreeString => write!(f, "FREESTR"),
            Instruction::Halt => write!(f, "HALT"),
            _ => write!(f, "{:?}", self),
        }
    }
}