mod core;

use core::vm::{VM, Instruction};

fn main() {
    // Program that defines and uses a function to calculate factorial
    let program = vec![
        // Define factorial function
        Instruction::DefineFunction("factorial".into(), 1),  // One parameter
        Instruction::BeginFunction,
        Instruction::PushParam(0),                 // Get the parameter
        Instruction::CreateLocal("n".into()),      // Store in local variable

        // If n <= 1, return 1
        Instruction::LoadLocal("n".into()),
        Instruction::Push(1),
        Instruction::LessEqual,
        Instruction::JumpIfZero(11),              // If n > 1, continue
        Instruction::Push(1),                     // Return 1
        Instruction::Return,

        // Else return n * factorial(n-1)
        Instruction::LoadLocal("n".into()),        // Push n
        Instruction::Push(1),
        Instruction::Sub,                         // n - 1
        Instruction::Call("factorial".into()),     // Recursive call
        Instruction::LoadLocal("n".into()),        // Push n again
        Instruction::Mul,                         // Multiply
        Instruction::Return,
        Instruction::EndFunction,

        // Main program
        Instruction::Push(5),                     // Calculate 5!
        Instruction::Call("factorial".into()),
        Instruction::Halt,
    ];

    let mut vm = VM::new(program);
    vm.enable_debug();

    println!("Calculating factorial of 5");
    println!("--------------------------");

    while let Ok(true) = vm.step() {
        if let Some(&value) = vm.get_state().stack.last() {
            if matches!(vm.get_state().instructions().last(), Some(Instruction::Halt)) {
                println!("\nResult: 5! = {}", value);
            }
        }
    }
}