export enum InstructionType {
    // Stack Operations
    Push = 'Push',
    Pop = 'Pop',
    Dup = 'Dup',
    Swap = 'Swap',

    // Arithmetic
    Add = 'Add',
    Sub = 'Sub',
    Mul = 'Mul',
    Div = 'Div',

    // Memory
    Load = 'Load',
    Store = 'Store',

    // Control Flow
    Jump = 'Jump',
    JumpIf = 'JumpIf',
    JumpIfZero = 'JumpIfZero',
    JumpIfNotZero = 'JumpIfNotZero',

    // Array operations
    NewArray = 'NewArray',
    ArrayGet = 'ArrayGet',
    ArraySet = 'ArraySet',
    ArrayLength = 'ArrayLength',
    FreeArray = 'FreeArray',

    // String operations
    NewString = 'NewString',
    StringConcat = 'StringConcat',
    StringLength = 'StringLength',
    FreeString = 'FreeString',

    // I/O Operations
    Print = 'Print',
    PrintChar = 'PrintChar',
    PrintStr = 'PrintStr',

    Halt = 'Halt',
}

export type Instruction = {
    type: InstructionType;
    value?: number | string;
};

export type VMState = {
    stack: number[];
    memory: Record<string, number>;
    programCounter: number;
    instructions: Instruction[];
    output: string[];
};