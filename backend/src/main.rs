use virtual_machine::core::vm::VM;
use virtual_machine::core::assembler::Assembler;
use virtual_machine::core::state::DebugOptions;

fn main() {
    // Create a sample program that demonstrates various VM features
    let source = r#"
        // Basic arithmetic
        PUSH 10
        PUSH 5
        ADD
        PRINT

        // Store and load
        PUSH 42
        STORE x
        LOAD x
        PRINT

        // Array operations
        PUSH 3       // array size
        NEWARRAY

        // Store some values
        DUP
        PUSH 0
        PUSH 100
        ARRAYSET

        DUP
        PUSH 1
        PUSH 200
        ARRAYSET

        // Read and print a value
        DUP
        PUSH 1
        ARRAYGET
        PRINT

        // String operations
        NEWSTRING "Hello, "
        NEWSTRING "VM!"
        STRINGCONCAT
        PRINT

        HALT
    "#;

    println!("Starting VM with sample program...\n");

    // Create assembler and VM instances
    let mut assembler = Assembler::new();
    match assembler.assemble(source) {
        Ok(program) => {
            let mut vm = VM::new(program);

            // Optional: Enable debug options to see what's happening
            vm.set_debug_options(DebugOptions {
                show_instructions: true,
                show_stack: true,
                show_pc: false,
                show_memory: false,
            });

            // Run the program
            println!("Executing program...\n");
            while let Ok(true) = vm.step() {}

            println!("\nProgram output:");
            for output in vm.take_output() {
                println!("{}", output);
            }
        }
        Err(e) => {
            println!("Failed to assemble program: {:?}", e);
        }
    }
}