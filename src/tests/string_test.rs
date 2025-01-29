use crate::core::vm::VM;
use crate::core::assembler::Assembler;

pub const SOURCE: &str = r#"
    NEWSTR "Hello"
    STORE str1

    NEWSTR " World!"
    STORE str2

    LOAD str1
    LOAD str2
    STRCAT
    STORE result

    LOAD result
    STRLEN
    PRINT

    HALT
"#;

#[test]
pub fn test_string_operations() {
    let mut assembler = Assembler::new();
    let program = assembler.assemble(SOURCE).unwrap();
    let mut vm = VM::new(program);

    while let Ok(true) = vm.step() {}

    let output = vm.take_output().join("");
    assert_eq!(output, "12\n");
}