use crate::core::vm::VM;
use crate::core::assembler::Assembler;

pub const SOURCE: &str = r#"
    PRINTSTR "Hello, World!\n"
    PUSH 42
    PRINT
    HALT
"#;

#[test]
pub fn test_io_operations() {
    let mut assembler = Assembler::new();
    let program = assembler.assemble(SOURCE).unwrap();
    let mut vm = VM::new(program);

    while let Ok(true) = vm.step() {}

    let output = vm.take_output().join("");
    assert_eq!(output, "Hello, World!\n42\n");
}