use virtual_machine::core::vm::{VM, DebugOptions, Instruction};
use virtual_machine::core::assembler::Assembler;

fn main() {
    let source = r#"
        // Test I/O operations
        PRINTSTR "=== I/O Test Program ===\n"
        PRINTSTR "-------------------------\n"

        // Test character output
        PRINTSTR "1. Character output: "
        PUSH 65      // ASCII 'A'
        PRINTCHAR
        PUSH 10      // Newline
        PRINTCHAR

        // Test numeric output
        PRINTSTR "2. Number output: "
        PUSH 42      // Number
        PRINT
        PUSH 10      // Newline
        PRINTCHAR

        // Test string output
        PRINTSTR "3. String output: Hello, World!\n"

        PRINTSTR "-------------------------\n"
        PRINTSTR "Program completed!\n"
        HALT
    "#;

    let mut assembler = Assembler::new();
    match assembler.assemble(source) {
        Ok(program) => {
            // Print debug header
            println!("\n┌─────────────────────────────────────┐");
            println!("│          VM Debug Output            │");
            println!("├─────────────────────────────────────┤");

            let mut vm = VM::new(program);
            vm.set_debug_options(DebugOptions {
                show_instructions: true,
                show_stack: true,
                show_pc: false,
                show_memory: false,
            });

            // Execute program
            while let Ok(true) = vm.step() {}

            println!("└─────────────────────────────────────┘");

            // Print program output
            println!("\n┌─────────────────────────────────────┐");
            println!("│          Program Output             │");
            println!("└─────────────────────────────────────┘\n");

            // Join output and fix newlines
            let output = vm.take_output().join("")
                .replace("\\n", "\n");
            print!("{}", output);
        }
        Err(e) => {
            println!("\n┌─────────────────────────────────────┐");
            println!("│          Assembly Error             │");
            println!("└─────────────────────────────────────┘\n");
            println!("Error: {}", e);
        }
    }
}