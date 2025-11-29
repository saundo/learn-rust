// String Types in Rust
// Understanding String vs &str and when to use each

// ============================================================================
// 1. String vs &str Basics
// ============================================================================

fn demonstrate_string_types() {
    // String - owned, heap-allocated, growable
    let s1: String = String::from("hello");
    
    // &str - borrowed, reference to string data
    let s2: &str = "world"; // string literal
    let s3: &str = &s1;     // borrow from String
    
    println!("   String: {}", s1);
    println!("   &str literal: {}", s2);
    println!("   &str from String: {}", s3);
}

// ============================================================================
// 2. String is Mutable, &str is Not
// ============================================================================

fn demonstrate_mutability() {
    let mut s = String::from("hello");
    s.push_str(" world"); // can modify
    s.push('!');
    println!("   Modified String: {}", s);
    
    let slice: &str = "hello";
    // slice.push_str(" world"); // ERROR: can't modify &str
    println!("   Immutable &str: {}", slice);
}

// ============================================================================
// 3. Function Parameters - Prefer &str for Flexibility
// ============================================================================

// Takes &str - works with both String and &str
fn print_message(msg: &str) {
    println!("   Message: {}", msg);
}

// Takes String - caller must give up ownership
fn consume_string(msg: String) {
    println!("   Consumed: {}", msg);
}

fn demonstrate_parameters() {
    let owned = String::from("owned string");
    let borrowed = "borrowed string";
    
    // Both work with &str parameter
    print_message(&owned);
    print_message(borrowed);
    
    // Only String works here (and it's consumed)
    consume_string(owned);
    // println!("{}", owned); // ERROR: owned was moved
}

// ============================================================================
// 4. Converting Between String and &str
// ============================================================================

fn demonstrate_conversions() {
    // &str to String
    let s1: String = "hello".to_string();
    let s2: String = String::from("world");
    let s3: String = "rust".to_owned();
    
    println!("   Created Strings: {}, {}, {}", s1, s2, s3);
    
    // String to &str (automatic with &)
    let s: String = String::from("hello");
    let slice: &str = &s;
    let slice2: &str = s.as_str();
    
    println!("   String as &str: {}, {}", slice, slice2);
}

// ============================================================================
// 5. String Slicing
// ============================================================================

fn demonstrate_slicing() {
    let s = String::from("hello world");
    
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    let full: &str = &s[..];
    
    println!("   Full: {}", s);
    println!("   First part: {}", hello);
    println!("   Second part: {}", world);
    println!("   Full slice: {}", full);
}

// ============================================================================
// 6. When to Use Which
// ============================================================================

// Return String when creating new data
fn create_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Return &str when returning part of input
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    
    s
}

fn demonstrate_return_types() {
    let name = "Alice";
    let greeting = create_greeting(name);
    println!("   Created: {}", greeting);
    
    let sentence = "hello world rust";
    let first = get_first_word(sentence);
    println!("   First word: {}", first);
}

// ============================================================================
// Main Function
// ============================================================================

fn main() {
    println!("=== String Types ===\n");

    println!("1. String vs &str:");
    demonstrate_string_types();
    
    println!("\n2. Mutability:");
    demonstrate_mutability();
    
    println!("\n3. Function Parameters:");
    demonstrate_parameters();
    
    println!("\n4. Conversions:");
    demonstrate_conversions();
    
    println!("\n5. String Slicing:");
    demonstrate_slicing();
    
    println!("\n6. Return Types:");
    demonstrate_return_types();
    
    println!("\n=== Key Takeaways ===");
    println!("• String: owned, heap-allocated, mutable");
    println!("• &str: borrowed reference, immutable");
    println!("• Prefer &str for function parameters");
    println!("• Return String when creating new data");
    println!("• Return &str when returning part of input");
    println!("• Use & to convert String to &str");
}
