// Ownership Basics in Rust
// Each value has a single owner, and when the owner goes out of scope, the value is dropped

// ============================================================================
// 1. Basic Ownership - Values Move by Default
// ============================================================================

fn take_ownership(s: String) {
    println!("   Inside function: {}", s);
    // s is dropped here when function ends
}

fn demonstrate_move() {
    let s1 = String::from("hello");
    take_ownership(s1);
    // s1 is no longer valid here - ownership was moved
    // println!("{}", s1); // This would cause a compile error!
}

// ============================================================================
// 2. Copy Types - Simple Types are Copied, Not Moved
// ============================================================================

fn take_integer(x: i32) {
    println!("   Inside function: {}", x);
}

fn demonstrate_copy() {
    let x = 5;
    take_integer(x);
    println!("   After function: {}", x); // x is still valid!
}

// ============================================================================
// 3. Returning Ownership
// ============================================================================

fn create_string() -> String {
    let s = String::from("created");
    s // ownership moves out to caller
}

fn take_and_return(s: String) -> String {
    println!("   Processing: {}", s);
    s // give ownership back
}

// ============================================================================
// 4. Clone - Explicit Deep Copy
// ============================================================================

fn demonstrate_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // explicit copy
    
    println!("   s1: {}", s1); // s1 is still valid
    println!("   s2: {}", s2); // s2 is also valid
}

// ============================================================================
// 5. Scope and Drop
// ============================================================================

fn demonstrate_scope() {
    {
        let s = String::from("inner scope");
        println!("   Inside: {}", s);
    } // s is dropped here
    
    // println!("{}", s); // Error: s no longer exists
    println!("   Outside: s has been dropped");
}

// ============================================================================
// Main Function
// ============================================================================

pub fn main() {
    println!("=== Ownership Basics ===\n");

    println!("1. Move Semantics:");
    demonstrate_move();
    
    println!("\n2. Copy Types (integers, bools, etc.):");
    demonstrate_copy();
    
    println!("\n3. Returning Ownership:");
    let s = create_string();
    println!("   Got string: {}", s);
    let s = take_and_return(s);
    println!("   Got it back: {}", s);
    
    println!("\n4. Using Clone:");
    demonstrate_clone();
    
    println!("\n5. Scope and Drop:");
    demonstrate_scope();
    
    println!("\n=== Key Takeaways ===");
    println!("• Each value has exactly one owner");
    println!("• When owner goes out of scope, value is dropped");
    println!("• Assignment/passing moves ownership (for heap types)");
    println!("• Simple types (i32, bool, etc.) are copied instead");
    println!("• Use .clone() for explicit deep copies");
}
