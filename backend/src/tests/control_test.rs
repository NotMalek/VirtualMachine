use crate::core::vm::VM;
use crate::core::assembler::Assembler;

pub const SOURCE: &str = r#"
    start: PUSH 0
    STORE counter

    loop: LOAD counter
    PUSH 1
    ADD
    STORE counter

    LOAD counter
    PRINT

    LOAD counter
    PUSH 5
    LT
    JMPNZ loop

    end: HALT
"#;

#[test]
pub fn test_control_flow() {
    let mut assembler = Assembler::new();
    let program = assembler.assemble(SOURCE).unwrap();
    let mut vm = VM::new(program);

    while let Ok(true) = vm.step() {}

    let output = vm.take_output().join("");
    assert_eq!(output, "1\n2\n3\n4\n5\n");
}