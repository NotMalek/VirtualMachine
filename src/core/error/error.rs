use thiserror::Error;

#[derive(Error, Debug)]
pub enum VMError {
    #[error("Stack underflow")]
    StackUnderflow,

    #[error("Invalid memory access at {0}")]
    InvalidMemoryAccess(usize),

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Invalid instruction at {0}")]
    InvalidInstruction(usize),

    #[error("Function not found: {0}")]
    FunctionNotFound(String),

    #[error("Empty call stack")]
    EmptyCallStack,

    #[error("Local variable not found: {0}")]
    LocalVarNotFound(String),

    #[error("Invalid parameter index: {0}")]
    InvalidParameter(usize),

    #[error("Invalid character code: {0}")]
    InvalidCharacter(i64),

    #[error("I/O error: {0}")]
    IOError(String),

    #[error("Invalid heap address: {0}")]
    InvalidHeapAddress(usize),

    #[error("Invalid array index: {0}")]
    InvalidArrayIndex(i64),

    #[error("Array bounds error: index {0} out of bounds {1}")]
    ArrayBoundsError(i64, usize),

    #[error("Type error: expected {0}, found {1}")]
    TypeError(String, String),
}