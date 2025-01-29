use crate::core::vm::VM;
use crate::core::state::DebugOptions; // Correct import path
use crate::core::assembler::Assembler;

mod core;
mod examples;

fn run_program(source: &str, name: &str) {
    println!("\n=== Running {} Example ===", name);
    println!("Source code:");
    println!("{}", source);
    println!("\nOutput:");

    let mut assembler = Assembler::new();
    match assembler.assemble(source) {
        Ok(program) => {
            let mut vm = VM::new(program);
            vm.set_debug_options(DebugOptions {
                show_instructions: true,
                show_stack: true,
                show_pc: false,
                show_memory: false,
            });

            while let Ok(true) = vm.step() {}

            let output = vm.take_output().join("")
                .replace("\\n", "\n");
            print!("{}", output);
        }
        Err(e) => {
            println!("Assembly Error: {}", e);
        }
    }
    println!("\n=== End of {} Example ===\n", name);
}

fn main() {
    println!("Virtual Machine Example Programs");
    println!("===============================");

    // Run each example program
    run_program(examples::io_test::SOURCE, "I/O Operations");
    run_program(examples::array_test::SOURCE, "Array Operations");
    run_program(examples::string_test::SOURCE, "String Operations");
    run_program(examples::arithmetic_test::SOURCE, "Arithmetic Operations");
    run_program(examples::control_test::SOURCE, "Control Flow");
}