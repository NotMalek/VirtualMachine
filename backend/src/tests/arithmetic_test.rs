use crate::core::vm::VM;
use crate::core::assembler::Assembler;
use crate::core::state::DebugOptions;

pub const SOURCE: &str = r#"
    // First calculate 30 (10 + 20) and print it
    PUSH 10
    PUSH 20
    ADD
    PRINT

    // Then calculate 10 (30 - 20) and print it
    PUSH 30    // Push fresh value of 30
    PUSH 20
    SUB
    PRINT

    // Then calculate 20 (10 * 2) and print it
    PUSH 10    // Push fresh value of 10
    PUSH 2
    MUL
    PRINT

    // Finally calculate 5 (20 / 4) and print it
    PUSH 20    // Push fresh value of 20
    PUSH 4
    DIV
    PRINT

    HALT
"#;

#[test]
pub fn test_arithmetic_operations() {
    let mut assembler = Assembler::new();
    println!("Original source:\n{}", SOURCE);

    let program = assembler.assemble(SOURCE).unwrap();
    println!("Assembled instructions: {:?}", program);

    let mut vm = VM::new(program);

    vm.set_debug_options(DebugOptions {
        show_instructions: true,
        show_stack: true,
        show_pc: true,
        show_memory: true,
    });

    let mut step_count = 0;
    let max_steps = 100;
    let mut all_output = Vec::new();

    while step_count < max_steps {
        println!("\nStep {}", step_count);
        println!("Stack before: {:?}", vm.get_state().stack);

        match vm.step() {
            Ok(true) => {
                let output = vm.take_output();
                if !output.is_empty() {
                    println!("Output: {:?}", output);
                    all_output.extend(output);
                }
                println!("Stack after: {:?}", vm.get_state().stack);
                step_count += 1;
            },
            Ok(false) => {
                println!("Program halted normally at step {}", step_count);
                break;
            },
            Err(e) => {
                println!("Error at step {}: {:?}", step_count, e);
                println!("Current stack: {:?}", vm.get_state().stack);
                println!("Current instruction: {:?}", vm.current_instruction());
                break;
            }
        }
    }

    let final_output = all_output.join("");
    println!("\nFinal Results:");
    println!("Steps executed: {}", step_count);
    println!("Final output: {:?}", final_output);
    println!("Final stack: {:?}", vm.get_state().stack);

    assert_eq!(final_output, "30\n10\n20\n5\n");
}