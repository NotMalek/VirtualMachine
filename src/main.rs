use virtual_machine::core::{
    vm::{VM, DebugOptions},
    assembler::Assembler
};

fn main() {
    let source = r#"
        // Initialize counter
                PUSH 5          // Initial value
                STORE counter   // Store in memory

        // Main loop
        loop:   LOAD counter    // Get counter value
                JMPZ done      // If counter is 0, we're done

                LOAD counter   // Load for decrement
                PUSH 1        // Prepare for decrement
                SUB          // counter -= 1
                STORE counter // Save decremented value

                JMP loop      // Continue

        done:   HALT          // Stop execution
    "#;

    let mut assembler = Assembler::new();
    match assembler.assemble(source) {
        Ok(program) => {
            println!("\nAssembly successful!");

            let mut vm = VM::new(program);

            // Configure debug options
            vm.set_debug_options(DebugOptions {
                show_instructions: true,
                show_stack: false,  // We'll track this manually
                show_pc: false,     // Not needed for this demo
                show_memory: false, // We'll track counter manually
            });

            println!("\nExecuting program:");
            println!("-----------------");

            let mut last_counter = None;
            while let Ok(true) = vm.step() {
                // Only show counter when it changes
                if let Some(&value) = vm.get_state().memory.get("counter") {
                    if last_counter != Some(value) {
                        println!("Counter decremented to: {}", value);
                        last_counter = Some(value);
                    }
                }
            }

            if let Some(&final_value) = vm.get_state().memory.get("counter") {
                println!("\nProgram finished!");
                println!("Final counter value: {}", final_value);

                if final_value == 0 {
                    println!("Successfully counted down to zero!");
                }
            }
        }
        Err(e) => println!("Assembly error: {}", e),
    }
}