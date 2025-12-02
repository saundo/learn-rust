// Ownership Examples Runner
// Run individual examples or all at once

mod ownership;

use std::env;

fn print_menu() {
    println!("\n=== Ownership Examples ===");
    println!("1. Ownership Basics");
    println!("2. Borrowing");
    println!("3. Mutable Borrowing");
    println!("4. String Types");
    println!("5. Lifetimes");
    println!("all. Run all examples");
    println!("\nUsage: cargo run --example ownership [1-5|all]");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let choice = if args.len() > 1 {
        args[1].as_str()
    } else {
        print_menu();
        return;
    };
    
    match choice {
        "1" => ownership::basics::main(),
        "2" => ownership::borrowing::main(),
        "3" => ownership::mutable_borrowing::main(),
        "4" => ownership::string_types::main(),
        "5" => ownership::lifetimes::main(),
        "all" => {
            ownership::basics::main();
            println!("\n{}\n", "=".repeat(50));
            ownership::borrowing::main();
            println!("\n{}\n", "=".repeat(50));
            ownership::mutable_borrowing::main();
            println!("\n{}\n", "=".repeat(50));
            ownership::string_types::main();
            println!("\n{}\n", "=".repeat(50));
            ownership::lifetimes::main();
        }
        _ => {
            println!("Invalid choice: {}", choice);
            print_menu();
        }
    }
}
