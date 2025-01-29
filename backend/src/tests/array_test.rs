use crate::core::vm::VM;
use crate::core::assembler::Assembler;

pub const SOURCE: &str = r#"
    PUSH 3
    NEWARRAY
    STORE arr

    LOAD arr
    PUSH 0
    PUSH 10
    ARRAYSET

    LOAD arr
    PUSH 1
    PUSH 20
    ARRAYSET

    LOAD arr
    PUSH 0
    ARRAYGET
    PRINT

    LOAD arr
    PUSH 1
    ARRAYGET
    PRINT

    HALT
"#;

#[test]
pub fn test_array_operations() {
    let mut assembler = Assembler::new();
    let program = assembler.assemble(SOURCE).unwrap();
    let mut vm = VM::new(program);

    while let Ok(true) = vm.step() {}

    let output = vm.take_output().join("");
    assert_eq!(output, "10\n20\n");
}